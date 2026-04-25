import SwiftUI
import AVFoundation
import Vision

@main
struct EyetrackerApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
                .frame(minWidth: 800, minHeight: 600)
        }
    }
}

struct ContentView: View {
    @StateObject var viewModel = MacOSEyetrackerViewModel()

    var body: some View {
        ZStack {
            // Webcam preview
            CameraPreviewRepresentable(viewModel: viewModel)
                .ignoresSafeArea()

            VStack(alignment: .leading, spacing: 16) {
                HStack {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("macOS Eye Tracker")
                            .font(.headline)

                        if let gaze = viewModel.currentGaze {
                            Text("Gaze: (\(Int(gaze.x)), \(Int(gaze.y)))")
                                .font(.caption)
                                .foregroundColor(.secondary)

                            Text("Confidence: \(String(format: "%.1f%%", gaze.confidence * 100))")
                                .font(.caption)
                                .foregroundColor(.secondary)
                        }
                    }
                    .padding()
                    .background(Color.black.opacity(0.3))
                    .cornerRadius(8)

                    Spacer()
                }
                .padding()

                Spacer()
            }

            // Gaze cursor
            if let gaze = viewModel.currentGaze {
                Circle()
                    .stroke(Color.red, lineWidth: 2)
                    .frame(width: 20, height: 20)
                    .position(x: CGFloat(gaze.x), y: CGFloat(gaze.y))
                    .opacity(gaze.confidence > 0.5 ? 0.8 : 0.3)
            }
        }
        .onAppear {
            viewModel.startCapture()
        }
        .onDisappear {
            viewModel.stopCapture()
        }
    }
}

class MacOSEyetrackerViewModel: NSObject, ObservableObject {
    @Published var currentGaze: (x: Float, y: Float, confidence: Float)?

    private var captureSession: AVCaptureSession?
    private var estimator: GazeEstimatorFFI?
    private var videoOutput: AVCaptureVideoDataOutput?
    private let outputQueue = DispatchQueue(label: "eyetracker.video.output")

    override init() {
        super.init()
        setupEstimator()
    }

    func setupEstimator() {
        let camera = CameraIntrinsicsFFI(
            focal_length_x: 480.0,
            focal_length_y: 480.0,
            principal_point_x: 320.0,
            principal_point_y: 240.0,
            width: 640,
            height: 480
        )

        let screen = ScreenInfoFFI(
            width_px: 1680,
            height_px: 1050,
            dpi: 96.0
        )

        estimator = GazeEstimatorFFI(camera: camera, screen: screen)
    }

    func startCapture() {
        let session = AVCaptureSession()
        session.sessionPreset = .vga640x480

        guard let device = AVCaptureDevice.default(for: .video) else {
            print("No camera available")
            return
        }

        do {
            let input = try AVCaptureDeviceInput(device: device)
            session.addInput(input)

            let output = AVCaptureVideoDataOutput()
            output.setSampleBufferDelegate(self, queue: outputQueue)
            session.addOutput(output)

            self.captureSession = session
            self.videoOutput = output
            session.startRunning()
        } catch {
            print("Failed to setup camera: \(error)")
        }
    }

    func stopCapture() {
        captureSession?.stopRunning()
    }
}

extension MacOSEyetrackerViewModel: AVCaptureVideoDataOutputSampleBufferDelegate {
    func captureOutput(
        _ output: AVCaptureOutput,
        didDrop sampleBuffer: CMSampleBuffer,
        from connection: AVCaptureConnection
    ) {
    }

    func captureOutput(
        _ output: AVCaptureOutput,
        didOutput sampleBuffer: CMSampleBuffer,
        from connection: AVCaptureConnection
    ) {
        // Process frame with Vision face detection
        guard let pixelBuffer = CMSampleBufferGetImageBuffer(sampleBuffer) else {
            return
        }

        let request = VNDetectFaceLandmarksRequest { [weak self] request, error in
            guard let observations = request.results as? [VNFaceObservation] else {
                return
            }

            for face in observations {
                self?.processFaceLandmarks(face, timestamp: CMSampleBufferGetPresentationTimeStamp(sampleBuffer).value)
            }
        }

        let handler = VNImageRequestHandler(cvPixelBuffer: pixelBuffer, options: [:])
        try? handler.perform([request])
    }

    private func processFaceLandmarks(_ face: VNFaceObservation, timestamp: Int64) {
        guard let landmarks = face.landmarks else { return }

        let leftEyePoints = landmarks.leftEye?.pointsInImage(imageSize: CGSize(width: 640, height: 480)) ?? []
        let rightEyePoints = landmarks.rightEye?.pointsInImage(imageSize: CGSize(width: 640, height: 480)) ?? []
        let nosePoints = landmarks.nose?.pointsInImage(imageSize: CGSize(width: 640, height: 480)) ?? []

        let leftEyeCenter = leftEyePoints.isEmpty ? CGPoint.zero : centerPoint(leftEyePoints)
        let rightEyeCenter = rightEyePoints.isEmpty ? CGPoint.zero : centerPoint(rightEyePoints)
        let noseCenter = nosePoints.isEmpty ? CGPoint.zero : centerPoint(nosePoints)

        let faceLandmarks = FaceLandmarksFFI(
            left_eye: Point2DFFI(x: Float(leftEyeCenter.x), y: Float(leftEyeCenter.y)),
            right_eye: Point2DFFI(x: Float(rightEyeCenter.x), y: Float(rightEyeCenter.y)),
            nose: Point2DFFI(x: Float(noseCenter.x), y: Float(noseCenter.y)),
            left_cheek: Point2DFFI(x: 0, y: 0),
            right_cheek: Point2DFFI(x: 0, y: 0)
        )

        if let gaze = estimator?.estimate_gaze(landmarks: faceLandmarks, timestamp_ms: timestamp / 1_000_000) {
            DispatchQueue.main.async {
                self.currentGaze = (x: gaze.x, y: gaze.y, confidence: gaze.confidence)
            }
        }
    }

    private func centerPoint(_ points: [CGPoint]) -> CGPoint {
        guard !points.isEmpty else { return .zero }
        let sumX = points.reduce(0) { $0 + $1.x }
        let sumY = points.reduce(0) { $0 + $1.y }
        return CGPoint(x: sumX / CGFloat(points.count), y: sumY / CGFloat(points.count))
    }
}

struct CameraPreviewRepresentable: NSViewRepresentable {
    var viewModel: MacOSEyetrackerViewModel

    func makeNSView(context: Context) -> NSView {
        let view = NSView()
        if let session = viewModel.captureSession {
            let previewLayer = AVCaptureVideoPreviewLayer(session: session)
            previewLayer.videoGravity = .resizeAspectFill
            view.layer?.addSublayer(previewLayer)
        }
        return view
    }

    func updateNSView(_ nsView: NSView, context: Context) {
    }
}

#Preview {
    ContentView()
}
