package io.nativeblocks.gradleplugin.tasks

import io.nativeblocks.gradleplugin.NativeblocksCli
import org.gradle.api.DefaultTask
import org.gradle.api.file.DirectoryProperty
import org.gradle.api.tasks.Internal
import org.gradle.api.tasks.TaskAction
import org.gradle.process.ExecOperations
import org.gradle.work.DisableCachingByDefault
import javax.inject.Inject

@DisableCachingByDefault(because = "Downloads the latest embeddable frames and localizations")
abstract class NativeblocksEmbedTask : DefaultTask() {

    @get:Internal
    abstract val appDir: DirectoryProperty

    @get:Internal
    abstract val rootDir: DirectoryProperty

    @get:Inject
    abstract val execOperations: ExecOperations

    init {
        outputs.upToDateWhen { false }
    }

    @TaskAction
    fun embed() {
        val cli = NativeblocksCli(execOperations, rootDir.get().asFile)
        val output = cli.run("embed", "sync", "--platform", "ANDROID", "--target", appDir.get().asFile.absolutePath)
        logger.lifecycle(output)
    }
}
