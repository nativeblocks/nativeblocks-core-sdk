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
        .plugin(name: "EmbedContent", targets: ["EmbedContent"]),
        .plugin(name: "Embed", targets: ["Embed"]),
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
                    description: "Upload blocks, actions and modifiers to Nativeblocks with the Nativeblocks CLI."),
                permissions: [
                    .allowNetworkConnections(
                        scope: PluginNetworkPermissionScope.all(),
                        reason: "This command uploads the integrations with the Nativeblocks CLI.")
                ]
            ),
            dependencies: [
                .target(name: "NativeblocksTool")
            ]
        ),

        .plugin(
            name: "EmbedContent",
            capability: .buildTool()
        ),

        .plugin(
            name: "Embed",
            capability: .command(
                intent: .custom(
                    verb: "Embed",
                    description: "Download the embeddable frames and localizations into this app with the Nativeblocks CLI."),
                permissions: [
                    .writeToPackageDirectory(
                        reason: "This command writes the embedded content into the project's build folder."),
                    .allowNetworkConnections(
                        scope: PluginNetworkPermissionScope.all(),
                        reason: "This command downloads the embedded content with the Nativeblocks CLI."),
                ]
            )
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
