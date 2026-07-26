package io.nativeblocks.compiler.util

object Diagnostic {

    fun exceptionDispatcher(type: DiagnosticType): Exception {
        when (type) {
            is DiagnosticType.KspArgNotFound -> {
                return IllegalArgumentException("Please provide basePackageName and moduleName through the ksp compiler option, https://kotlinlang.org/docs/ksp-quickstart.html#pass-options-to-processors")
            }

            is DiagnosticType.IntegrationKeyTypeConvention -> {
                return IllegalArgumentException("Integration keyType ${(type.keyType)} must contain only letters, underscores, or slashes")
            }

            is DiagnosticType.IntegrationKeyTypeUniqueness -> {
                return IllegalArgumentException("The ${type.keyType} has been used before, please use an unique keyType for each integration")
            }

            is DiagnosticType.MetaCustomType -> {
                return IllegalArgumentException("Custom type (${type.type}) is not supported, please use primitive type for (${type.key})")
            }

            is DiagnosticType.MetaCustomPicker -> {
                return IllegalArgumentException("Custom picker is not supported, please use supported one for (${type.key}) ${type.filePath}")
            }

            is DiagnosticType.ConflictAnnotation -> {
                return IllegalArgumentException("You can not use all annotations at the same time, please use one")
            }

            is DiagnosticType.RequireFunctionAnnotation -> {
                return IllegalArgumentException("Requires only one '@NativeActionFunction' annotated function")
            }

            is DiagnosticType.RequireFunctionParameterAnnotation -> {
                return IllegalArgumentException("'@NativeActionFunction' Requires one parameter data class annotated with '@NativeActionParameter'")
            }

            is DiagnosticType.ThenUniqueness -> {
                return IllegalArgumentException("NativeActionEvent supports only NEXT, FAILURE, SUCCESS, and END events, and each must be used exactly once without repetition")
            }

            is DiagnosticType.ThenConflict -> {
                return IllegalArgumentException("NativeActionEvent containing NEXT or END cannot also contain SUCCESS or FAILURE")
            }

            is DiagnosticType.SlotMustBeComposable -> {
                return IllegalArgumentException("Slot should be a composable function")
            }

            is DiagnosticType.SlotComposableIndex -> {
                return IllegalArgumentException("Slot function has to use BlockIndex type, Please make sure the function defined like: @Composable (index: BlockIndex) -> Unit")
            }
        }
    }
}

sealed interface DiagnosticType {
    data object KspArgNotFound : DiagnosticType
    data class IntegrationKeyTypeConvention(val keyType: String) : DiagnosticType
    data class IntegrationKeyTypeUniqueness(val keyType: String) : DiagnosticType
    data class MetaCustomType(val key: String, val type: String) : DiagnosticType
    data class MetaCustomPicker(val filePath: String, val key: String) : DiagnosticType
    data object ConflictAnnotation : DiagnosticType
    data object RequireFunctionAnnotation : DiagnosticType
    data object RequireFunctionParameterAnnotation : DiagnosticType
    data object ThenUniqueness : DiagnosticType
    data object ThenConflict : DiagnosticType
    data object SlotMustBeComposable : DiagnosticType
    data object SlotComposableIndex : DiagnosticType
}
