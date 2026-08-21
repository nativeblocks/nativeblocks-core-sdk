import Foundation
import SwiftUI

/// A singleton class responsible for managing and providing blocks within the Nativeblocks SDK.
/// The `NativeBlockProvider` class maintains a registry of blocks that can be dynamically provided based on the block type.
public class NativeBlockProvider {
    /// A dictionary to store the blocks, where the key is the block type and the value is the corresponding block implementation.
    private var blocks = [String: ((BlockContext) -> any View)]()

    /// Fallback block implementation to be used when a specific block type is not found
    private var fallbackBlock: ((String, String) -> any View)? = nil

    /// Registers a new block with the specified block key type.
    /// - Parameters:
    ///   - blockType: The key type for the block to be registered.
    ///   - block: The block instance that implements the `INativeBlock` protocol.
    func provideBlock(blockType: String, block: @escaping ((BlockContext) -> any View)) {
        blocks[blockType] = block
    }

    /// Sets the fallback block to be used when no matching block is registered or available.
    /// This block will be displayed in place of unsupported or unrecognized blocks.
    ///
    /// - Parameter block: An instance conforming to the `any INativeFallbackBlock` protocol to be used as the fallback.
    func onFallbackBlock(block: @escaping (String, String) -> any View) {
        fallbackBlock = block
    }

    /// Retrieves the dictionary of all registered blocks.
    /// - Returns: A dictionary containing all registered blocks where the key is the block type and the value is the block instance.
    func getProvidedBlocks() -> [String: ((BlockContext) -> any View)] {
        return blocks
    }

    /// Retrieves the fallback block.
    /// - Returns: Registered block
    func getFallbackBlock() -> ((String, String) -> any View)? {
        return fallbackBlock
    }
}
