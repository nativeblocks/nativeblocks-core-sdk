import Foundation
import SwiftParser
import SwiftSyntax
import _NativeblocksCompilerCommon

public class JsonGenerator {
    public var blocksJson: [String: [JsonMetaType: Data]] = [:]
    public var actionsJson: [String: [JsonMetaType: Data]] = [:]
    public var blocks: [Integration] = []
    public var actions: [Integration] = []

    public init() {}

    public func generate(from files: [String], organizationId: String = "") throws {
        print("Generate jsons...")
        (blocks, actions) = NativeItemVisitor.extractNatives(from: files)

        if blocks.isEmpty, actions.isEmpty {
            print("There is no actions or blocks to generate")
            return
        }

        blocks = blocks.map { block in
            let (meta, _) = BlockExtractor.extractVariable(from: block.syntaxStruct!)
            var copy = block
            copy.meta = meta
            copy.organizationId = organizationId
            return copy
        }

        actions = actions.map { action in
            let (meta, _) = ActionExtractor.extractVariable(from: action.syntaxClass!)
            var copy = action
            copy.meta = meta
            copy.organizationId = organizationId
            return copy
        }

        blocksJson = try Dictionary(
            uniqueKeysWithValues: blocks.map { block in
                print("generate Json \(block.keyType) Block")
                let jsonList = try generateBlock(from: block)
                return (block.keyType, jsonList)
            }
        )

        actionsJson = try Dictionary(
            uniqueKeysWithValues: actions.map { action in
                print("generate Json \(action.keyType) Action")
                let jsonList = try generateAction(from: action)
                return (action.keyType, jsonList)
            }
        )
    }

    public func save(
        to directory: String,
        with fileManager: FileManager,
        prefix: String = "/.nativeblocks"
    ) throws {
        print("Save jsons...")

        if blocksJson.isEmpty, actionsJson.isEmpty {
            print("There is no actions or blocks to save")
            return
        }

        let baseDirectory = "\(directory)\(prefix)"
        try? fileManager.deleteDirectory(atPath: baseDirectory)
        try fileManager.createDirectory(atPath: baseDirectory)

        for block in blocksJson {
            let blockDirectory = "\(baseDirectory)/block/\(block.key)"
            print("save Json \(block.key) Block at:\(blockDirectory)")
            try fileManager.createDirectory(atPath: blockDirectory)
            try saveJsons(jsons: block.value, in: blockDirectory)
        }

        for action in actionsJson {
            let directory = "\(baseDirectory)/action/\(action.key)"
            print("save Json \(action.key) Action at:\(directory)")
            try fileManager.createDirectory(atPath: directory)
            try saveJsons(jsons: action.value, in: directory)
        }
    }

    public func generateBlock(from block: Integration) throws -> [JsonMetaType: Data] {
        var export: [JsonMetaType: Data] = [:]
        export[JsonMetaType.integration] = try JSONEncoder().encode(block)
        let datas = block.meta.compactMap { $0 as? DataMeta }
        let events = block.meta.compactMap { $0 as? EventMeta }
        let properties = block.meta.compactMap { $0 as? PropertyMeta }
        let slots = block.meta.compactMap { $0 as? SlotMeta }
        export[JsonMetaType.data] = try JSONEncoder().encode(datas)
        export[JsonMetaType.event] = try JSONEncoder().encode(events)
        export[JsonMetaType.properties] = try JSONEncoder().encode(properties)
        export[JsonMetaType.slot] = try JSONEncoder().encode(slots)
        return export
    }

    public func generateAction(from action: Integration) throws -> [JsonMetaType: Data] {
        var export: [JsonMetaType: Data] = [:]
        export[JsonMetaType.integration] = try JSONEncoder().encode(action)
        let datas = action.meta.compactMap { $0 as? DataMeta }
        let events = action.meta.compactMap { $0 as? EventMeta }
        let properties = action.meta.compactMap { $0 as? PropertyMeta }
        export[JsonMetaType.data] = try JSONEncoder().encode(datas)
        export[JsonMetaType.event] = try JSONEncoder().encode(events)
        export[JsonMetaType.properties] = try JSONEncoder().encode(properties)
        return export
    }

    private func saveJsons(jsons: [JsonMetaType: Data], in directory: String) throws {
        try jsons.forEach { (type: JsonMetaType, json: Data) in
            print("/\(type.fileName) ==> \(json))")
            try String(data: json, encoding: .utf8)?.write(
                toFile: directory + "/\(type.fileName)", atomically: true, encoding: .utf8)
        }
    }
}

public enum JsonMetaType: CaseIterable {
    case integration
    case data
    case event
    case properties
    case slot

    public var fileName: String {
        switch self {
        case .integration:
            return "integration.json"
        case .data:
            return "data.json"
        case .event:
            return "events.json"
        case .properties:
            return "properties.json"
        case .slot:
            return "slots.json"
        }
    }
}
