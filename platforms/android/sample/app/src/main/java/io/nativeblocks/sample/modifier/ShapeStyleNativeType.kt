package io.nativeblocks.sample.modifier

import io.nativeblocks.runtime.api.provider.type.INativeType

/**
 * The outline a shape modifier draws.
 */
enum class ShapeStyle {
    Rectangle,
    RoundedRectangle,
    Circle,
    Capsule,
}

/**
 * A class that implements [INativeType] to handle [ShapeStyle] conversion to and from strings.
 * This class is used to represent the [ShapeStyle] type in a string format, such as "rectangle",
 * "roundedRectangle", "circle", or "capsule".
 *
 * Example usage:
 * ```
 * NativeblocksManager.getInstance()
 *     .provideTypeConverter(ShapeStyle::class, ShapeStyleNativeType())
 * ```
 */
class ShapeStyleNativeType : INativeType<ShapeStyle> {

    /**
     * Converts the given [ShapeStyle] to a string representation.
     *
     * @param input The [ShapeStyle] value to convert.
     * @return A string representation: "rectangle", "roundedRectangle", "circle", or "capsule".
     */
    override fun toString(input: ShapeStyle?): String {
        return when (input) {
            ShapeStyle.RoundedRectangle -> "roundedRectangle"
            ShapeStyle.Circle -> "circle"
            ShapeStyle.Capsule -> "capsule"
            else -> "rectangle"
        }
    }

    /**
     * Converts the given string to the corresponding [ShapeStyle].
     *
     * @param input The string to convert.
     * @return The corresponding [ShapeStyle] value, defaulting to [ShapeStyle.Rectangle].
     */
    override fun fromString(input: String?): ShapeStyle {
        return when (input) {
            "roundedRectangle" -> ShapeStyle.RoundedRectangle
            "circle" -> ShapeStyle.Circle
            "capsule" -> ShapeStyle.Capsule
            else -> ShapeStyle.Rectangle
        }
    }
}
