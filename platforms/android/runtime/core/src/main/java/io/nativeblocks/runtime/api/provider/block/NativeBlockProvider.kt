package io.nativeblocks.runtime.api.provider.block

import androidx.compose.runtime.Composable

/**
 * Provides and manages native blocks within the framework.
 */
internal class NativeBlockProvider {

    /**
     * Map of block types to their corresponding implementations.
     */
    private val blocks = mutableMapOf<String, BlockComposable>()

    /**
     * Fallback block implementation to be used when a specific block type is not found.
     */
    private var fallbackBlock: (@Composable (keyType: String, key: String) -> Unit)? = null

    /**
     * Registers a new block implementation for a specific block type.
     * @param blockType The type of the block.
     * @param block The block implementation to register.
     */
    fun provideBlock(blockType: String, block: BlockComposable) {
        blocks[blockType] = block
    }

    /**
     * Retrieves all registered block implementations.
     * @return A map of block types to their implementations.
     */
    fun getProvidedBlocks(): Map<String, BlockComposable> {
        return blocks.toMap()
    }

    /**
     * Registers a fallback block implementation to be used when a specific block type is not found.
     * @param block The fallback block implementation to register.
     */
    fun onFallbackBlock(block: @Composable (keyType: String, key: String) -> Unit) {
        fallbackBlock = block
    }

    /**
     * Retrieves the fallback block implementation.
     * @return The fallback block implementation, or null if none is registered.
     */
    fun getFallbackBlock(): @Composable ((keyType: String, key: String) -> Unit)? {
        return fallbackBlock
    }

}
