import Foundation
import PackagePlugin

@main
struct EmbedContent: BuildToolPlugin {
    func createBuildCommands(context: PluginContext, target: Target) throws -> [Command] {
        return []
    }
}

#if canImport(XcodeProjectPlugin)
    import XcodeProjectPlugin

    extension EmbedContent: XcodeBuildToolPlugin {
        func createBuildCommands(context: XcodePluginContext, target: XcodeTarget) throws -> [Command] {
            guard case .application = target.product?.kind else {
                return []
            }
            return embedCommands(
                projectDirectory: URL(fileURLWithPath: context.xcodeProject.directory.string),
                workDirectory: URL(fileURLWithPath: context.pluginWorkDirectory.string))
        }
    }
#endif

private let embeddedBuildDir = "build/nativeblocks/embedded"
private let embeddedDir = "nativeblocksEmbedded"

private func embedCommands(projectDirectory: URL, workDirectory: URL) -> [Command] {
    let fileManager = FileManager.default
    var source = projectDirectory
        .appendingPathComponent(embeddedBuildDir)
        .appendingPathComponent(embeddedDir)
    var inputs: [Path] = []
    if let files = fileManager.enumerator(at: source, includingPropertiesForKeys: [.isRegularFileKey]) {
        for case let file as URL in files
        where (try? file.resourceValues(forKeys: [.isRegularFileKey]).isRegularFile) == true {
            inputs.append(Path(file.path))
        }
    }
    if inputs.isEmpty {
        source = workDirectory.appendingPathComponent("empty")
        try? fileManager.createDirectory(at: source, withIntermediateDirectories: true)
    }
    let output = workDirectory.appendingPathComponent(embeddedDir)
    return [
        .buildCommand(
            displayName: "Embed Nativeblocks content",
            executable: Path("/usr/bin/rsync"),
            arguments: ["-a", "--delete", source.path + "/", output.path],
            inputFiles: inputs,
            outputFiles: [Path(output.path)]
        )
    ]
}
