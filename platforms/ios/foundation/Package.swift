// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "NativeblocksFoundation",
    platforms: [.iOS(.v15), .macOS(.v13)],
    products: [
        .library(
            name: "NativeblocksFoundation",
            targets: ["NativeblocksFoundation"]
        )
    ],
    dependencies: [
        .package(path: "../compiler"),
        .package(path: "../runtime"),
    ],
    targets: [
        .target(
            name: "NativeblocksFoundation",
            dependencies: [
                .product(name: "NativeblocksRuntime", package: "runtime"),
                .product(name: "NativeblocksCompiler", package: "compiler"),
            ]
        ),
    ]
)
