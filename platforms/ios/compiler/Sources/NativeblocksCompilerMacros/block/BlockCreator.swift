import SwiftSyntax
import SwiftSyntaxBuilder
import _NativeblocksCompilerCommon

struct BlockCreator {
    static func create(
        structName: String,
        metaData: [DataMeta],
        metaProp: [PropertyMeta],
        metaEvent: [EventMeta],
        metaSlot: [SlotMeta],
        metaExtraParams: [ExtraParamMeta]
    ) throws -> StructDeclSyntax {
        return try StructDeclSyntax("public struct \(raw: structName)Block: View") {
            try VariableDeclSyntax("var blockProps: BlockProps")
            try VariableDeclSyntax(
                  """
                  public var body: some View
                  """
            ) {
                """
                Group {
                    if let visibility = blockProps.onFindVariable(blockProps.block.visibility)?.value,
                       visibility == "false" {
                        EmptyView()
                    } else {
                        InternalView(blockProps: blockProps)
                    }
                }
                """
            }
           
            try StructDeclSyntax("private struct InternalView: View") {
                try VariableDeclSyntax("var blockProps: BlockProps")

                try VariableDeclSyntax(
                    """
                    @Environment(\\.nativeWindowWidthClass) var windowManager
                    """
                )

                for data in metaData {
                    try VariableDeclSyntax(
                        """
                        @State private var \(raw: data.key)DataValue = \(raw: dataDefaultMapper(dataItem: data))
                        """
                    )
                }

                try VariableDeclSyntax(
                    """
                    var body: some View
                    """
                ) {
                    if !metaData.isEmpty {
                        """
                        let data = blockProps.block.data
                        """
                    }
                    if !metaProp.isEmpty {
                        """
                        let properties = blockProps.block.properties
                        """
                    }
                    """
                    //Block Data
                    """
                    for data in metaData {
                        """
                        let \(raw: data.key)Data = blockProps.onFindVariable(data["\(raw: data.key)"]?.value ?? "")
                        """
                    }

                    """
                    //Block Properties
                    """
                    for prop in metaProp {
                        """
                        let \(raw: prop.key)Prop = \(raw: propTypeMapper(item: prop) ?? "")
                        """
                    }

                    """
                    //Block Events
                    """
                    for event in metaEvent {
                        """
                        let \(raw: event.event)Event = blockProvideEvent(blockProps: blockProps, eventType: "\(raw: event.event)")
                        """
                    }

                    """
                    //Block Slots
                    """
                    for slot in metaSlot {
                        """
                        let \(raw: slot.slot)Slot = blockProvideSlot(blockProps: blockProps, slotType: "\(raw: slot.slot)")
                        """
                    }

                    let dataArguments = metaData.map {
                        (
                            $0.position,
                            """
                            \($0.key): \($0.key)DataValue
                            """
                        )
                    }

                    let propArguments = metaProp.map {
                        (
                            $0.position,
                            """
                            \($0.key): \($0.key)Prop
                            """
                        )
                    }

                    let eventArguments = metaEvent.map { event in
                        (
                            event.position,
                            """
                            \(event.event):\(event.isOptionalFunction ? "\(event.event)Event == nil ? nil :" : "") { \(event.dataBinding.map { "\($0)Param" }.joined(separator: ",")) \(event.dataBinding.isEmpty ? "" : "in")
                            \(event.dataBinding.map { param in
                                """
                                if var \(param)Updated = \(param)Data {
                                    \(param)Updated.value = String(describing: \(param)Param)
                                    blockProps.onVariableChange(\(param)Updated)
                                }
                                """
                            }.joined())
                            \(event.event)Event?()
                            }
                            """
                        )
                    }
                    let slotArguments = metaSlot.map { slot in
                        (
                            slot.position,
                            """
                            \(slot.slot): \(slot.slot)Slot == nil ? \(slot.isOptionalFunction ? "nil" : "{ \(slot.hasBlockIndex ? "index" : "")\(slot.hasBlockIndex && slot.hasBlockScope ? ", ":"")\(slot.hasBlockScope ? "scope" : "")\((slot.hasBlockIndex || slot.hasBlockScope) ? " in" : "") AnyView(EmptyView())}") : { \(slot.hasBlockIndex ? "index" : "")\(slot.hasBlockIndex && slot.hasBlockScope ? ", ":"")\(slot.hasBlockScope ? "scope" : "")\((slot.hasBlockIndex || slot.hasBlockScope) ? " in" : "")
                                (blockProps.onSubBlock(blockProps.block.subBlocks ?? [:], \(slot.slot)Slot!, \(slot.hasBlockIndex ?"index": "-1"), \(slot.hasBlockScope ?"scope": "nil")))
                            }
                            """
                        )
                    }

                    let extraParamArguments = metaExtraParams.map {
                        (
                            $0.position,
                            """
                            \($0.key): \($0.key)
                            """
                        )
                    }

                    let arguments = (dataArguments + propArguments + eventArguments + slotArguments + extraParamArguments)
                        .sorted { $0.0 < $1.0 }
                        .map { $0.1 }
                        .joined(separator: ",\n")

                    """
                    return \(raw: structName)(\n\(raw: arguments)\n)
                    """
                    for data in metaData {
                        """
                        .task(id: \(raw: data.key)Data) {
                        let result = \(raw: data.key)Data?.value
                        \(raw: data.key)DataValue = \(raw: dataTypeMapper(dataItem: data))
                        }
                        """
                    }
                }
            }
        }
    }

