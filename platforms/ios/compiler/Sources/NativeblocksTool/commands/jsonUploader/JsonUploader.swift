import Foundation
import _NativeblocksCompilerCommon

public class JsonUploader {
    private let endpoint: String
    private let authToken: String
    private let organizationId: String

    public init(
        endpoint: String,
        authToken: String,
        organizationId: String
    ) {
        self.endpoint = endpoint
        self.authToken = authToken
        self.organizationId = organizationId
        NetworkExecutor.initialize(endpoint: endpoint, apiKey: authToken)
    }

    public func upload(blocks: [Integration], actions: [Integration]) throws {
        for block in blocks {
            print("Sync Block:\(block.keyType) start...")
            var input = block
            input.organizationId = organizationId
            let integrationId = try JsonUploader.syncIntegration(input: input)

            let datas = block.meta.compactMap { $0 as? DataMeta }
            let events = block.meta.compactMap { $0 as? EventMeta }
            let properties = block.meta.compactMap { $0 as? PropertyMeta }
            let slots = block.meta.compactMap { $0 as? SlotMeta }

            print("Sync Data start")
            try JsonUploader.syncIntegrationData(
                integrationId: integrationId,
                organizationId: organizationId,
                meta: datas
            )
            print("Sync Data done")

            print("Sync Properties start")
            try JsonUploader.syncIntegrationProperties(
                integrationId: integrationId,
                organizationId: organizationId,
                meta: properties
            )
            print("Sync Properties done")

            print("Sync Events start")
            try JsonUploader.syncIntegrationEvents(
                integrationId: integrationId,
                organizationId: organizationId,
                meta: events
            )
            print("Sync Events done")

            print("Sync Slots start")
            try JsonUploader.syncIntegrationSlots(
                integrationId: integrationId,
                organizationId: organizationId,
                meta: slots
            )
            print("Sync Slots done")

            print("Sync Block:\(block.keyType) done")
        }

        for action in actions {
            print("Sync Action:\(action.keyType) start...")
            var input = action
            input.organizationId = organizationId
            let integrationId = try JsonUploader.syncIntegration(input: input)

            let datas = action.meta.compactMap { $0 as? DataMeta }
            let events = action.meta.compactMap { $0 as? EventMeta }
            let properties = action.meta.compactMap { $0 as? PropertyMeta }

            print("Sync Data start")
            try JsonUploader.syncIntegrationData(
                integrationId: integrationId,
                organizationId: organizationId,
                meta: datas
            )
            print("Sync Data done")

            print("Sync Properties start")
            try JsonUploader.syncIntegrationProperties(
                integrationId: integrationId,
                organizationId: organizationId,
                meta: properties
            )
            print("Sync Properties done")

            print("Sync Events start")
            try JsonUploader.syncIntegrationEvents(
                integrationId: integrationId,
                organizationId: organizationId,
                meta: events
            )
            print("Sync Events done")

            print("Sync Action:\(action.keyType) done")
        }

        print("Sync done")
    }

    static func syncIntegration(input: Integration) throws -> String {
        let semaphore = DispatchSemaphore(value: 0)
        var result: ResultModel<SyncIntegrationResultRawModel>?
        NetworkExecutor.getInstance().performOperation(
            NetworkOperation(
                JsonUploaderQuery.syncIntegration(),
                operationName: "syncIntegration",
                variables: ["input": AnyEncodable(input)]
            )
        ) { (response: ResultModel<SyncIntegrationResultRawModel>) in
            result = response
            semaphore.signal()
        }
        semaphore.wait()
        switch result! {
        case .success(let raw):
            return raw.syncIntegration?.id ?? ""
        case .error(let errorModel):
            throw errorModel
        }
    }

    static func syncIntegrationData(
        integrationId: String, organizationId: String, meta: [DataMeta]
    ) throws {
        if meta.isEmpty {
            return
        }
        let input = SyncIntegrationDataInput(
            integrationId: integrationId,
            organizationId: organizationId,
            data: meta
        )

        let semaphore = DispatchSemaphore(value: 0)
        var result: ResultModel<EmptyResultModel>?
        NetworkExecutor.getInstance().performOperation(
            NetworkOperation(
                JsonUploaderQuery.syncIntegrationData(),
                operationName: "syncIntegrationData",
                variables: ["input": AnyEncodable(input)]
            )
        ) { (response: ResultModel<EmptyResultModel>) in
            result = response
            semaphore.signal()
        }
        semaphore.wait()
        switch result! {
        case .success(_):
            return
        case .error(let errorModel):
            throw errorModel
        }
    }

    static func syncIntegrationProperties(
        integrationId: String, organizationId: String, meta: [PropertyMeta]
    ) throws {
        if meta.isEmpty {
            return
        }
        let input = SyncIntegrationPropertiesInput(
            integrationId: integrationId,
            organizationId: organizationId,
            properties: meta
        )

        let semaphore = DispatchSemaphore(value: 0)
        var result: ResultModel<EmptyResultModel>?
        NetworkExecutor.getInstance().performOperation(
            NetworkOperation(
                JsonUploaderQuery.syncIntegrationProperties(),
                operationName: "syncIntegrationProperties",
                variables: ["input": AnyEncodable(input)]
            )
        ) { (response: ResultModel<EmptyResultModel>) in
            result = response
            semaphore.signal()
        }
        semaphore.wait()
        switch result! {
        case .success(_):
            return
        case .error(let errorModel):
            throw errorModel
        }
    }

    static func syncIntegrationEvents(
        integrationId: String, organizationId: String, meta: [EventMeta]
    ) throws {
        if meta.isEmpty {
            return
        }
        let input = SyncIntegrationEventsInput(
            integrationId: integrationId,
            organizationId: organizationId,
            events: meta
        )

        let semaphore = DispatchSemaphore(value: 0)
        var result: ResultModel<EmptyResultModel>?
        NetworkExecutor.getInstance().performOperation(
            NetworkOperation(
                JsonUploaderQuery.syncIntegrationEvents(),
                operationName: "syncIntegrationEvents",
                variables: ["input": AnyEncodable(input)]
            )
        ) { (response: ResultModel<EmptyResultModel>) in
            result = response
            semaphore.signal()
        }
        semaphore.wait()
        switch result! {
        case .success(_):
            return
        case .error(let errorModel):
            throw errorModel
        }
    }

    static func syncIntegrationSlots(
        integrationId: String, organizationId: String, meta: [SlotMeta]
    ) throws {
        if meta.isEmpty {
            return
        }
        let input = SyncIntegrationSlotsInput(
            integrationId: integrationId,
            organizationId: organizationId,
            slots: meta
        )

        let semaphore = DispatchSemaphore(value: 0)
        var result: ResultModel<EmptyResultModel>?
        NetworkExecutor.getInstance().performOperation(
            NetworkOperation(
                JsonUploaderQuery.syncIntegrationSlots(),
                operationName: "syncIntegrationSlots",
                variables: ["input": AnyEncodable(input)]
            )
        ) { (response: ResultModel<EmptyResultModel>) in
            result = response
            semaphore.signal()
        }
        semaphore.wait()
        switch result! {
        case .success(_):
            return
        case .error(let errorModel):
            throw errorModel
        }
    }
}
