// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import CompilerPluginSupport
import PackageDescription

let package = Package(
    name: "NativeblocksCompiler",
    platforms: [.macOS(.v13), .iOS(.v15)],
    products: [
        .library(name: "NativeblocksCompiler", targets: ["NativeblocksCompiler"]),
        .plugin(name: "GenerateProvider", targets: ["GenerateProvider"]),
        .plugin(name: "Sync", targets: ["Sync"]),
        .plugin(name: "PrepareSchema", targets: ["PrepareSchema"]),
        .executable(name: "NativeblocksTool", targets: ["NativeblocksTool"]),
    ],
    dependencies: [
        .package(url: "https://github.com/apple/swift-syntax.git", from: "509.0.0")
    ],
    targets: [
        .macro(
            name: "NativeblocksCompilerMacros",
            dependencies: [
                .product(name: "SwiftSyntaxMacros", package: "swift-syntax"),
                .product(name: "SwiftCompilerPlugin", package: "swift-syntax"),
                "_NativeblocksCompilerCommon",
            ]
        ),

        .plugin(
            name: "GenerateProvider",
            capability: .command(
                intent: .custom(
                    verb: "GenerateProvider",
                    description: "Generate `{Target}ActionProvider.swift` and `{Target}BlockProvider.swift` from Block and Action macros."),
                permissions: [
                    .writeToPackageDirectory(
                        reason: "This command write the new Provider to the source root.")
                ]
            ),
            dependencies: [
                .target(name: "NativeblocksTool")
            ]
        ),

        .plugin(
            name: "Sync",
            capability: .command(
                intent: .custom(
                    verb: "Sync",
                    description: "Synchronize JSON blocks and actions with the Nativeblocks Studio."),
                permissions: [
                    .writeToPackageDirectory(
                        reason: "This command writes the new JSON files to the source root."),
                    .allowNetworkConnections(
                        scope: PluginNetworkPermissionScope.all(),
                        reason: "This command synchronizes JSONs with the Nativeblocks server."),
                ]
            ),
            dependencies: [
                .target(name: "NativeblocksTool")
            ]
        ),

        .plugin(
            name: "PrepareSchema",
            capability: .command(
                intent: .custom(
                    verb: "PrepareSchema",
                    description: "Generate local JSON schemas for Blocks and Actions in the `.nativeblocks` directory."
                ),
                permissions: [
                    .writeToPackageDirectory(
                        reason: "This command write the new json blocks to the source root.")
                ]
            ),
            dependencies: [
                .target(name: "NativeblocksTool")
            ]
        ),

        .executableTarget(
            name: "NativeblocksTool",
            dependencies: [
                .product(name: "SwiftSyntax", package: "swift-syntax"),
                .product(name: "SwiftParser", package: "swift-syntax"),
                .product(name: "SwiftSyntaxBuilder", package: "swift-syntax"),
                "_NativeblocksCompilerCommon",
            ]
        ),
        .target(
            name: "NativeblocksCompiler",
            dependencies: ["NativeblocksCompilerMacros"]
        ),

        .target(
            name: "_NativeblocksCompilerCommon",
            dependencies: [
                .product(name: "SwiftSyntax", package: "swift-syntax"),
                .product(name: "SwiftParser", package: "swift-syntax"),
                .product(name: "SwiftSyntaxBuilder", package: "swift-syntax"),
            ]
        ),
    ]
)
