// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "NativeblocksRuntime",
    platforms: [.iOS(.v15)],
    products: [
        .library(
            name: "NativeblocksRuntime",
            targets: ["NativeblocksRuntime"]
        )
    ],
    targets: [
        .binaryTarget(
            name: "NativeblocksRuntimeCFFI",
            path: "Frameworks/NativeblocksRuntimeCFFI.xcframework"
        ),
        .target(
            name: "NativeblocksRuntimeFFI",
            dependencies: ["NativeblocksRuntimeCFFI"],
            swiftSettings: [
                .unsafeFlags([
                    "-enable-library-evolution"
                ])
            ]
        ),
        .target(
            name: "NativeblocksRuntime",
            dependencies: ["NativeblocksRuntimeFFI"],
            swiftSettings: [
                .unsafeFlags([
                    "-enable-library-evolution"
                ])
            ]
        )
    ]
)
