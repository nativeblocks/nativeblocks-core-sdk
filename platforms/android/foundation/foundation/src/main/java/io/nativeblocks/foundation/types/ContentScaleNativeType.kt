package io.nativeblocks.foundation.types

import androidx.compose.ui.layout.ContentScale
import io.nativeblocks.runtime.api.provider.type.INativeType

/**
 * A class that provides serialization and deserialization methods for [ContentScale] objects.
 * It implements the [INativeType] interface for [ContentScale].
 *
 * Example usage:
 * ```kotlin
 * NativeblocksManager.getInstance()
 *     .provideTypeConverter(ContentScale::class, ContentScaleNativeType())
 * ```
 */
class ContentScaleNativeType : INativeType<ContentScale> {

    /**
     * Converts a [ContentScale] object to its string representation.
     *
     * @param input The [ContentScale] object to be converted to a string.
     * @return A string representation of the [ContentScale], or an empty string if the input is null or not recognized.
     */
    override fun toString(input: ContentScale?): String {
        return when (input) {
            ContentScale.None -> "none"
            ContentScale.Crop -> "crop"
            ContentScale.Inside -> "inside"
            ContentScale.Fit -> "fit"
            ContentScale.FillBounds -> "fillBounds"
            ContentScale.FillWidth -> "fillWidth"
            ContentScale.FillHeight -> "fillHeight"
            else -> ""
        }
    }

    /**
     * Converts a string to a [ContentScale] object.
     *
     * @param input The string representation of a [ContentScale].
     * @return The corresponding [ContentScale] object, or [ContentScale.None] if the input is null or not recognized.
     */
    override fun fromString(input: String?): ContentScale {
        return when (input) {
            "none" -> ContentScale.None
            "crop" -> ContentScale.Crop
            "inside" -> ContentScale.Inside
            "fit" -> ContentScale.Fit
            "fillBounds" -> ContentScale.FillBounds
            "fillWidth" -> ContentScale.FillWidth
            "fillHeight" -> ContentScale.FillHeight
            else -> ContentScale.None
        }
    }
}