package io.nativeblocks.runtime.api

/**
 * Represents the different editions of the Nativeblocks framework.
 */
sealed interface NativeblocksEdition {

    /**
     * Cloud-based configuration for the Nativeblocks framework.
     * @param endpoint The API endpoint for fetching frame definitions and other resources.
     * @param apiKey The API key for authenticating requests to the server.
     * @param developmentMode Indicates if development mode is enabled for testing and debugging purposes.
     */
    data class Cloud(
        val endpoint: String,
        val apiKey: String,
        val developmentMode: Boolean
    ) : NativeblocksEdition

    /**
     * Community-based configuration for the Nativeblocks framework.
     * @param framesData A map where each key represents a route, and each value is the URL or path to the corresponding frame data.
     */
    class Community(
        val framesData: Map<String, String>,
    ) : NativeblocksEdition
}
