package io.nativeblocks.compiler.action

import com.google.devtools.ksp.containingFile
import com.google.devtools.ksp.processing.Dependencies
import com.google.devtools.ksp.processing.Resolver
import com.google.devtools.ksp.processing.SymbolProcessor
import com.google.devtools.ksp.processing.SymbolProcessorEnvironment
import com.google.devtools.ksp.symbol.ClassKind
import com.google.devtools.ksp.symbol.KSAnnotated
import com.google.devtools.ksp.symbol.KSAnnotation
import com.google.devtools.ksp.symbol.KSClassDeclaration
import com.google.devtools.ksp.symbol.KSValueParameter
import com.google.devtools.ksp.symbol.Modifier
import com.google.devtools.ksp.validate
import io.nativeblocks.compiler.generateBindingDataJson
import io.nativeblocks.compiler.generateDataJson
import io.nativeblocks.compiler.generateEventJson
import io.nativeblocks.compiler.generateIntegrationJson
import io.nativeblocks.compiler.generatePropertyJson
import io.nativeblocks.compiler.getExtraParam
import io.nativeblocks.compiler.undeclaredBindingData
import io.nativeblocks.compiler.meta.Data
import io.nativeblocks.compiler.meta.Event
import io.nativeblocks.compiler.meta.ExtraParam
import io.nativeblocks.compiler.meta.Property
import io.nativeblocks.compiler.type.Action
import io.nativeblocks.compiler.type.ActionData
import io.nativeblocks.compiler.type.ActionEvent
import io.nativeblocks.compiler.type.ActionFunction
import io.nativeblocks.compiler.type.ActionParameter
import io.nativeblocks.compiler.type.ActionProp
import io.nativeblocks.compiler.util.Diagnostic
import io.nativeblocks.compiler.util.DiagnosticType
import io.nativeblocks.compiler.util.capitalize
import io.nativeblocks.compiler.util.getAnnotation
import io.nativeblocks.compiler.writeJson
import java.io.OutputStream

private const val PACKAGE_NAME_SUFFIX = ".integration.consumer.action"

internal class ActionProcessor(private val environment: SymbolProcessorEnvironment) : SymbolProcessor {

    private val integrationKeyTypes = mutableListOf<String>()
    private val integrations = mutableListOf<ActionFunctionModel>()

