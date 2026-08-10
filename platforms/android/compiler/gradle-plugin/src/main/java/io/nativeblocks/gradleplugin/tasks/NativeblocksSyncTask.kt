package io.nativeblocks.gradleplugin.tasks

import io.nativeblocks.gradleplugin.GlobalState
import io.nativeblocks.gradleplugin.NativeblocksConfig
import io.nativeblocks.gradleplugin.integration.IntegrationRepository
import kotlinx.coroutines.runBlocking
import org.gradle.api.DefaultTask
import org.gradle.api.GradleException
import org.gradle.api.provider.Property
import org.gradle.api.tasks.Input
import org.gradle.api.tasks.TaskAction

abstract class NativeblocksSyncTask : DefaultTask() {

    @get:Input
    abstract val config: Property<NativeblocksConfig>

    @get:Input
    abstract val flavor: Property<String>

    @get:Input
    abstract val basePackageName: Property<String>

    @get:Input
    abstract val moduleName: Property<String>

    @TaskAction
    fun sync() {
        val configValue = config.get()

        GlobalState.endpoint = configValue.endpoint
        GlobalState.authToken = configValue.authToken
        GlobalState.organizationId = configValue.organizationId
        GlobalState.basePackageName = basePackageName.get()
        GlobalState.moduleName = moduleName.get()

        if (configValue.endpoint.isEmpty() ||
            configValue.authToken.isEmpty() ||
            configValue.organizationId.isEmpty()
        ) {
            throw GradleException("Please make sure endpoint, authToken and organizationId are provided in nativeblocks.json")
        }

        val integrationRepository = IntegrationRepository()
        runBlocking {
            integrationRepository.syncIntegration(project, flavor.get())
        }
    }
}
