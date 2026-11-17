package io.nativeblocks.gradleplugin.tasks

import io.nativeblocks.gradleplugin.NativeblocksCli
import org.gradle.api.DefaultTask
import org.gradle.api.GradleException
import org.gradle.api.file.DirectoryProperty
import org.gradle.api.tasks.Internal
import org.gradle.api.tasks.TaskAction
import org.gradle.process.ExecOperations
import org.gradle.work.DisableCachingByDefault
import java.io.File
import javax.inject.Inject

@DisableCachingByDefault(because = "Uploads integrations to Nativeblocks")
abstract class NativeblocksSyncTask : DefaultTask() {

    @get:Internal
    abstract val integrationsDir: DirectoryProperty

    @get:Internal
    abstract val rootDir: DirectoryProperty

    @get:Inject
    abstract val execOperations: ExecOperations

    init {
        outputs.upToDateWhen { false }
    }

    @TaskAction
    fun sync() {
        val root = rootDir.get().asFile
        if (File(root, "nativeblocks.json").exists()) {
            logger.warn(
                "nativeblocks.json is not used anymore, sync uses your Nativeblocks CLI login. " +
                    "Delete it, it contains an auth token."
            )
        }

        val source = integrationsDir.get().asFile
        val integrations = source.walk()
            .filter { File(it, "integration.json").isFile }
            .sortedBy { it.path }
            .toList()
        if (integrations.isEmpty()) {
            throw GradleException(
                "There is no integration under ${source.path}, Please make sure implementation is correct and there is no build issue"
            )
        }

        val cli = NativeblocksCli(execOperations, root)
        integrations.forEach { integration ->
            cli.run("integration", "sync", "--directory", integration.absolutePath)
            logger.lifecycle("Integration ${integration.name} has been uploaded successfully")
        }
    }
}
