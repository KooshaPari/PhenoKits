// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "EyetrackerSwift",
    platforms: [
        .iOS(.v16)
    ],
    products: [
        .library(name: "EyetrackerSwift", targets: ["EyetrackerSwift"]),
    ],
    targets: [
        .target(
            name: "EyetrackerSwift",
            dependencies: ["eyetracker_ffi"],
            path: "Sources"
        ),
        .binaryTarget(
            name: "eyetracker_ffi",
            path: "Frameworks/eyetracker_ffi.xcframework"
        ),
    ]
)
