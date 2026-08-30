package io.nativeblocks.compiler.modifier

import com.google.devtools.ksp.symbol.KSFunctionDeclaration
import com.google.devtools.ksp.symbol.KSVisitorVoid
import com.squareup.kotlinpoet.ClassName
import com.squareup.kotlinpoet.CodeBlock
import com.squareup.kotlinpoet.FileSpec
import com.squareup.kotlinpoet.FunSpec
import io.nativeblocks.compiler.meta.Data
import io.nativeblocks.compiler.meta.Event
import io.nativeblocks.compiler.meta.ExtraParam
import io.nativeblocks.compiler.util.PRIMITIVE_TYPES
import io.nativeblocks.compiler.util.converterVar
import io.nativeblocks.compiler.util.plusAssign
import io.nativeblocks.compiler.util.valueConversion
import java.io.OutputStream

internal class ModifierVisitor(
    private val file: OutputStream,
    private val fileName: String,
    private val packageName: String,
    private val consumerPackageName: String,
    private val metaEvents: MutableList<Event>,
    private val metaData: MutableList<Data>,
    private val extraParams: MutableList<ExtraParam>,
) : KSVisitorVoid() {

    override fun visitFunctionDeclaration(function: KSFunctionDeclaration, data: Unit) {
        val importComposable = ClassName("androidx.compose.runtime", "Composable")
        val importRemember = ClassName("androidx.compose.runtime", "remember")
        val importModifier = ClassName("androidx.compose.ui", "Modifier")

        val importModifierContext = ClassName("io.nativeblocks.runtime.api.provider.modifier", "ModifierContext")
        val importModifierProvideEvent = ClassName("io.nativeblocks.runtime.api.util", "modifierProvideEvent")
        val importNativeblocksManager = ClassName("io.nativeblocks.runtime.api", "NativeblocksManager")
        val importModifierFunction = ClassName(consumerPackageName, function.simpleName.asString())

        val func = FunSpec.builder(fileName)
            .addAnnotation(importComposable)
            .addParameter("modifierContext", importModifierContext)
            .returns(importModifier)
            .addComment("modifier meta fields")
            .addStatement("val data = modifierContext.modifier.data")

        val customDataTypes =
            metaData.map { it.typeClass.canonicalName }.filter { it !in PRIMITIVE_TYPES }.distinct()
        if (customDataTypes.isNotEmpty()) {
            func.addStatement("val manager = NativeblocksManager.getInstance(modifierContext.instanceName)")
        }
        if (metaData.isNotEmpty()) {
            func.addComment("modifier data")
            customDataTypes.forEach {
                func.addStatement("val ${converterVar(it)} = manager.getTypeConverter($it::class)")
            }
            metaData.forEach {
                func.addStatement("val ${it.key} = modifierContext.resolveTemplate(modifierContext.onFindVariable.invoke(data[\"${it.key}\"]))")
                func.beginControlFlow("val ${it.key}Value = remember(${it.key})")
                func.addStatement("val result = ${it.key}")
                func.addStatement(dataTypeMapper(it))
                func.endControlFlow()
            }
        }
        if (metaEvents.isNotEmpty()) {
            func.addComment("modifier events")
            metaEvents.forEach {
                func.addStatement("val ${it.event} = modifierProvideEvent(modifierContext, \"${it.event}\")")
            }
        }
        func.addComment("call the function")

        func.addCode("return ").addCode(function.simpleName.asString()).addCode("(")
            .addCode(CodeBlock.builder().indent().build())
            .addStatement("")

        extraParams.forEach {
            func.addStatement("${it.key} = ${it.key},")
        }

        metaData.forEach {
            func.addStatement("${it.key} = ${it.key}Value,")
        }

        metaEvents.forEach {
            val eventArg = function.parameters.find { arg -> arg.name?.asString() == it.event }
            val type = eventArg?.type?.resolve()
            val eventArgSize = type?.arguments?.size ?: 0
            val items = MutableList(eventArgSize) { index -> "p$index" }
            items.removeAt(items.lastIndex)

            if (type?.isMarkedNullable == true) {
                func.beginControlFlow("${it.event} = if (${it.event} != null)")
                func.addStatement("{ ${items.joinToString()} ->")
                it.dataBindings.forEachIndexed { index, dataBound ->
                    func.addStatement("modifierContext.onUpdateVariable.invoke(data[\"$dataBound\"], p${index}.toString())")
                }
                func.addStatement("${it.event}.invoke()")
                func.addStatement("}")
                func.addStatement("} else {")
                func.addStatement("null")
                func.addStatement("},")
            } else {
                func.addStatement("${it.event} = { ${items.joinToString()} ->")
                it.dataBindings.forEachIndexed { index, dataBound ->
                    func.addStatement("modifierContext.onUpdateVariable.invoke(data[\"$dataBound\"], p${index}.toString())")
                }
                func.addStatement("${it.event}?.invoke()")
                func.addStatement("},")
            }
        }
        func.addCode(")")

        val modifierFile = FileSpec.builder(packageName, fileName)
            .addImport(importModifierFunction, "")
            .addImport(importModifierProvideEvent, "")
            .addImport(importNativeblocksManager, "")
            .addImport(importRemember, "")
            .addFunction(func.build())
            .build()
        file += modifierFile.toString()
    }

    private fun dataTypeMapper(dataItem: Data): String =
        valueConversion(
            canonicalName = dataItem.typeClass.canonicalName,
            source = "result",
            default = dataItem.value
        )
}