    override fun process(resolver: Resolver): List<KSAnnotated> {
        val symbols = resolver
            .getSymbolsWithAnnotation(annotationName = Action::class.qualifiedName.orEmpty())
            .filterIsInstance<KSClassDeclaration>()
            .toList()

        val basePackageName = environment.options["basePackageName"].orEmpty()
        val moduleName = environment.options["moduleName"].orEmpty()

        if (basePackageName.isEmpty() || moduleName.isEmpty()) {
            throw Diagnostic.exceptionDispatcher(DiagnosticType.KspArgNotFound)
        }

        val fullPackageName = basePackageName + PACKAGE_NAME_SUFFIX

        if (symbols.isEmpty()) return emptyList()

        val (validSymbols, deferredSymbols) = symbols.partition { it.validate() }

        validSymbols.forEach { klass ->
            val containingFile = listOfNotNull(klass.containingFile).toTypedArray()
            // check action duplication (myAction and MyAction are the same from the compiler prospective, we need to normalize it and throw an error)
            val integrationJson =
                klass.getAnnotation(Action::class.simpleName.orEmpty()).generateIntegrationJson(
                    kind = "ACTION",
                    integrationKeyTypes = integrationKeyTypes
                )
            integrations.add(
                ActionFunctionModel(
                    packageName = klass.packageName.asString(),
                    className = klass.simpleName.asString(),
                    keyType = integrationJson.keyType
                )
            )
            val packageName = fullPackageName.replace(".", "/")
            writeJson(
                codeGenerator = environment.codeGenerator,
                packageName = packageName + "/" + klass.simpleName.asString().capitalize(),
                fileName = "integration",
                json = integrationJson,
                sources = containingFile
            )

            val properties = mutableListOf<Property>()
            val data = mutableListOf<Data>()
            val events = mutableListOf<Event>()
            val extraParams = mutableListOf<ExtraParam>()
            val bindingData = mutableListOf<Data>()

            val functions = klass.getAllFunctions().filter { function ->
                function.annotations.filter {
                    it.shortName.asString() == ActionFunction::class.simpleName
                }.toList().isNotEmpty()
            }.toList()

            if (functions.size != 1) {
                throw Diagnostic.exceptionDispatcher(DiagnosticType.RequireFunctionAnnotation)
            }

            val parameters = klass.declarations
                .filterIsInstance<KSClassDeclaration>()
                .filter { it.classKind == ClassKind.CLASS }
                .filter { it.modifiers.contains(Modifier.DATA) }
                .filter { innerKlass ->
                    innerKlass.annotations.filter {
                        it.shortName.asString() == ActionParameter::class.simpleName
                    }.toList().isNotEmpty()
                }.toList()

            if (parameters.size != 1) {
                throw Diagnostic.exceptionDispatcher(DiagnosticType.RequireFunctionParameterAnnotation)
            }

            val primaryConstructor = parameters.first().primaryConstructor
            primaryConstructor?.let { constructor ->
                constructor.parameters.forEach { param ->
                    // check the field has any annotation or not
                    if (param.annotations.firstOrNull() != null) {
                        // we need to check the field just has one annotation at the same time, if there is more throw an error, else continue the process
                        val annotations = getNativeblocksAnnotations(param)
                        if (annotations.size > 1) {
                            throw Diagnostic.exceptionDispatcher(DiagnosticType.ConflictAnnotation)
                        }
                        when (val annotation = annotations.first().shortName.asString()) {
                            ActionProp::class.simpleName -> {
                                val propertyJson = param.getAnnotation(annotation).generatePropertyJson(
                                    param = param,
                                    kind = integrationJson.kind,
                                    filePath = param.containingFile?.filePath.orEmpty()
                                )
                                properties.add(propertyJson)
                            }

                            ActionData::class.simpleName -> {
                                val dataItem = param.getAnnotation(annotation).generateDataJson(
                                    param = param,
                                )
                                data.add(dataItem)
                            }

                            ActionEvent::class.simpleName -> {
                                val event = param.getAnnotation(annotation).generateEventJson(param = param)
                                events.add(event)
                                bindingData.addAll(param.generateBindingDataJson(event.event, event.dataBindings))
                            }
                        }
                    } else {
                        val extraParam = param.getExtraParam()
                        if (extraParam.key == "actionContext" && extraParam.type == "io.nativeblocks.runtime.api.provider.action.ActionContext")
                            extraParams.add(extraParam)
                    }
                }
            }

            val syntheticData = undeclaredBindingData(data, bindingData)

            writeJson(
                codeGenerator = environment.codeGenerator,
                packageName = fullPackageName.replace(".", "/") + "/" + klass.simpleName.asString().capitalize(),
                fileName = "properties",
                json = properties,
                sources = containingFile
            )
            writeJson(
                codeGenerator = environment.codeGenerator,
                packageName = fullPackageName.replace(".", "/") + "/" + klass.simpleName.asString().capitalize(),
                fileName = "events",
                json = events,
                sources = containingFile
            )
            writeJson(
                codeGenerator = environment.codeGenerator,
                packageName = fullPackageName.replace(".", "/") + "/" + klass.simpleName.asString().capitalize(),
                fileName = "data",
                json = data + syntheticData,
                sources = containingFile
            )

            val fileName = klass.simpleName.asString() + "Action"
            val file: OutputStream = environment.codeGenerator.createNewFile(
                dependencies = Dependencies(false, sources = containingFile),
                packageName = fullPackageName,
                fileName = fileName,
            )
            klass.accept(
                ActionVisitor(
                    file = file,
                    fileName = fileName,
                    function = functions.first(),
                    functionParameter = parameters.first(),
                    packageName = fullPackageName,
                    consumerPackageName = klass.packageName.asString(),
                    klass = klass,
                    metaProperties = properties,
                    metaEvents = events,
                    metaData = data,
                    metaBindingData = syntheticData,
                    extraParams = extraParams,
                ), Unit
            )
            file.close()
        }

        // the provider aggregates every action, so emit it once all symbols are processed
        if (deferredSymbols.isEmpty() && integrations.isNotEmpty()) {
            val sources = resolver.getAllFiles().toList().toTypedArray()
            val fileName = "${moduleName}ActionProvider"
            val file: OutputStream = environment.codeGenerator.createNewFile(
                dependencies = Dependencies(aggregating = true, sources = sources),
                packageName = "$fullPackageName.provider",
                fileName = fileName,
            )
            ActionProviderVisitor(file, fileName, fullPackageName, integrations)
            file.close()
        }

        return deferredSymbols
    }

    private fun getNativeblocksAnnotations(param: KSValueParameter): List<KSAnnotation> {
        val nativeblocksAnnotations = param.annotations.filter {
            it.shortName.asString() == ActionData::class.simpleName ||
                    it.shortName.asString() == ActionEvent::class.simpleName ||
                    it.shortName.asString() == ActionProp::class.simpleName
        }
        return nativeblocksAnnotations.toList()
    }
}