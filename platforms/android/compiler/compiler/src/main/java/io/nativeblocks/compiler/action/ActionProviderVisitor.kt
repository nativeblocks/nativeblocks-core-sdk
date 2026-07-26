package io.nativeblocks.compiler.action

import com.google.devtools.ksp.symbol.KSVisitorVoid
import com.squareup.kotlinpoet.ClassName
import com.squareup.kotlinpoet.FileSpec
import com.squareup.kotlinpoet.FunSpec
import com.squareup.kotlinpoet.ParameterSpec
import com.squareup.kotlinpoet.TypeSpec
import io.nativeblocks.compiler.util.camelcase
import io.nativeblocks.compiler.util.plusAssign
import java.io.OutputStream

internal class ActionProviderVisitor(
    private val file: OutputStream,
    fileName: String,
    packageName: String,
    integrations: MutableList<ActionFunctionModel>,
) : KSVisitorVoid() {

    private val importNativeblocksManager =
        ClassName("io.nativeblocks.runtime.api", "NativeblocksManager")

    init {
        val func = FunSpec.builder("provideActions")
        func.addParameter(
            ParameterSpec.builder("instanceName", String::class)
                .defaultValue("%S", "default")
                .build()
        )
        integrations.forEach {
            func.addParameter(
                ParameterSpec(it.className.camelcase(), ClassName(it.packageName, it.className))
            )
        }

        func.addStatement("NativeblocksManager.getInstance(instanceName)")
        integrations.forEach {
            func.addCode(
                """
                |.provideAction(
                |   actionType = "${it.keyType}",
                |   action = ${it.className}Action(${it.className.camelcase()})
                |)
                """.trimMargin()
            )
        }

        val actionClass = FileSpec.builder(packageName, fileName)
            .addImport(importNativeblocksManager, "")

        integrations.forEach {
            actionClass.addImport(ClassName(it.packageName, it.className), "")
        }

        actionClass.addType(
            TypeSpec.objectBuilder(fileName)
                .addFunction(func.build())
                .build()
        )
        file += actionClass.build().toString()
    }

}