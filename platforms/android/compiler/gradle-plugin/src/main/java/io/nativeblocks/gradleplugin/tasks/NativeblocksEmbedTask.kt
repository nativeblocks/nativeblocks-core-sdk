package io.nativeblocks.gradleplugin.tasks

import org.gradle.api.DefaultTask
import org.gradle.api.GradleException
import org.gradle.api.file.DirectoryProperty
import org.gradle.api.provider.Property
import org.gradle.api.tasks.Input
import org.gradle.api.tasks.Internal
import org.gradle.api.tasks.OutputDirectory
import org.gradle.api.tasks.TaskAction
import org.gradle.process.ExecOperations
import org.gradle.work.DisableCachingByDefault
import java.io.ByteArrayOutputStream
import javax.inject.Inject

@DisableCachingByDefault(because = "Downloads the latest embeddable frames and localizations")
abstract class NativeblocksEmbedTask : DefaultTask() {

    @get:OutputDirectory
    abstract val outputDir: DirectoryProperty

    @get:Internal
    abstract val workingDir: DirectoryProperty

    @get:Input
    abstract val instance: Property<String>

    @get:Inject
    abstract val execOperations: ExecOperations

    init {
        outputs.upToDateWhen { false }
    }

    @TaskAction
    fun embed() {
        val instanceName = instance.get()
        if (!isValidInstanceName(instanceName)) {
            throw GradleException(
                "Invalid nativeblocks.embeddingInstance '$instanceName'. Use the instance name you pass to NativeblocksManager.initialize."
            )
        }

        val version = runCli("version")
            ?: throw GradleException(
                """
                Nativeblocks CLI not found. Embedding needs it to download your frames.
                Install it: curl -fsSL https://nativeblocks.io/download/cli/installer.sh | bash
                Then make sure `nativeblocks` is on the PATH Gradle sees (restart Android Studio and run ./gradlew --stop).
                """.trimIndent()
            )
        if (version.exitCode != 0) throw unsupportedOr("Nativeblocks CLI failed", version)

        val root = outputDir.get().dir("nativeblocksEmbedded").asFile
        root.listFiles()?.filter { it.name != instanceName }?.forEach { it.deleteRecursively() }
        val out = root.resolve(instanceName).absolutePath
        val sync = runCli("embed", "sync", "--platform", "ANDROID", "--out", out)
            ?: throw GradleException("Failed to run the Nativeblocks CLI")
        if (sync.exitCode != 0) throw unsupportedOr("Nativeblocks embed sync failed", sync)
        logger.lifecycle(sync.output.trim())
    }

    private fun isValidInstanceName(name: String): Boolean =
        name.isNotBlank() && name.all { it.isLetterOrDigit() && it.code < 128 || it == '_' || it == '-' }

    private class CliResult(val exitCode: Int, val output: String)

    private fun unsupportedOr(message: String, result: CliResult): GradleException =
        if (result.output.contains("unknown command")) {
            GradleException("Your Nativeblocks CLI does not support embedding. Update it with `nativeblocks update`.")
        } else {
            GradleException("$message:\n${result.output.trim()}")
        }

    private fun runCli(vararg args: String): CliResult? {
        val output = ByteArrayOutputStream()
        return try {
            val result = execOperations.exec { spec ->
                spec.commandLine(listOf("nativeblocks") + args)
                spec.workingDir = workingDir.get().asFile
                spec.standardOutput = output
                spec.errorOutput = output
                spec.isIgnoreExitValue = true
            }
            CliResult(result.exitValue, output.toString())
        } catch (e: Exception) {
            null
        }
    }
}
