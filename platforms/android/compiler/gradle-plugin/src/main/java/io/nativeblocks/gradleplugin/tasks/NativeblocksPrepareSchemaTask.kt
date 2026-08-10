package io.nativeblocks.gradleplugin.tasks

import io.nativeblocks.gradleplugin.GlobalState
import io.nativeblocks.gradleplugin.integration.IntegrationRepository
import kotlinx.coroutines.runBlocking
import org.gradle.api.DefaultTask
import org.gradle.api.provider.Property
import org.gradle.api.tasks.Input
import org.gradle.api.tasks.TaskAction

abstract class NativeblocksPrepareSchemaTask : DefaultTask() {

    @get:Input
    abstract val flavor: Property<String>

    @get:Input
    abstract val basePackageName: Property<String>

    @get:Input
    abstract val moduleName: Property<String>

    @TaskAction
    fun prepareSchema() {
        GlobalState.basePackageName = basePackageName.get()
        GlobalState.moduleName = moduleName.get()

        val integrationRepository = IntegrationRepository()
        runBlocking {
            integrationRepository.prepareSchema(project, flavor.get())
        }
    }
}
