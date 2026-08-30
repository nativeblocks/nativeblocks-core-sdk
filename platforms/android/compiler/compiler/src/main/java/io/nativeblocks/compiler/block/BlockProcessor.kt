package io.nativeblocks.compiler.block

import com.google.devtools.ksp.containingFile
import com.google.devtools.ksp.processing.Dependencies
import com.google.devtools.ksp.processing.Resolver
import com.google.devtools.ksp.processing.SymbolProcessor
import com.google.devtools.ksp.processing.SymbolProcessorEnvironment
import com.google.devtools.ksp.symbol.KSAnnotated
import com.google.devtools.ksp.symbol.KSAnnotation
import com.google.devtools.ksp.symbol.KSFunctionDeclaration
import com.google.devtools.ksp.symbol.KSValueParameter
import com.google.devtools.ksp.validate
import io.nativeblocks.compiler.generateBindingDataJson
import io.nativeblocks.compiler.generateDataJson
import io.nativeblocks.compiler.generateEventJson
import io.nativeblocks.compiler.generateIntegrationJson
import io.nativeblocks.compiler.generatePropertyJson
import io.nativeblocks.compiler.generateSlotJson
import io.nativeblocks.compiler.getExtraParam
import io.nativeblocks.compiler.undeclaredBindingData
import io.nativeblocks.compiler.validateSlotBindings
import io.nativeblocks.compiler.meta.Data
import io.nativeblocks.compiler.meta.Event
import io.nativeblocks.compiler.meta.ExtraParam
import io.nativeblocks.compiler.meta.Property
import io.nativeblocks.compiler.meta.Slot
import io.nativeblocks.compiler.type.Block
import io.nativeblocks.compiler.type.BlockData
import io.nativeblocks.compiler.type.BlockEvent
import io.nativeblocks.compiler.type.BlockProp
import io.nativeblocks.compiler.type.BlockSlot
import io.nativeblocks.compiler.util.Diagnostic
import io.nativeblocks.compiler.util.DiagnosticType
import io.nativeblocks.compiler.util.capitalize
import io.nativeblocks.compiler.util.getAnnotation
import io.nativeblocks.compiler.writeJson
import java.io.OutputStream

internal const val DESCRIBE_SCOPE_PARAM = "describeScope"

private const val PACKAGE_NAME_SUFFIX = ".integration.consumer.block"

internal class BlockProcessor(private val environment: SymbolProcessorEnvironment) : SymbolProcessor {

    private val integrationKeyTypes = mutableListOf<String>()
    private val integrations = mutableListOf<BlockFunctionModel>()

