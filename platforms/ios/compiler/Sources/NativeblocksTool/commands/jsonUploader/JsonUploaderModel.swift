import Foundation
import _NativeblocksCompilerCommon

struct SyncIntegrationResultModel: Codable {
    var id: String
    var name: String
}

struct SyncIntegrationResultRawModel: Codable {
    var syncIntegration: SyncIntegrationResultModel?
}

struct SyncIntegrationPropertiesInput: Encodable {
    var integrationId: String
    var organizationId: String
    var properties: [PropertyMeta]
}

struct SyncIntegrationEventsInput: Encodable {
    var integrationId: String
    var organizationId: String
    var events: [EventMeta]
}

struct SyncIntegrationDataInput: Encodable {
    var integrationId: String
    var organizationId: String
    var data: [DataMeta]
}

struct SyncIntegrationSlotsInput: Encodable {
    var integrationId: String
    var organizationId: String
    var slots: [SlotMeta]
}

struct EmptyResultModel: Codable {}
