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
import io.nativeblocks.compiler.util.PRIMITIVE_PROP_TYPES
import io.nativeblocks.compiler.util.camelcase
import io.nativeblocks.compiler.util.converterVar
import io.nativeblocks.compiler.util.dataConversion
import io.nativeblocks.compiler.util.plusAssign
import io.nativeblocks.compiler.util.propertyConversion
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
        val importINativeAction = ClassName("io.nativeblocks.core.api.provider.action", "INativeAction")
        val importActionProps = ClassName("io.nativeblocks.core.api.provider.action", "ActionProps")
        val importNativeBlockModel = ClassName("io.nativeblocks.core.api.provider.model", "NativeBlockModel")
        val importNativeActionModel = ClassName("io.nativeblocks.core.api.provider.model", "NativeActionModel")
        val importNativeActionTriggerModel = ClassName("io.nativeblocks.core.api.provider.model", "NativeActionTriggerModel")
        val importNativeActionTriggerThen = ClassName("io.nativeblocks.core.api.provider.model", "NativeActionTriggerThen")
        val importNativeActionTriggerPropertyModel = ClassName("io.nativeblocks.core.api.provider.model", "NativeActionTriggerPropertyModel")
        val importNativeActionTriggerDataModel = ClassName("io.nativeblocks.core.api.provider.model", "NativeActionTriggerDataModel")
        val importCoroutinesLaunch = ClassName("kotlinx.coroutines", "launch")
        val importNativeblocksManager = ClassName("io.nativeblocks.core.api", "NativeblocksManager")
        val importActionKlass = ClassName(consumerPackageName, klass.simpleName.asString())

        val func = FunSpec.builder("handle")
            .addModifiers(KModifier.OVERRIDE)
            .addParameter("actionProps", importActionProps)
            .beginControlFlow("actionProps.coroutineScope.launch")
            .addComment("action meta fields")
            .addCode(
                """
                    |val data = actionProps.trigger?.data ?: mapOf()
                    |val properties = actionProps.trigger?.properties ?: mapOf()
                """.trimMargin()
            )
        func.addStatement("")
        func.addComment("action trigger data")
        metaData.forEach {
            func.addStatement("val ${it.key} = actionProps.onFindVariable.invoke(data[\"${it.key}\"]?.value.orEmpty())")
        }

        func.addComment("action trigger data value")
        metaData.forEach {
            func.addStatement("val ${it.key}Value = ${dataTypeMapper(it)}")
        }

        func.addComment("action trigger properties")
        if (metaProperties.any { it.typeClass.canonicalName !in PRIMITIVE_PROP_TYPES }) {
            func.addStatement("val manager = NativeblocksManager.getInstance(actionProps.instanceName)")
        }
        metaProperties.map { it.typeClass.canonicalName }.filter { it !in PRIMITIVE_PROP_TYPES }.distinct()
            .forEach {
                func.addStatement("val ${converterVar(it)} = manager.getTypeConverter($it::class)")
            }
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
                    .addStatement("actionProps.onChangeVariable.invoke(${dataBound}Updated)")
                    .endControlFlow()
            }
            when (it.then) {
                "SUCCESS" -> {
                    func.beginControlFlow("actionProps.trigger?.let")
                        .addStatement("actionProps.onHandleSuccessNextTrigger.invoke(it)")
                        .endControlFlow()
                }

                "FAILURE" -> {
                    func.beginControlFlow("actionProps.trigger?.let")
                        .addStatement("actionProps.onHandleFailureNextTrigger.invoke(it)")
                        .endControlFlow()
                }

                "NEXT" -> {
                    func.beginControlFlow("actionProps.trigger?.let")
                        .addStatement("actionProps.onHandleNextTrigger.invoke(it)")
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
            .addImport(importActionProps, "")
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
        propertyConversion(
            canonicalName = prop.typeClass.canonicalName,
            source = """properties["${prop.key}"]?.value""",
            default = prop.value
        )

    private fun dataTypeMapper(dataItem: Data): String =
        dataConversion(
            type = dataItem.type,
            source = "${dataItem.key}?.value",
            default = dataItem.value,
            key = dataItem.key
        )

}