    override fun process(resolver: Resolver): List<KSAnnotated> {
        val symbols = resolver
            .getSymbolsWithAnnotation(annotationName = Block::class.qualifiedName.orEmpty())
            .filterIsInstance<KSFunctionDeclaration>()
            .toList()

        val basePackageName = environment.options["basePackageName"].orEmpty()
        val moduleName = environment.options["moduleName"].orEmpty()

        if (basePackageName.isEmpty() || moduleName.isEmpty()) {
            throw Diagnostic.exceptionDispatcher(DiagnosticType.KspArgNotFound)
        }

        val fullPackageName = basePackageName + PACKAGE_NAME_SUFFIX

        if (symbols.isEmpty()) return emptyList()

        val (validSymbols, deferredSymbols) = symbols.partition { it.validate() }

        validSymbols.forEach { function ->
            val containingFile = listOfNotNull(function.containingFile).toTypedArray()
            // check component duplication (MyButton and myButton are the same from the compiler prospective, we need to normalize it and throw an error)
            val integrationJson =
                function.getAnnotation(Block::class.simpleName.orEmpty()).generateIntegrationJson(
                    kind = "BLOCK",
                    integrationKeyTypes = integrationKeyTypes
                )
            val describing = function.parameters.any { param ->
                param.name?.asString() == DESCRIBE_SCOPE_PARAM
            }
            integrations.add(
                BlockFunctionModel(
                    functionName = function.simpleName.asString(),
                    keyType = integrationJson.keyType,
                    describing = describing,
                )
            )
            val packageName = fullPackageName.replace(".", "/")

            writeJson(
                codeGenerator = environment.codeGenerator,
                packageName = packageName + "/" + function.simpleName.asString().capitalize(),
                fileName = "integration",
                json = integrationJson,
                sources = containingFile
            )

            val properties = mutableListOf<Property>()
            val data = mutableListOf<Data>()
            val events = mutableListOf<Event>()
            val slots = mutableListOf<Slot>()
            val extraParams = mutableListOf<ExtraParam>()
            val bindingData = mutableListOf<Data>()

            function.parameters.forEach { param ->
                // check the field has any annotation or not
                if (param.annotations.firstOrNull() != null) {
                    // we need to check the field just has one annotation at the same time, if there is more throw an error, else continue the process
                    val annotations = getNativeblocksAnnotations(param)
                    if (annotations.size > 1) {
                        throw Diagnostic.exceptionDispatcher(DiagnosticType.ConflictAnnotation)
                    }
                    when (val annotation = annotations.first().shortName.asString()) {
                        BlockProp::class.simpleName -> {
                            val propertyJson = param.getAnnotation(annotation).generatePropertyJson(
                                param = param,
                                kind = integrationJson.kind,
                                filePath = param.containingFile?.filePath.orEmpty()
                            )
                            properties.add(propertyJson)
                        }

                        BlockData::class.simpleName -> {
                            val dataJson = param.getAnnotation(annotation).generateDataJson(
                                param = param,
                            )
                            data.add(dataJson)
                        }

                        BlockEvent::class.simpleName -> {
                            val eventJson = param.getAnnotation(annotation).generateEventJson(param = param)
                            events.add(eventJson)
                            bindingData.addAll(param.generateBindingDataJson(eventJson.event, eventJson.dataBindings))
                        }

                        BlockSlot::class.simpleName -> {
                            val slotJson = param.getAnnotation(annotation).generateSlotJson(
                                param = param,
                            )
                            slots.add(slotJson)
                        }
                    }
                } else {
                    val extraParam = param.getExtraParam()
                    if (extraParam.key == "blockContext" && extraParam.type == "io.nativeblocks.runtime.api.provider.block.BlockContext")
                        extraParams.add(extraParam)
                    if (extraParam.key == DESCRIBE_SCOPE_PARAM)
                        extraParams.add(extraParam)
                }
            }

            val syntheticData = undeclaredBindingData(data, bindingData)
            validateSlotBindings(slots, data + syntheticData)

            writeJson(
                codeGenerator = environment.codeGenerator,
                packageName = fullPackageName.replace(".", "/") + "/" + function.simpleName.asString().capitalize(),
                fileName = "properties",
                json = properties,
                sources = containingFile
            )
            writeJson(
                codeGenerator = environment.codeGenerator,
                packageName = fullPackageName.replace(".", "/") + "/" + function.simpleName.asString().capitalize(),
                fileName = "events",
                json = events,
                sources = containingFile
            )
            writeJson(
                codeGenerator = environment.codeGenerator,
                packageName = fullPackageName.replace(".", "/") + "/" + function.simpleName.asString().capitalize(),
                fileName = "data",
                json = data + syntheticData,
                sources = containingFile
            )
            writeJson(
                codeGenerator = environment.codeGenerator,
                packageName = fullPackageName.replace(".", "/") + "/" + function.simpleName.asString().capitalize(),
                fileName = "slots",
                json = slots,
                sources = containingFile
            )

            val fileName = function.simpleName.asString() + "Block"
            val file: OutputStream = environment.codeGenerator.createNewFile(
                dependencies = Dependencies(false, sources = containingFile),
                packageName = fullPackageName,
                fileName = fileName,
            )
            function.accept(
                BlockVisitor(
                    file = file,
                    fileName = fileName,
                    packageName = fullPackageName,
                    consumerPackageName = function.packageName.asString(),
                    metaProperties = properties,
                    metaEvents = events,
                    metaData = data,
                    describing = describing,
                    metaSlots = slots,
                    extraParams = extraParams,
                ), Unit
            )
            file.close()
        }
        // the provider aggregates every block, so emit it once all symbols are processed
        if (deferredSymbols.isEmpty() && integrations.isNotEmpty()) {
            val sources = resolver.getAllFiles().toList().toTypedArray()
            val fileName = "${moduleName}BlockProvider"
            val file: OutputStream = environment.codeGenerator.createNewFile(
                dependencies = Dependencies(aggregating = true, sources = sources),
                packageName = "$fullPackageName.provider",
                fileName = fileName,
            )
            BlockProviderVisitor(file, fileName, (basePackageName + PACKAGE_NAME_SUFFIX), integrations)
            file.close()
        }
        return deferredSymbols
    }

    private fun getNativeblocksAnnotations(param: KSValueParameter): List<KSAnnotation> {
        val nativeblocksAnnotations = param.annotations.filter {
            it.shortName.asString() == BlockData::class.simpleName ||
                    it.shortName.asString() == BlockSlot::class.simpleName ||
                    it.shortName.asString() == BlockEvent::class.simpleName ||
                    it.shortName.asString() == BlockProp::class.simpleName
        }
        return nativeblocksAnnotations.toList()
    }
}