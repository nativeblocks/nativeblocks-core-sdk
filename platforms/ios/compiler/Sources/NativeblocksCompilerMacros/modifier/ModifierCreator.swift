import SwiftSyntax
import SwiftSyntaxBuilder
import _NativeblocksCompilerCommon

struct ModifierCreator {
    static func create(
        structName: String,
        metaData: [DataMeta],
        metaEvent: [EventMeta],
        metaExtraParams: [ExtraParamMeta]
    ) throws -> StructDeclSyntax {
        return try StructDeclSyntax("public struct \(raw: structName)Modifier: ViewModifier") {
            try VariableDeclSyntax("var modifierContext: ModifierContext")

            for data in metaData where SyntaxUtils.isPrimitiveTypeSupported(data.type) {
                try VariableDeclSyntax(
                    """
                    @State private var \(raw: data.key)DataValue: \(raw: data.type) = \(raw: dataDefaultMapper(dataItem: data))
                    """
                )
            }

            try FunctionDeclSyntax("public func body(content: Content) -> some View") {
                if !metaData.isEmpty {
                    """
                    let data = modifierContext.modifier.data
                    """
                }
                """
                //Modifier Data
                """
                for data in metaData {
                    """
                    let \(raw: data.key)Data = modifierContext.resolveTemplate(modifierContext.onFindVariable(data["\(raw: data.key)"]))
                    """
                    if !SyntaxUtils.isPrimitiveTypeSupported(data.type) {
                        """
                        let \(raw: data.key)DataValue = \(raw: TypeUtils.valueConversion(type: data.type, source: "\(data.key)Data", defaultValue: data.value, instance: "modifierContext.instanceName"))
                        """
                    }
                }

                """
                //Modifier Events
                """
                for event in metaEvent {
                    """
                    let \(raw: event.event)Event = modifierProvideEvent(modifierContext: modifierContext, eventType: "\(raw: event.event)")
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

                let eventArguments = metaEvent.map { event in
                    (
                        event.position,
                        """
                        \(event.event):\(event.isOptionalFunction ? "\(event.event)Event == nil ? nil :" : "") { \(event.dataBindings.map { "\($0)Param" }.joined(separator: ",")) \(event.dataBindings.isEmpty ? "" : "in")
                        \(event.dataBindings.map { param in
                            """
                            modifierContext.onUpdateVariable(data["\(param)"], String(describing: \(param)Param))
                            """
                        }.joined())
                        \(event.event)Event?()
                        }
                        """
                    )
                }

                let extraParamArguments = metaExtraParams.map {
                    (
                        $0.position,
                        """
                        \($0.key): modifierContext
                        """
                    )
                }

                let arguments = (dataArguments + eventArguments + extraParamArguments)
                    .sorted { $0.0 < $1.0 }
                    .map { $0.1 }
                    .joined(separator: ",\n")

                """
                return content.modifier(\(raw: structName)(\n\(raw: arguments)\n))
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
            instance: "modifierContext.instanceName"
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
}
