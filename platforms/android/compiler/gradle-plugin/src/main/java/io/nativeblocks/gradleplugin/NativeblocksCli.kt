package io.nativeblocks.gradleplugin

import org.gradle.api.GradleException
import org.gradle.process.ExecOperations
import java.io.ByteArrayOutputStream
import java.io.File

internal class NativeblocksCli(
    private val execOperations: ExecOperations,
    private val workingDir: File,
) {
    private val executable: File = find() ?: throw GradleException(
        """
        Nativeblocks CLI not found. Nativeblocks tasks need it to reach your project.
        Install it: curl -fsSL https://nativeblocks.io/download/cli/installer.sh | bash
        Then run `nativeblocks auth login` and `nativeblocks organization set`.
        """.trimIndent()
    )

    fun run(vararg arguments: String): String {
        val output = ByteArrayOutputStream()
        val result = execOperations.exec {
            it.executable = executable.absolutePath
            it.args(arguments.toList())
            it.workingDir = workingDir
            it.standardOutput = output
            it.errorOutput = output
            it.isIgnoreExitValue = true
        }
        if (result.exitValue != 0) {
            throw GradleException("Nativeblocks CLI failed: ${errorMessage(output.toString())}")
        }
        return output.toString().trim()
    }

    private fun errorMessage(output: String): String {
        val error = output.lineSequence().firstOrNull { it.startsWith("Error:") }
        return error?.removePrefix("Error:")?.trim() ?: output.substringBefore("Usage:").trim()
    }

    private fun find(): File? {
        val names = listOf("nativeblocks", "nativeblocks.exe")
        val pathDirs = System.getenv("PATH").orEmpty()
            .split(File.pathSeparator)
            .filter { it.isNotBlank() }
            .map(::File)
        val installDir = File(System.getProperty("user.home"), ".nativeblocks/bin")
        return (pathDirs + installDir)
            .flatMap { dir -> names.map { File(dir, it) } }
            .firstOrNull { it.isFile && it.canExecute() }
    }
}
