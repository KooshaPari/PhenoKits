import SwiftUI

@main
struct EyetrackerApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}

struct ContentView: View {
    @StateObject var viewModel = EyetrackerViewModel()

    var body: some View {
        ZStack {
            // AR View with face tracking
            EyetrackerView(viewModel: viewModel)
                .ignoresSafeArea()

            // Overlay: gaze cursor and info
            VStack {
                HStack {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Eye Tracker MVP")
                            .font(.headline)
                            .foregroundColor(.white)

                        if let gaze = viewModel.currentGaze {
                            Text("Gaze: (\(Int(gaze.x)), \(Int(gaze.y)))")
                                .font(.caption)
                                .foregroundColor(.gray)

                            Text("Confidence: \(String(format: "%.1f%%", viewModel.gazeConfidence * 100))")
                                .font(.caption)
                                .foregroundColor(.gray)
                        } else {
                            Text("Initializing...")
                                .font(.caption)
                                .foregroundColor(.gray)
                        }
                    }
                    .padding()
                    .background(Color.black.opacity(0.6))
                    .cornerRadius(8)

                    Spacer()
                }
                .padding()

                Spacer()

                // Gaze cursor (if available)
                if let gaze = viewModel.currentGaze {
                    Circle()
                        .stroke(Color.red, lineWidth: 2)
                        .frame(width: 20, height: 20)
                        .position(x: CGFloat(gaze.x), y: CGFloat(gaze.y))
                        .opacity(viewModel.gazeConfidence > 0.5 ? 0.8 : 0.3)
                }
            }
        }
    }
}

#Preview {
    ContentView()
}
