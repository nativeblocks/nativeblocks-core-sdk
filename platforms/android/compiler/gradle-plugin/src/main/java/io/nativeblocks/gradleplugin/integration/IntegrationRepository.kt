package io.nativeblocks.gradleplugin.integration

import com.apollographql.apollo3.ApolloClient
import com.apollographql.apollo3.api.Optional
import com.apollographql.apollo3.network.okHttpClient
import io.nativeblocks.gradleplugin.GlobalState
import io.nativeblocks.gradleplugin.network.AuthorizationInterceptor
import io.nativeblocks.gradleplugin.network.NetworkRequestExecutor
import io.nativeblocks.gradleplugin.network.doOnError
import io.nativeblocks.gradleplugin.network.doOnSuccess
import io.nativeblocks.gradleplugin.network.execute
import io.nativeblocks.network.SyncIntegrationDataMutation
import io.nativeblocks.network.SyncIntegrationEventsMutation
import io.nativeblocks.network.SyncIntegrationMutation
import io.nativeblocks.network.SyncIntegrationPropertiesMutation
import io.nativeblocks.network.SyncIntegrationSlotsMutation
import io.nativeblocks.network.type.IntegrationDataInput
import io.nativeblocks.network.type.IntegrationEventInput
import io.nativeblocks.network.type.IntegrationPropertyInput
import io.nativeblocks.network.type.IntegrationSlotInput
import io.nativeblocks.network.type.SyncIntegrationDataInput
import io.nativeblocks.network.type.SyncIntegrationEventsInput
import io.nativeblocks.network.type.SyncIntegrationInput
import io.nativeblocks.network.type.SyncIntegrationPropertiesInput
import io.nativeblocks.network.type.SyncIntegrationSlotsInput
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.booleanOrNull
import kotlinx.serialization.json.intOrNull
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import okhttp3.OkHttpClient
import org.gradle.api.GradleException
import org.gradle.api.Project
import java.io.File
import java.nio.file.Files
import java.nio.file.StandardCopyOption
import java.util.concurrent.TimeUnit

class IntegrationRepository {

    private val authorizationInterceptor = AuthorizationInterceptor(GlobalState.authToken.orEmpty())

    private val okHttpClient = OkHttpClient.Builder()
        .addInterceptor(authorizationInterceptor)
        .readTimeout(120, TimeUnit.SECONDS)
        .connectTimeout(120, TimeUnit.SECONDS)
        .build()

    private val apolloClient = ApolloClient.Builder()
        .httpServerUrl(GlobalState.endpoint.orEmpty())
        .okHttpClient(okHttpClient)
        .build()

    private val networkRequestExecutor = NetworkRequestExecutor()

    private fun copyDirectoryToRoot(sourceDirPath: String, projectRootDir: File) {
        val sourceDir = File(sourceDirPath)
        if (!sourceDir.exists() || !sourceDir.isDirectory) {
            println("Source directory does not exist or is not a directory: $sourceDirPath")
            return
        }
        sourceDir.walkTopDown().forEach { file ->
            val relativePath = file.relativeTo(sourceDir).path
            val targetFile = File(projectRootDir.path.toString() + "/.nativeblocks/integrations", relativePath)
            if (file.isDirectory) {
                targetFile.mkdirs()
            } else {
                targetFile.parentFile.mkdirs()
                Files.copy(file.toPath(), targetFile.toPath(), StandardCopyOption.REPLACE_EXISTING)
                println("Copied: ${file.path} to ${targetFile.path}")
            }
        }
    }

    private fun getSubdirectories(path: String): MutableList<File> {
        val baseDir = File(path)
        val result = mutableListOf<File>()
        if (baseDir.exists() && baseDir.isDirectory) {
            listOf("action", "block").forEach { dirName ->
                val subDir = File(baseDir, dirName)
                if (subDir.exists() && subDir.isDirectory) {
                    subDir.walk()
                        .filter { it.isDirectory && it != subDir }
                        .forEach { result.add(it) }
                }
            }
        }
        return result
    }

    private fun mergeFilePaths(fileList: List<File>): Map<String, IntegrationMeta> {
        val result = mutableMapOf<String, IntegrationMeta>()
        fileList.forEach { f ->
            val parent = f.parentFile.name
            val filename = f.name

            val meta = when (filename) {
                "integration.json" -> IntegrationMeta(Json.parseToJsonElement(f.readText()), null, null, null, null)
                "properties.json" -> IntegrationMeta(null, Json.parseToJsonElement(f.readText()), null, null, null)
                "events.json" -> IntegrationMeta(null, null, Json.parseToJsonElement(f.readText()), null, null)
                "data.json" -> IntegrationMeta(null, null, null, Json.parseToJsonElement(f.readText()), null)
                "slots.json" -> IntegrationMeta(null, null, null, null, Json.parseToJsonElement(f.readText()))
                else -> IntegrationMeta(null, null, null, null, null)
            }

            val existingMeta = result.getOrPut(parent) {
                IntegrationMeta(null, null, null, null, null)
            }

            result[parent] = existingMeta.copy(
                integrationJson = meta.integrationJson ?: existingMeta.integrationJson,
                propertiesJson = meta.propertiesJson ?: existingMeta.propertiesJson,
                eventsJson = meta.eventsJson ?: existingMeta.eventsJson,
                dataJson = meta.dataJson ?: existingMeta.dataJson,
                slotsJson = meta.slotsJson ?: existingMeta.slotsJson
            )
        }
        return result
    }

