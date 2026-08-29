package io.nativeblocks.compiler.block

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
import io.nativeblocks.compiler.meta.Slot
import io.nativeblocks.compiler.util.Diagnostic
import io.nativeblocks.compiler.util.DiagnosticType
import io.nativeblocks.compiler.util.PRIMITIVE_TYPES
import io.nativeblocks.compiler.util.converterVar
import io.nativeblocks.compiler.util.plusAssign
import io.nativeblocks.compiler.util.valueConversion
import java.io.OutputStream

internal class BlockVisitor(
    private val file: OutputStream,
    private val fileName: String,
    private val packageName: String,
    private val consumerPackageName: String,
    private val metaProperties: MutableList<Property>,
    private val metaEvents: MutableList<Event>,
    private val metaData: MutableList<Data>,
    private val metaSlots: MutableList<Slot>,
    private val extraParams: MutableList<ExtraParam>,
) : KSVisitorVoid() {

    override fun visitFunctionDeclaration(function: KSFunctionDeclaration, data: Unit) {
        val importComposable = ClassName("androidx.compose.runtime", "Composable")
        val importRemember = ClassName("androidx.compose.runtime", "remember")

        val importBlockContext = ClassName("io.nativeblocks.runtime.api.provider.block", "BlockContext")
        val importBlockFindWindowSizeClass = ClassName("io.nativeblocks.runtime.api.util", "findWindowSizeClass")
        val importLocalWindowWidthClass = ClassName("io.nativeblocks.runtime.api.util", "LocalNativeWindowWidthClass")
        val importBlockProvideEvent = ClassName("io.nativeblocks.runtime.api.util", "blockProvideEvent")
        val importNativeblocksManager = ClassName("io.nativeblocks.runtime.api", "NativeblocksManager")
        val importBlockFunction = ClassName(consumerPackageName, function.simpleName.asString())
        val importBlockProvideSlot = ClassName("io.nativeblocks.runtime.api.util", "blockProvideSlot")

        val func = FunSpec.builder(fileName)
            .addAnnotation(importComposable)
            .addParameter("blockContext", importBlockContext)
            .addComment("block meta fields")
            .addCode(
                """
                    |val data = blockContext.block.data
                    |val properties = blockContext.block.properties
                """.trimMargin()
            )
        func.addStatement("")
        val customDataTypes =
            metaData.map { it.typeClass.canonicalName }.filter { it !in PRIMITIVE_TYPES }.distinct()
        val customTypeClasses =
            metaProperties.map { it.typeClass.canonicalName }.filter { it !in PRIMITIVE_TYPES }
                .distinct()
        if (customDataTypes.isNotEmpty() || customTypeClasses.isNotEmpty()) {
            func.addStatement("val manager = NativeblocksManager.getInstance(blockContext.instanceName)")
        }
        if (metaData.isNotEmpty()) {
            func.addComment("block data")
            customDataTypes.forEach {
                func.addStatement("val ${converterVar(it)} = manager.getTypeConverter($it::class)")
            }
            metaData.forEach {
                func.addStatement("val ${it.key} = blockContext.onFindVariable.invoke(data[\"${it.key}\"])")
                func.beginControlFlow("val ${it.key}Value = remember(${it.key})")
                func.addStatement("val result = ${it.key}")
                func.addStatement(dataTypeMapper(it))
                func.endControlFlow()
            }
        }
        if (metaProperties.isNotEmpty()) {
            func.addComment("block properties")
            func.addStatement("val windowManager = LocalNativeWindowWidthClass.current")
            func.beginControlFlow("val resolvedProperties = remember(properties, windowManager)")
            customTypeClasses.forEach {
                func.addStatement("val ${converterVar(it)} = manager.getTypeConverter($it::class)")
            }
            func.addStatement("${fileName}ResolvedProperties(")
            metaProperties.forEach {
                func.addStatement("${it.key} = ${propTypeMapper(it)},")
            }
            func.addStatement(")")
            func.endControlFlow()
        }
        if (metaSlots.isNotEmpty()) {
            func.addComment("block slots")
            metaSlots.forEach {
                func.addStatement("val ${it.slot} = blockProvideSlot(blockContext, \"${it.slot}\") ")
            }
        }
        if (metaEvents.isNotEmpty()) {
            func.addComment("block events")
            metaEvents.forEach {
                func.addStatement("val ${it.event} = blockProvideEvent(blockContext, \"${it.event}\")")
            }
        }
        func.addComment("call the function")

        func.addCode(function.simpleName.asString()).addCode("(")
            .addCode(CodeBlock.builder().indent().build())
            .addStatement("")

        extraParams.forEach {
            func.addStatement("${it.key} = ${it.key},")
        }

        metaData.forEach {
            func.addStatement("${it.key} = ${it.key}Value,")
        }

        metaProperties.forEach {
            func.addStatement("${it.key} = resolvedProperties.${it.key},")
        }

        metaSlots.forEach {
            val slotArg = function.parameters.find { arg -> arg.name?.asString() == it.slot }
            val type = slotArg?.type?.resolve()

            if (type?.isFunctionType == false) {
                throw Diagnostic.exceptionDispatcher(DiagnosticType.SlotMustBeComposable)
            }

            val blockIndexes = type?.arguments?.filter { ksArg ->
                ksArg.type?.resolve()?.declaration?.simpleName?.asString() == "BlockIndex"
            }
            val blockScope = type?.arguments?.filter { ksArg ->
                ksArg.type?.resolve()?.declaration?.simpleName?.asString() == "Any"
            }
            if (blockIndexes.isNullOrEmpty()) {
                throw Diagnostic.exceptionDispatcher(DiagnosticType.SlotComposableIndex)
            }
            if (type.isMarkedNullable) {
                func.beginControlFlow("${it.slot} = if (${it.slot} != null)")
                if (blockScope.isNullOrEmpty()) {
                    func.addStatement("@Composable { index -> ")
                    func.addStatement("blockContext.onSubBlock.invoke(blockContext.block?.subBlocks.orEmpty(), ${it.slot}, index, null)")
                } else {
                    func.addStatement("@Composable { index, scope -> ")
                    func.addStatement("blockContext.onSubBlock.invoke(blockContext.block?.subBlocks.orEmpty(), ${it.slot}, index, scope)")
                }
                func.endControlFlow()
                func.addStatement("} else {")
                func.addStatement("null")
                func.addStatement("},")
            } else {
                if (blockScope.isNullOrEmpty()) {
                    func.addStatement("${it.slot} = @Composable { index -> ")
                    func.beginControlFlow("if (${it.slot} != null)")
                    func.addStatement("blockContext.onSubBlock.invoke(blockContext.block.subBlocks.orEmpty(), ${it.slot}, index, null)")
                } else {
                    func.addStatement("${it.slot} = @Composable { index, scope -> ")
                    func.beginControlFlow("if (${it.slot} != null)")
                    func.addStatement("blockContext.onSubBlock.invoke(blockContext.block.subBlocks.orEmpty(), ${it.slot}, index, scope)")
                }
                func.endControlFlow()
                func.addStatement("},")
            }
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
                    func.addStatement("blockContext.onUpdateVariable.invoke(data[\"$dataBound\"], p${index}.toString())")
                }
                func.addStatement("${it.event}.invoke()")
                func.addStatement("}")
                func.addStatement("} else {")
                func.addStatement("null")
                func.addStatement("},")
            } else {
                func.addStatement("${it.event} = { ${items.joinToString()} ->")
                it.dataBindings.forEachIndexed { index, dataBound ->
                    func.addStatement("blockContext.onUpdateVariable.invoke(data[\"$dataBound\"], p${index}.toString())")
                }
                func.addStatement("${it.event}?.invoke()")
                func.addStatement("},")
            }
        }
        func.addCode(")")

        val blockFileBuilder = FileSpec.builder(packageName, fileName)
            .addImport(importBlockFunction, "")
            .addImport(importBlockProvideSlot, "")
            .addImport(importBlockFindWindowSizeClass, "")
            .addImport(importLocalWindowWidthClass, "")
            .addImport(importBlockProvideEvent, "")
            .addImport(importNativeblocksManager, "")
            .addImport(importRemember, "")

        if (metaProperties.isNotEmpty()) {
            blockFileBuilder.addType(resolvedPropertiesType(fileName))
        }

        val blockFile = blockFileBuilder
            .addFunction(func.build())
            .build()
        file += blockFile.toString()
    }

    private fun resolvedPropertiesType(fileName: String): TypeSpec {
        val constructor = FunSpec.constructorBuilder()
        metaProperties.forEach { constructor.addParameter(it.key, propTypeName(it)) }
        val type = TypeSpec.classBuilder("${fileName}ResolvedProperties")
            .addModifiers(KModifier.PRIVATE, KModifier.DATA)
            .primaryConstructor(constructor.build())
        metaProperties.forEach {
            type.addProperty(
                PropertySpec.builder(it.key, propTypeName(it)).initializer(it.key).build()
            )
        }
        return type.build()
    }

    private fun propTypeName(prop: Property): ClassName {
        return ClassName(prop.typeClass.packageName, prop.typeClass.simpleNames)
    }

    private fun propTypeMapper(prop: Property): String =
        valueConversion(
            canonicalName = prop.typeClass.canonicalName,
            source = """findWindowSizeClass(properties["${prop.key}"], windowManager)""",
            default = prop.value
        )

    private fun dataTypeMapper(dataItem: Data): String =
        valueConversion(
            canonicalName = dataItem.typeClass.canonicalName,
            source = "result",
            default = dataItem.value
        )
}