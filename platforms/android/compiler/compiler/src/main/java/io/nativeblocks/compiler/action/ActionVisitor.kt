package io.nativeblocks.compiler.action

import com.google.devtools.ksp.symbol.KSClassDeclaration
import com.google.devtools.ksp.symbol.KSFunctionDeclaration
import com.google.devtools.ksp.symbol.KSVisitorVoid
import com.squareup.kotlinpoet.ClassName
import com.squareup.kotlinpoet.CodeBlock
import com.squareup.kotlinpoet.FileSpec
import com.squareup.kotlinpoet.FunSpec
import com.squareup.kotlinpoet.KModifier
import com.squareup.kotlinpoet.PropertySpec
import com.squareup.kotlinpoet.TypeSpec
import io.nativeblocks.compiler.meta.Data
import io.nativeblocks.compiler.meta.Event
import io.nativeblocks.compiler.meta.ExtraParam
import io.nativeblocks.compiler.meta.Property
import io.nativeblocks.compiler.util.PRIMITIVE_TYPES
import io.nativeblocks.compiler.util.camelcase
import io.nativeblocks.compiler.util.converterVar
import io.nativeblocks.compiler.util.plusAssign
import io.nativeblocks.compiler.util.valueConversion
import java.io.OutputStream

internal class ActionVisitor(
    private val file: OutputStream,
    private val fileName: String,
    private val function: KSFunctionDeclaration,
    private val functionParameter: KSClassDeclaration,
    private val packageName: String,
    private val consumerPackageName: String,
    private val klass: KSClassDeclaration,
    private val metaProperties: MutableList<Property>,
    private val metaEvents: MutableList<Event>,
    private val metaData: MutableList<Data>,
    private val extraParams: MutableList<ExtraParam>,
) : KSVisitorVoid() {

    override fun visitClassDeclaration(classDeclaration: KSClassDeclaration, data: Unit) {
        val importINativeAction = ClassName("io.nativeblocks.runtime.api.provider.action", "INativeAction")
        val importActionContext = ClassName("io.nativeblocks.runtime.api.provider.action", "ActionContext")
        val importNativeBlockModel = ClassName("io.nativeblocks.runtime.api.provider.model", "NativeBlockModel")
        val importNativeActionModel = ClassName("io.nativeblocks.runtime.api.provider.model", "NativeActionModel")
        val importNativeActionTriggerModel = ClassName("io.nativeblocks.runtime.api.provider.model", "NativeActionTriggerModel")
        val importNativeActionTriggerThen = ClassName("io.nativeblocks.runtime.api.provider.model", "NativeActionTriggerThen")
        val importNativeActionTriggerPropertyModel = ClassName("io.nativeblocks.runtime.api.provider.model", "NativeActionTriggerPropertyModel")
        val importNativeActionTriggerDataModel = ClassName("io.nativeblocks.runtime.api.provider.model", "NativeActionTriggerDataModel")
        val importCoroutinesLaunch = ClassName("kotlinx.coroutines", "launch")
        val importNativeblocksManager = ClassName("io.nativeblocks.runtime.api", "NativeblocksManager")
        val importActionKlass = ClassName(consumerPackageName, klass.simpleName.asString())

        val func = FunSpec.builder("handle")
            .addModifiers(KModifier.OVERRIDE)
            .addParameter("actionContext", importActionContext)
            .beginControlFlow("actionContext.coroutineScope.launch")
            .addComment("action meta fields")
            .addCode(
                """
                    |val data = actionContext.trigger?.data ?: mapOf()
                    |val properties = actionContext.trigger?.properties ?: mapOf()
                """.trimMargin()
            )
        func.addStatement("")
        val customDataTypes =
            metaData.map { it.typeClass.canonicalName }.filter { it !in PRIMITIVE_TYPES }.distinct()
        val customTypeClasss =
            metaProperties.map { it.typeClass.canonicalName }.filter { it !in PRIMITIVE_TYPES }
                .distinct()
        if (customDataTypes.isNotEmpty() || customTypeClasss.isNotEmpty()) {
            func.addStatement("val manager = NativeblocksManager.getInstance(actionContext.instanceName)")
        }
        (customDataTypes + customTypeClasss).distinct().forEach {
            func.addStatement("val ${converterVar(it)} = manager.getTypeConverter($it::class)")
        }

        func.addComment("action trigger data")
        metaData.forEach {
            func.addStatement("val ${it.key} = actionContext.onFindVariable.invoke(data[\"${it.key}\"]?.value.orEmpty())")
        }

        func.addComment("action trigger data value")
        metaData.forEach {
            func.addStatement("val ${it.key}Value = ${dataTypeMapper(it)}")
        }

        func.addComment("action trigger properties")
        metaProperties.forEach {
            func.addStatement("val ${it.key} = ${propTypeMapper(it)}")
        }

        func.addCode(klass.simpleName.asString().camelcase() + "." + function.simpleName.asString())
            .addCode("(")
            .addCode(CodeBlock.builder().indent().build())
            .addCode(klass.simpleName.asString() + "." + functionParameter.simpleName.asString() + "(")
            .addStatement("")

        extraParams.forEach {
            func.addStatement("${it.key} = ${it.key},")
        }

        metaData.forEach {
            func.addStatement("${it.key} = ${it.key}Value,")
        }

        metaProperties.forEach {
            func.addStatement("${it.key} = ${it.key},")
        }

        metaEvents.forEach {
            val eventArg = functionParameter.primaryConstructor?.parameters?.find { arg ->
                arg.name?.asString() == it.functionName
            }
            val eventArgSize = eventArg?.type?.resolve()?.arguments?.size ?: 0
            val items = MutableList(eventArgSize) { index -> "p$index" }
            items.removeAt(items.lastIndex)

            func.addStatement("${it.functionName} = { ${items.joinToString()} ->")
            it.dataBinding.forEachIndexed { index, dataBound ->
                func.addStatement("val ${dataBound}Updated = $dataBound?.copy(value = p${index}.toString())")
                    .beginControlFlow("if (${dataBound}Updated != null)")
                    .addStatement("actionContext.onUpdateVariable.invoke(${dataBound}Updated)")
                    .endControlFlow()
            }
            when (it.then) {
                "SUCCESS" -> {
                    func.beginControlFlow("actionContext.trigger?.let")
                        .addStatement("actionContext.onHandleSuccessNextTrigger.invoke(it)")
                        .endControlFlow()
                }

                "FAILURE" -> {
                    func.beginControlFlow("actionContext.trigger?.let")
                        .addStatement("actionContext.onHandleFailureNextTrigger.invoke(it)")
                        .endControlFlow()
                }

                "NEXT" -> {
                    func.beginControlFlow("actionContext.trigger?.let")
                        .addStatement("actionContext.onHandleNextTrigger.invoke(it)")
                        .endControlFlow()
                }

                "END" -> {}
            }
            func.addStatement("},")
        }
        func.addCode(")")
        func.addCode(")")
        func.endControlFlow()

        val flux = FunSpec.constructorBuilder()
            .addParameter(klass.simpleName.asString().camelcase(), importActionKlass)
            .build()

        val actionClass = FileSpec.builder(packageName, fileName)
            .addImport(importActionContext, "")
            .addImport(importNativeBlockModel, "")
            .addImport(importNativeActionModel, "")
            .addImport(importNativeActionTriggerModel, "")
            .addImport(importNativeActionTriggerThen, "")
            .addImport(importNativeActionTriggerPropertyModel, "")
            .addImport(importNativeActionTriggerDataModel, "")
            .addImport(importCoroutinesLaunch, "")
            .addImport(importActionKlass, "")
            .addImport(importNativeblocksManager, "")
            .addType(
                TypeSpec.classBuilder(fileName)
                    .primaryConstructor(flux)
                    .addProperty(
                        PropertySpec.builder(klass.simpleName.asString().camelcase(), importActionKlass)
                            .initializer(klass.simpleName.asString().camelcase())
                            .addModifiers(KModifier.PRIVATE)
                            .build()
                    )
                    .addSuperinterface(importINativeAction)
                    .addFunction(func.build())
                    .build()
            ).build()
        file += actionClass.toString()
    }

    private fun propTypeMapper(prop: Property): String =
        valueConversion(
            canonicalName = prop.typeClass.canonicalName,
            source = """properties["${prop.key}"]?.value""",
            default = prop.value
        )

    private fun dataTypeMapper(dataItem: Data): String =
        valueConversion(
            canonicalName = dataItem.typeClass.canonicalName,
            source = "${dataItem.key}?.value",
            default = dataItem.value
        )

}