    private fun rootDirectoryPath(flavor: String): String {
        return "generated/ksp/$flavor/resources/${GlobalState.basePackageName?.replace(".", "/")}/integration/consumer"
    }

    suspend fun prepareSchema(project: Project, flavor: String) {
        val directoryPath = rootDirectoryPath(flavor)
        val jsonDirectory = project.layout.buildDirectory.files(directoryPath)

        val files = jsonDirectory.first().listFiles()
        if (files.isNullOrEmpty()) {
            throw GradleException("There is no integration under $directoryPath, Please make sure implementation is correct and there is no build issue")
        }
        withContext(Dispatchers.IO) {
            copyDirectoryToRoot(jsonDirectory.asPath, project.rootDir)
        }
    }

    suspend fun syncIntegration(project: Project, flavor: String) {
        val directoryPath = rootDirectoryPath(flavor)
        val jsonDirectory = project.layout.buildDirectory.files(directoryPath)
        mergeFilePaths(
            getSubdirectories(jsonDirectory.asPath)
                .flatMap { it.listFiles().orEmpty().toList() })
            .forEach { entity ->
                entity.value.integrationJson?.let { integrationJson ->
                    val request = apolloClient.mutation(
                        SyncIntegrationMutation(
                            input = SyncIntegrationInput(
                                name = integrationJson.jsonObject["name"]?.jsonPrimitive?.content.orEmpty(),
                                keyType = integrationJson.jsonObject["keyType"]?.jsonPrimitive?.content.orEmpty(),
                                imageIcon = Optional.presentIfNotNull(integrationJson.jsonObject["imageIcon"]?.jsonPrimitive?.content.orEmpty()),
                                price = integrationJson.jsonObject["price"]?.jsonPrimitive?.intOrNull ?: 0,
                                description = Optional.presentIfNotNull(integrationJson.jsonObject["description"]?.jsonPrimitive?.content.orEmpty()),
                                documentation = Optional.presentIfNotNull(integrationJson.jsonObject["documentation"]?.jsonPrimitive?.content.orEmpty()),
                                platformSupport = integrationJson.jsonObject["platformSupport"]?.jsonPrimitive?.content.orEmpty(),
                                public = integrationJson.jsonObject["public"]?.jsonPrimitive?.booleanOrNull ?: false,
                                kind = integrationJson.jsonObject["kind"]?.jsonPrimitive?.content.orEmpty(),
                                organizationId = GlobalState.organizationId.orEmpty(),
                                version = integrationJson.jsonObject["version"]?.jsonPrimitive?.intOrNull ?: 1,
                                versionName = integrationJson.jsonObject["versionName"]?.jsonPrimitive?.content.orEmpty(),
                                deprecated = Optional.presentIfNotNull(integrationJson.jsonObject["deprecated"]?.jsonPrimitive?.booleanOrNull ?: false),
                                deprecatedReason = Optional.presentIfNotNull(integrationJson.jsonObject["deprecatedReason"]?.jsonPrimitive?.content.orEmpty()),
                            )
                        )
                    )
                    val req = networkRequestExecutor.requestExecutor(request)
                    req.doOnSuccess { res ->
                        val id = res?.syncIntegration?.id.orEmpty()
                        val name = res?.syncIntegration?.name.orEmpty()
                        entity.value.propertiesJson?.let { syncProperties(id, name, it) }
                        entity.value.eventsJson?.let { syncEvents(id, name, it) }
                        entity.value.dataJson?.let { syncData(id, name, it) }
                        entity.value.slotsJson?.let { syncSlots(id, name, it) }
                        println("Integration ${res?.syncIntegration?.keyType} has been uploaded successfully")
                    }
                    req.doOnError { error ->
                        throw GradleException("The ${integrationJson.jsonObject["name"]?.jsonPrimitive?.content.orEmpty()} failed to upload because: ${error.message}")
                    }
                }
            }
    }

