import SwiftUI
import ARKit
import Vision

/// Main AR-based eye-tracking view with real-time gaze rendering
struct EyetrackerView: UIViewControllerRepresentable {
    @StateObject var viewModel: EyetrackerViewModel

    func makeUIViewController(context: Context) -> ARViewControllerWrapper {
        let controller = ARViewControllerWrapper()
        controller.viewModel = viewModel
        return controller
    }

    func updateUIViewController(_ uiViewController: ARViewControllerWrapper, context: Context) {
    }
}

class ARViewControllerWrapper: UIViewController, ARSessionDelegate {
    var viewModel: EyetrackerViewModel?
    var arSession: ARSession!
    var faceGeometry: ARFaceGeometry?

    override func viewDidLoad() {
        super.viewDidLoad()
        setupARSession()
    }

    func setupARSession() {
        arSession = ARSession()
        arSession.delegate = self

        let configuration = ARFaceTrackingConfiguration()
        configuration.isLightEstimationEnabled = true

        if ARFaceTrackingConfiguration.isSupported {
            arSession.run(configuration)
        }
    }

    // MARK: ARSessionDelegate

    func session(_ session: ARSession, didUpdate frame: ARFrame) {
        // Extract face anchor and landmarks
        guard let faceAnchor = frame.anchors.compactMap({ $0 as? ARFaceAnchor }).first else {
            return
        }

        let leftEye = faceAnchor.leftEyeTransform.translation
        let rightEye = faceAnchor.rightEyeTransform.translation
        let lookAt = faceAnchor.lookAtPoint

        // Send to Rust estimator via FFI
        DispatchQueue.main.async {
            self.viewModel?.processARFrame(
                leftEye: (Float(leftEye.x), Float(leftEye.y)),
                rightEye: (Float(rightEye.x), Float(rightEye.y)),
                lookAt: (Float(lookAt.x), Float(lookAt.y)),
                timestamp: Int64(Date().timeIntervalSince1970 * 1000)
            )
        }
    }
}

@MainActor
class EyetrackerViewModel: NSObject, ObservableObject {
    @Published var currentGaze: (x: Float, y: Float)?
    @Published var gazeConfidence: Float = 0.0
    @Published var isCalibrating = false

    private var estimator: GazeEstimatorFFI?
    private var detectionRequest: VNDetectFaceLandmarksRequest?

    override init() {
        super.init()
        setupEstimator()
    }

    func setupEstimator() {
        // Initialize with dummy camera intrinsics (would be calibrated in real app)
        let camera = CameraIntrinsicsFFI(
            focal_length_x: 500.0,
            focal_length_y: 500.0,
            principal_point_x: 320.0,
            principal_point_y: 240.0,
            width: 640,
            height: 480
        )

        let screen = ScreenInfoFFI(
            width_px: 1080,
            height_px: 1920,
            dpi: 326.0
        )

        estimator = GazeEstimatorFFI(camera: camera, screen: screen)
    }

    func processARFrame(leftEye: (Float, Float), rightEye: (Float, Float), lookAt: (Float, Float), timestamp: Int64) {
        let landmarks = FaceLandmarksFFI(
            left_eye: Point2DFFI(x: Float(leftEye.0), y: Float(leftEye.1)),
            right_eye: Point2DFFI(x: Float(rightEye.0), y: Float(rightEye.1)),
            nose: Point2DFFI(x: 0.0, y: 0.0),
            left_cheek: Point2DFFI(x: 0.0, y: 0.0),
            right_cheek: Point2DFFI(x: 0.0, y: 0.0)
        )

        if let gaze = estimator?.estimate_gaze(landmarks: landmarks, timestamp_ms: timestamp) {
            self.currentGaze = (x: gaze.x, y: gaze.y)
            self.gazeConfidence = gaze.confidence
        }
    }
}

#Preview {
    EyetrackerView(viewModel: EyetrackerViewModel())
}
