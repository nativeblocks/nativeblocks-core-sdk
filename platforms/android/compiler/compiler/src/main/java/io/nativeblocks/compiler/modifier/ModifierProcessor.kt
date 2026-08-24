package io.nativeblocks.compiler.modifier

import com.google.devtools.ksp.processing.Dependencies
import com.google.devtools.ksp.processing.Resolver
import com.google.devtools.ksp.processing.SymbolProcessor
import com.google.devtools.ksp.processing.SymbolProcessorEnvironment
import com.google.devtools.ksp.symbol.KSAnnotated
import com.google.devtools.ksp.symbol.KSAnnotation
import com.google.devtools.ksp.symbol.KSFunctionDeclaration
import com.google.devtools.ksp.symbol.KSValueParameter
import com.google.devtools.ksp.validate
import io.nativeblocks.compiler.generateDataJson
import io.nativeblocks.compiler.generateEventJson
import io.nativeblocks.compiler.generateIntegrationJson
import io.nativeblocks.compiler.getExtraParam
import io.nativeblocks.compiler.meta.Data
import io.nativeblocks.compiler.meta.Event
import io.nativeblocks.compiler.meta.ExtraParam
import io.nativeblocks.compiler.type.NativeModifier
import io.nativeblocks.compiler.type.NativeModifierData
import io.nativeblocks.compiler.type.NativeModifierEvent
import io.nativeblocks.compiler.util.Diagnostic
import io.nativeblocks.compiler.util.DiagnosticType
import io.nativeblocks.compiler.util.capitalize
import io.nativeblocks.compiler.util.getAnnotation
import io.nativeblocks.compiler.writeJson
import java.io.OutputStream

private const val PACKAGE_NAME_SUFFIX = ".integration.consumer.modifier"

internal class ModifierProcessor(private val environment: SymbolProcessorEnvironment) : SymbolProcessor {

    private val integrationKeyTypes = mutableListOf<String>()
    private val integrations = mutableListOf<ModifierFunctionModel>()

    override fun process(resolver: Resolver): List<KSAnnotated> {
        val symbols = resolver
            .getSymbolsWithAnnotation(annotationName = NativeModifier::class.qualifiedName.orEmpty())
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
            val integrationJson =
                function.getAnnotation(NativeModifier::class.simpleName.orEmpty()).generateIntegrationJson(
                    kind = "MODIFIER",
                    integrationKeyTypes = integrationKeyTypes
                )
            integrations.add(
                ModifierFunctionModel(
                    functionName = function.simpleName.asString(),
                    keyType = integrationJson.keyType
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

            val data = mutableListOf<Data>()
            val events = mutableListOf<Event>()
            val extraParams = mutableListOf<ExtraParam>()

            function.parameters.forEach { param ->
                if (param.annotations.firstOrNull() != null) {
                    val annotations = getNativeblocksAnnotations(param)
                    if (annotations.size > 1) {
                        throw Diagnostic.exceptionDispatcher(DiagnosticType.ConflictAnnotation)
                    }
                    when (val annotation = annotations.first().shortName.asString()) {
                        NativeModifierData::class.simpleName -> {
                            val dataJson = param.getAnnotation(annotation).generateDataJson(
                                param = param,
                            )
                            data.add(dataJson)
                        }

                        NativeModifierEvent::class.simpleName -> {
                            val eventJson = param.getAnnotation(annotation).generateEventJson(param = param)
                            events.add(eventJson)
                        }
                    }
                } else {
                    val extraParam = param.getExtraParam()
                    if (extraParam.key == "modifierContext" && extraParam.type == "io.nativeblocks.runtime.api.provider.modifier.ModifierContext")
                        extraParams.add(extraParam)
                }
            }

            writeJson(
                codeGenerator = environment.codeGenerator,
                packageName = fullPackageName.replace(".", "/") + "/" + function.simpleName.asString().capitalize(),
                fileName = "data",
                json = data,
                sources = containingFile
            )
            writeJson(
                codeGenerator = environment.codeGenerator,
                packageName = fullPackageName.replace(".", "/") + "/" + function.simpleName.asString().capitalize(),
                fileName = "events",
                json = events,
                sources = containingFile
            )

            val fileName = function.simpleName.asString() + "Modifier"
            val file: OutputStream = environment.codeGenerator.createNewFile(
                dependencies = Dependencies(false, sources = containingFile),
                packageName = fullPackageName,
                fileName = fileName,
            )
            function.accept(
                ModifierVisitor(
                    file = file,
                    fileName = fileName,
                    packageName = fullPackageName,
                    consumerPackageName = function.packageName.asString(),
                    metaEvents = events,
                    metaData = data,
                    extraParams = extraParams,
                ), Unit
            )
            file.close()
        }
        // the provider aggregates every modifier, so emit it once all symbols are processed
        if (deferredSymbols.isEmpty() && integrations.isNotEmpty()) {
            val sources = resolver.getAllFiles().toList().toTypedArray()
            val fileName = "${moduleName}ModifierProvider"
            val file: OutputStream = environment.codeGenerator.createNewFile(
                dependencies = Dependencies(aggregating = true, sources = sources),
                packageName = "$fullPackageName.provider",
                fileName = fileName,
            )
            ModifierProviderVisitor(file, fileName, (basePackageName + PACKAGE_NAME_SUFFIX), integrations)
            file.close()
        }
        return deferredSymbols
    }

    private fun getNativeblocksAnnotations(param: KSValueParameter): List<KSAnnotation> {
        val nativeblocksAnnotations = param.annotations.filter {
            it.shortName.asString() == NativeModifierData::class.simpleName ||
                    it.shortName.asString() == NativeModifierEvent::class.simpleName
        }
        return nativeblocksAnnotations.toList()
    }
}
