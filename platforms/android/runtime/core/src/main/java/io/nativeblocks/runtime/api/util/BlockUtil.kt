package io.nativeblocks.runtime.api.util

import android.app.Activity
import android.content.Context
import android.content.ContextWrapper
import androidx.compose.material3.windowsizeclass.ExperimentalMaterial3WindowSizeClassApi
import androidx.compose.material3.windowsizeclass.WindowWidthSizeClass
import androidx.compose.material3.windowsizeclass.calculateWindowSizeClass
import androidx.compose.runtime.Composable
import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.platform.LocalContext
import io.nativeblocks.runtime.api.provider.block.BlockContext
import io.nativeblocks.runtime.api.provider.model.NativeBlockPropertyModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockSlotModel

/**
 * Finds the Activity instance from the given Context.
 *
 * @return The Activity instance if found, or null if not.
 */
fun Context.findActivity(): Activity? {
    var context = this
    while (context is ContextWrapper) {
        if (context is Activity) return context
        context = context.baseContext
    }
    return null
}

enum class NativeDeviceWidth { MOBILE, TABLET, DESKTOP }

val LocalNativeWindowWidthClass = staticCompositionLocalOf { NativeDeviceWidth.MOBILE }

@OptIn(ExperimentalMaterial3WindowSizeClassApi::class)
@Composable
internal fun currentWindowWidthClass(): NativeDeviceWidth {
    val activity = LocalContext.current.findActivity() ?: return NativeDeviceWidth.MOBILE
    return when (calculateWindowSizeClass(activity).widthSizeClass) {
        WindowWidthSizeClass.Medium -> NativeDeviceWidth.TABLET
        WindowWidthSizeClass.Expanded -> NativeDeviceWidth.DESKTOP
        else -> NativeDeviceWidth.MOBILE
    }
}

/**
 * Non-composable resolver: picks the device-specific value for the given [windowManager]. A block
 * reads `val windowManager = LocalNativeWindowWidthClass.current` **once** and passes it here for
 * every property, instead of doing a composition-local read per property.
 *
 * @param prop A NativeBlockPropertyModel containing size-related values for different device types.
 * @param windowManager The device width for the current frame.
 * @return The appropriate size value, or `null` when [prop] is null.
 */
fun findWindowSizeClass(prop: NativeBlockPropertyModel?, windowManager: NativeDeviceWidth): String? {
    prop ?: return null
    return when (windowManager) {
        NativeDeviceWidth.TABLET -> prop.valueTablet
        NativeDeviceWidth.DESKTOP -> prop.valueDesktop
        NativeDeviceWidth.MOBILE -> prop.valueMobile
    }
}

/**
 * Provides the [NativeBlockSlotModel] for the specified slot type if the block supports it.
 *
 * @param blockContext The properties of the block, including its sub-blocks.
 * @param slotType The type of slot to check and provide.
 * @return The [NativeBlockSlotModel] for the specified slot type if the block supports it, otherwise `null`.
 */
fun blockProvideSlot(
    blockContext: BlockContext,
    slotType: String,
): NativeBlockSlotModel? {
    val block = blockContext.block
    return if (block.subBlocks.orEmpty().containsKey(slotType)) {
        blockContext.block.slots[slotType]
    } else {
        null
    }
}