    private static func dataTypeMapper(dataItem: DataMeta) -> String {
        switch dataItem.type.uppercased() {
        case "STRING":
            return
                """
                result ?? "\(dataItem.value)"
                """
        case "INT", "INT64", "INT32", "INT16", "INT8", "UINT", "UINT64", "UINT32", "UINT16", "UINT8",
            "FLOAT", "FLOAT80", "FLOAT64",
            "FLOAT32", "FLOAT16", "DOUBLE":
            return
                """
                \(dataItem.type)(result ?? "") ?? \(dataItem.value.isEmpty ? "0" : dataItem.value)
                """
        case "CGFLOAT":
            return
                """
                (result ?? "").toCGFloat() ?? \(dataItem.value.isEmpty ? "0.0" : dataItem.value)
                """
        case "BOOL":
            return
                """
                Bool(result ?? "") ?? \(dataItem.value.isEmpty ? "false" : dataItem.value)
                """
        default:
            return
                """
                """
        }
    }

    private static func dataDefaultMapper(dataItem: DataMeta) -> String {
        switch dataItem.type.uppercased() {
        case "STRING":
            return
                """
                "\(dataItem.value)"
                """
        case "INT", "INT64", "INT32", "INT16", "INT8", "UINT", "UINT64", "UINT32", "UINT16", "UINT8",
            "FLOAT", "FLOAT80", "FLOAT64",
            "FLOAT32", "FLOAT16", "DOUBLE":
            return
                """
                \(dataItem.value.isEmpty ? "0" : dataItem.value)
                """
        case "CGFLOAT":
            return
                """
                \(dataItem.value.isEmpty ? "0.0" : dataItem.value)
                """
        case "BOOL":
            return
                """
                \(dataItem.value.isEmpty ? "false" : dataItem.value)
                """
        default:
            return
                """
                """
        }
    }
    private static func propTypeMapper(item: PropertyMeta) -> String? {
        switch item.type.uppercased() {
        case "STRING":
            return
                """
                findWindowSizeClass(properties["\(item.key)"], windowManager) ?? "\(item.value)"
                """
        case "INT", "INT64", "INT32", "INT16", "INT8", "UINT", "UINT64", "UINT32", "UINT16", "UINT8",
            "FLOAT", "FLOAT80", "FLOAT64",
            "FLOAT32", "FLOAT16", "DOUBLE":
            return
                """
                \(item.type)(findWindowSizeClass(properties["\(item.key)"], windowManager) ?? "") ?? \(item.value.isEmpty ? "0" : item.value)
                """
        case "CGFLOAT":
            return
                """
                (findWindowSizeClass(properties["\(item.key)"], windowManager) ?? "").toCGFloat() ?? \(item.value.isEmpty ? "0.0" : item.value)
                """
        case "BOOL":
            return
                """
                Bool(findWindowSizeClass(properties["\(item.key)"], windowManager) ?? "") ??  \(item.value.isEmpty ? "false" : item.value)
                """
        default:
            return
                """
                NativeblocksManager.getInstance(name: blockProps.instanceName).getTypeConverter(\(item.type).self).fromString(findWindowSizeClass(properties["\(item.key)"], windowManager) ?? "\(item.value)")
                """
        }
    }
}
