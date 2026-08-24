package io.nativeblocks.compiler.modifier

import com.google.devtools.ksp.symbol.KSVisitorVoid
import com.squareup.kotlinpoet.ClassName
import com.squareup.kotlinpoet.FileSpec
import com.squareup.kotlinpoet.FunSpec
import com.squareup.kotlinpoet.ParameterSpec
import com.squareup.kotlinpoet.TypeSpec
import io.nativeblocks.compiler.util.plusAssign
import java.io.OutputStream

internal class ModifierProviderVisitor(
    private val file: OutputStream,
    fileName: String,
    packageName: String,
    integrations: MutableList<ModifierFunctionModel>,
) : KSVisitorVoid() {

    private val importNativeblocksManager =
        ClassName("io.nativeblocks.runtime.api", "NativeblocksManager")

    init {
        val func = FunSpec.builder("provideModifiers")
            .addParameter(
                ParameterSpec.builder("instanceName", String::class)
                    .defaultValue("%S", "default")
                    .build()
            )
            .addStatement("NativeblocksManager.getInstance(instanceName)")
        integrations.forEach {
            func.addCode(
                """
                |.provideModifier(
                |   modifierType = "${it.keyType}",
                |   modifier = { modifierContext -> ${it.functionName}Modifier(modifierContext) }
                |)
                """.trimMargin()
            )
        }

        val modifierClass = FileSpec.builder(packageName, fileName)
            .addImport(importNativeblocksManager, "")
            .addType(
                TypeSpec.objectBuilder(fileName)
                    .addFunction(func.build())
                    .build()
            ).build()
        file += modifierClass.toString()
    }
}
