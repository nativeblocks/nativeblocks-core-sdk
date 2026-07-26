import Foundation

enum JsonUploaderQuery {
    static func syncIntegrationData() -> String {
        return
            """
            mutation syncIntegrationData($input: SyncIntegrationDataInput!) {
                syncIntegrationData(input: $input) {
                    key
                    type
                }
            }
            """
    }

    static func syncIntegration() -> String {
        return
            """
            mutation syncIntegration($input: SyncIntegrationInput!) {
                syncIntegration(input: $input) {
                    id
                    keyType
                    name
                    imageIcon
                    price
                    description
                    kind
                    documentation
                }
            }
            """
    }

    static func syncIntegrationEvents() -> String {
        return
            """
            mutation syncIntegrationEvents($input: SyncIntegrationEventsInput!) {
                syncIntegrationEvents(input: $input) {
                    event
                }
            }
            """
    }

    static func syncIntegrationProperties() -> String {
        return
            """
            mutation syncIntegrationProperties($input: SyncIntegrationPropertiesInput!) {
                syncIntegrationProperties(input: $input) {
                    key
                    type
                    value
                    description
                    valuePicker
                    valuePickerGroup
                    valuePickerOptions
                }
            }
            """
    }

    static func syncIntegrationSlots() -> String {
        return
            """
            mutation syncIntegrationSlots($input: SyncIntegrationSlotsInput!) {
                syncIntegrationSlots(input: $input) {
                    slot
                }
            }
            """
    }
}
