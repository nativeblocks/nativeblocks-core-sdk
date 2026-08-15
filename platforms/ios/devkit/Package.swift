// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "NativeblocksDevkit",
    platforms: [.iOS(.v15)],
    products: [
        .library(
            name: "NativeblocksDevkit",
            targets: ["NativeblocksDevkit"]
        )
    ],
    dependencies: [
        .package(path: "../runtime")
    ],

    targets: [
        .target(
            name: "NativeblocksDevkit",
            dependencies: [.product(name: "NativeblocksRuntime", package: "runtime")],
            swiftSettings: [
                .unsafeFlags([
                    "-enable-library-evolution"
                ])
            ]
        )
    ]
)
