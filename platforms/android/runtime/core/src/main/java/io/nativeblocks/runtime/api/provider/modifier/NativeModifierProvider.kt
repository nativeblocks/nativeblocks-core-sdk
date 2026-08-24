package io.nativeblocks.runtime.api.provider.modifier

import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier

internal typealias ModifierComposable = @Composable (modifierContext: ModifierContext) -> Modifier

/**
 * Provides and manages native modifiers within the framework.
 */
internal class NativeModifierProvider {

    private val modifiers = mutableMapOf<String, ModifierComposable>()

    /**
     * Registers a new modifier implementation for a specific modifier type.
     */
    fun provideModifier(modifierType: String, modifier: ModifierComposable) {
        modifiers[modifierType] = modifier
    }

    /**
     * Retrieves all registered modifier implementations.
     */
    fun getProvidedModifiers(): Map<String, ModifierComposable> {
        return modifiers.toMap()
    }
}
