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
        let describing = metaExtraParams.contains { $0.key == describeScopeParam }
        return try StructDeclSyntax("public struct \(raw: structName)Block: View") {
            try VariableDeclSyntax("var blockContext: BlockContext")
            if describing {
                try VariableDeclSyntax("var describeScope: Any")
            }
            try VariableDeclSyntax(
                """
                @Environment(\\.nativeWindowWidthClass) var windowManager
                """
            )

            for data in metaData where SyntaxUtils.isPrimitiveTypeSupported(data.type) {
                try VariableDeclSyntax(
                    """
                    @State private var \(raw: data.key)DataValue: \(raw: data.type) = \(raw: dataDefaultMapper(dataItem: data))
                    """
                )
            }

            try VariableDeclSyntax(
                """
                public var body: some View
                """
            ) {
                if !metaData.isEmpty {
                    """
                    let data = blockContext.block.data
                    """
                }
                if !metaProp.isEmpty {
                    """
                    let properties = blockContext.block.properties
                    """
                }
                """
                //Block Data
                """
                for data in metaData {
                    """
                    let \(raw: data.key)Data = blockContext.onFindVariable(data["\(raw: data.key)"])
                    """
                    if !SyntaxUtils.isPrimitiveTypeSupported(data.type) {
                        """
                        let \(raw: data.key)DataValue = \(raw: TypeUtils.valueConversion(type: data.type, source: "\(data.key)Data", defaultValue: data.value, instance: "blockContext.instanceName"))
                        """
                    }
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
                    let \(raw: event.event)Event = blockProvideEvent(blockContext: blockContext, eventType: "\(raw: event.event)")
                    """
                }

                """
                //Block Slots
                """
                for slot in metaSlot {
                    """
                    let \(raw: slot.slot)Slot = blockProvideSlot(blockContext: blockContext, slotType: "\(raw: slot.slot)")
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
                        \(event.event):\(event.isOptionalFunction ? "\(event.event)Event == nil ? nil :" : "") { \(event.dataBindings.map { "\($0)Param" }.joined(separator: ",")) \(event.dataBindings.isEmpty ? "" : "in")
                        \(event.dataBindings.map { param in
                            """
                            blockContext.onUpdateVariable(data["\(param)"], String(describing: \(param)Param))
                            """
                        }.joined())
                        \(event.event)Event?()
                        }
                        """
                    )
                }
                let slotArguments = metaSlot.map { slot -> (Int, String) in
                    if slot.describing {
                        return (
                            slot.position,
                            """
                            \(slot.slot): \(slot.slot)Slot == nil ? { _ in AnyView(EmptyView()) } : { describeScope in
                                (blockContext.onDescribeSubBlock(blockContext.block.subBlocks ?? [:], \(slot.slot)Slot!, describeScope))
                            }
                            """
                        )
                    }
                    return (
                        slot.position,
                        """
                        \(slot.slot): \(slot.slot)Slot == nil ? \(slot.isOptionalFunction ? "nil" : "{ \(slot.hasBlockIndex ? "index" : "")\(slot.hasBlockIndex && slot.hasBlockScope ? ", ":"")\(slot.hasBlockScope ? "scope" : "")\((slot.hasBlockIndex || slot.hasBlockScope) ? " in" : "") AnyView(EmptyView())}") : { \(slot.hasBlockIndex ? "index" : "")\(slot.hasBlockIndex && slot.hasBlockScope ? ", ":"")\(slot.hasBlockScope ? "scope" : "")\((slot.hasBlockIndex || slot.hasBlockScope) ? " in" : "")
                            (blockContext.onSubBlock(blockContext.block.subBlocks ?? [:], \(slot.slot)Slot!, \(slot.hasBlockIndex ?"index": "-1"), \(slot.hasBlockScope ? "scope" : "nil")))
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
                for data in metaData where SyntaxUtils.isPrimitiveTypeSupported(data.type) {
                    """
                    .task(id: \(raw: data.key)Data) {
                    let result = \(raw: data.key)Data
                    \(raw: data.key)DataValue = \(raw: dataTypeMapper(dataItem: data))
                    }
                    """
                }
            }
        }
    }

    private static func dataTypeMapper(dataItem: DataMeta) -> String {
        return TypeUtils.valueConversion(
            type: dataItem.type,
            source: "result",
            defaultValue: dataItem.value,
            instance: "blockContext.instanceName"
        )
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
        return TypeUtils.valueConversion(
            type: item.type,
            source: "findWindowSizeClass(properties[\"\(item.key)\"], windowManager)",
            defaultValue: item.value,
            instance: "blockContext.instanceName"
        )
    }
}
