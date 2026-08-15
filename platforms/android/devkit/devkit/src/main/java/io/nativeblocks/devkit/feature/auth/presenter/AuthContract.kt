package io.nativeblocks.devkit.feature.auth.presenter

internal object AuthContract {
    data class UIState(
        val hasPermission: Boolean,
        val pasteFromClipboard: Boolean
    ) {
        companion object {
            fun init(): UIState {
                return UIState(
                    hasPermission = false,
                    pasteFromClipboard = false
                )
            }
        }
    }

    sealed interface UIAction {
        data class OnScan(val qrData: String) : UIAction
        data object OnPermissionRequest : UIAction
        data object OnClose : UIAction
        data object CopyFromClipboard : UIAction
        data class OnCopyFromClipboardFinished(val clipboard: String?) : UIAction
    }

    sealed interface UIEffect {
        data class ShowToast(val message: String) : UIEffect
        data object Close : UIEffect
    }
}