    private fun syncProperties(
        integrationId: String,
        integrationName: String,
        propertiesJson: JsonElement,
    ) {
        val request = apolloClient.mutation(
            SyncIntegrationPropertiesMutation(
                input = SyncIntegrationPropertiesInput(
                    integrationId = integrationId,
                    organizationId = GlobalState.organizationId.orEmpty(),
                    properties = propertiesJson.jsonArray.map { property ->
                        IntegrationPropertyInput(
                            key = property.jsonObject["key"]?.jsonPrimitive?.content.orEmpty(),
                            value = Optional.presentIfNotNull(property.jsonObject["value"]?.jsonPrimitive?.content.orEmpty()),
                            type = property.jsonObject["type"]?.jsonPrimitive?.content.orEmpty(),
                            description = property.jsonObject["description"]?.jsonPrimitive?.content.orEmpty(),
                            valuePicker = property.jsonObject["valuePicker"]?.jsonPrimitive?.content.orEmpty(),
                            valuePickerGroup = property.jsonObject["valuePickerGroup"]?.jsonPrimitive?.content.orEmpty(),
                            valuePickerOptions = property.jsonObject["valuePickerOptions"]?.jsonPrimitive?.content.orEmpty(),
                            deprecated = Optional.presentIfNotNull(
                                property.jsonObject["deprecated"]?.jsonPrimitive?.booleanOrNull
                                    ?: false
                            ),
                            deprecatedReason = Optional.presentIfNotNull(property.jsonObject["deprecatedReason"]?.jsonPrimitive?.content.orEmpty()),
                        )
                    }
                )
            )
        )
        runBlocking {
            networkRequestExecutor.requestExecutor(request).execute({ _ ->
            }, { error ->
                throw GradleException("The $integrationName failed to upload because: ${error.message}")
            })
        }
    }

    private fun syncEvents(
        integrationId: String,
        integrationName: String,
        eventsJson: JsonElement,
    ) {
        val request = apolloClient.mutation(
            SyncIntegrationEventsMutation(
                input = SyncIntegrationEventsInput(
                    integrationId = integrationId,
                    organizationId = GlobalState.organizationId.orEmpty(),
                    events = eventsJson.jsonArray.map { event ->
                        IntegrationEventInput(
                            event = event.jsonObject["event"]?.jsonPrimitive?.content.orEmpty(),
                            description = event.jsonObject["description"]?.jsonPrimitive?.content.orEmpty(),
                            deprecated = Optional.presentIfNotNull(
                                event.jsonObject["deprecated"]?.jsonPrimitive?.booleanOrNull
                                    ?: false
                            ),
                            deprecatedReason = Optional.presentIfNotNull(event.jsonObject["deprecatedReason"]?.jsonPrimitive?.content.orEmpty()),
                        )
                    }
                )
            )
        )
        runBlocking {
            networkRequestExecutor.requestExecutor(request).execute({ _ ->
            }, { error ->
                throw GradleException("The $integrationName failed to upload because: ${error.message}")
            })
        }
    }

    private fun syncData(
        integrationId: String,
        integrationName: String,
        dataJson: JsonElement,
    ) {
        val request = apolloClient.mutation(
            SyncIntegrationDataMutation(
                input = SyncIntegrationDataInput(
                    integrationId = integrationId,
                    organizationId = GlobalState.organizationId.orEmpty(),
                    data = dataJson.jsonArray.map { dataItem ->
                        IntegrationDataInput(
                            key = dataItem.jsonObject["key"]?.jsonPrimitive?.content.orEmpty(),
                            type = dataItem.jsonObject["type"]?.jsonPrimitive?.content.orEmpty(),
                            description = dataItem.jsonObject["description"]?.jsonPrimitive?.content.orEmpty(),
                            deprecated = Optional.presentIfNotNull(
                                dataItem.jsonObject["deprecated"]?.jsonPrimitive?.booleanOrNull
                                    ?: false
                            ),
                            deprecatedReason = Optional.presentIfNotNull(dataItem.jsonObject["deprecatedReason"]?.jsonPrimitive?.content.orEmpty()),
                        )
                    }
                )
            )
        )
        runBlocking {
            networkRequestExecutor.requestExecutor(request).execute({ _ ->
            }, { error ->
                throw GradleException("The $integrationName failed to upload because: ${error.message}")
            })
        }
    }

    private fun syncSlots(
        integrationId: String,
        integrationName: String,
        slotsJson: JsonElement,
    ) {
        val request = apolloClient.mutation(
            SyncIntegrationSlotsMutation(
                input = SyncIntegrationSlotsInput(
                    integrationId = integrationId,
                    organizationId = GlobalState.organizationId.orEmpty(),
                    slots = slotsJson.jsonArray.map { slot ->
                        IntegrationSlotInput(
                            slot = slot.jsonObject["slot"]?.jsonPrimitive?.content.orEmpty(),
                            description = slot.jsonObject["description"]?.jsonPrimitive?.content.orEmpty(),
                            deprecated = Optional.presentIfNotNull(
                                slot.jsonObject["deprecated"]?.jsonPrimitive?.booleanOrNull ?: false
                            ),
                            deprecatedReason = Optional.presentIfNotNull(slot.jsonObject["deprecatedReason"]?.jsonPrimitive?.content.orEmpty()),
                        )
                    }
                )
            )
        )
        runBlocking {
            networkRequestExecutor.requestExecutor(request).execute({ _ ->
            }, { error ->
                throw GradleException("The $integrationName failed to upload because: ${error.message}")
            })
        }
    }
}