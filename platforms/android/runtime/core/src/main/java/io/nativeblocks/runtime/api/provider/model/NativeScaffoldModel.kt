package io.nativeblocks.runtime.api.provider.model

/**
 * Represents the scaffold structure in the native UI framework, consisting of a list of frame routes.
 */
data class NativeScaffoldModel(
    /**
     * List of frame routes included in the scaffold.
     */
    val frames: List<NativeFrameRouteModel>
)

/**
 * Represents a single frame route, including its metadata and route-specific details.
 */
data class NativeFrameRouteModel(
    /**
     * Unique identifier of the frame route.
     */
    val id: String? = null,

    /**
     * Name of the frame route.
     */
    val name: String? = null,

    /**
     * Type of the frame (e.g., FRAME, BOTTOM_SHEET, DIALOG).
     */
    val type: FrameTypeModel? = null,

    /**
     * Route path associated with the frame.
     */
    val route: String? = null,

    /**
     * platform of the frame.
     */
    val platform: String? = null,

    /**
     * List of arguments for the route.
     */
    val routeArguments: List<NativeRouteArgumentsModel>? = null,
)

/**
 * Represents an argument for a route in the native UI framework.
 */
data class NativeRouteArgumentsModel(
    /**
     * Name of the route argument.
     */
    val name: String? = null
)

/**
 * Enum representing the types of frames supported in the native UI framework.
 */
enum class FrameTypeModel(val type: String) {
    /** Standard frame type. */
    FRAME("FRAME"),

    /** Bottom sheet frame type. */
    BOTTOM_SHEET("BOTTOM_SHEET"),

    /** Dialog frame type. */
    DIALOG("DIALOG");

    companion object {
        /**
         * Converts a string representation to a corresponding enum value.
         * @param type The string representation of the frame type.
         * @return The matching enum value or FRAME if no match is found.
         */
        fun fromString(type: String) = entries.find { it.type == type } ?: FRAME
    }
}
