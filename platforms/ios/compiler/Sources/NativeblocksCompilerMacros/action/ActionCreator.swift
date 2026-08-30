import SwiftSyntax
import SwiftSyntaxBuilder
import _NativeblocksCompilerCommon

enum ActionCreator {
    static func create(
        structName: String,
        actionInfo: ActionMeta?,
        metaData: [DataMeta],
        metaBindingData: [DataMeta],
        metaProp: [PropertyMeta],
        metaEvent: [EventMeta],
        metaExtraParams: [ExtraParamMeta]
    ) throws -> ClassDeclSyntax {
        return try ClassDeclSyntax("public class \(raw: structName)Action: INativeAction") {
            """
            var action: \(raw: structName)
            """
            """
            init(action: \(raw: structName)) {
                self.action = action
            }
            """
            try FunctionDeclSyntax("public func handle(actionContext: ActionContext)") {
                if actionInfo?.isAsync == true {
                    """
                    Task {

                    """
                }

                if !metaData.isEmpty {
                    """
                    let data = actionContext.trigger?.data ?? [:]
                    """
                }
                if !metaProp.isEmpty {
                    """
                    let properties = actionContext.trigger?.properties ?? [:]
                    """
                }

                """
                //Action trigger Data
                """
                for data in metaData + metaBindingData {
                    """
                    let \(raw: data.key)Data = actionContext.onFindVariable(data["\(raw: data.key)"]?.value ?? "")
                    """
                }
                for data in metaData {
                    """
                    let \(raw: data.key)DataValue = \(raw: dataTypeMapper(dataItem: data) ?? "")
                    """
                }
                """
                //Action trigger properties
                """
                for prop in metaProp {
                    """
                    let \(raw: prop.key)Prop = \(raw: propTypeMapper(item: prop) ?? "")
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
                        \(event.event): { \(event.dataBindings.map { "\($0)Param" }.joined(separator: ",")) \(event.dataBindings.isEmpty ? "" : "in")
                        \(event.dataBindings.map { param in
                            """
                            if var \(param)Updated = \(param)Data {
                                \(param)Updated.value = String(describing: \(param)Param)
                                actionContext.onUpdateVariable(\(param)Updated)
                            }
                            """
                        }.joined())
                        actionContext.onHandleEvent("\(event.event)")
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

                let arguments = (dataArguments + propArguments + eventArguments + extraParamArguments)
                    .sorted { $0.0 < $1.0 }
                    .map { $0.1 }
                    .joined(separator: ",\n")

                if actionInfo?.functionParamName.isEmpty == false && actionInfo?.parameterClass.isEmpty == false {
                    """
                    let param = \(raw: structName).\(raw: actionInfo?.parameterClass ?? "Struct")(\n\(raw: arguments))
                    """
                    """
                    \(raw: (actionInfo?.isAsync == true ? "await " : ""))action.\(raw: actionInfo?.functionName ?? "function")(param: \(raw: actionInfo?.functionParamName ?? "param"))
                    """
                } else {
                    """
                    \(raw: (actionInfo?.isAsync == true ? "await " : ""))action.\(raw: actionInfo?.functionName ?? "function")()
                    """
                }
                if actionInfo?.isAsync == true {
                    """
                    }
                    """
                }
            }
        }
    }

    private static func dataTypeMapper(dataItem: DataMeta) -> String? {
        return TypeUtils.valueConversion(
            type: dataItem.type,
            source: "actionContext.resolveTemplate(\(dataItem.key)Data?.value)",
            defaultValue: dataItem.value,
            instance: "actionContext.instanceName"
        )
    }

    private static func propTypeMapper(item: PropertyMeta) -> String? {
        return TypeUtils.valueConversion(
            type: item.type,
            source: "properties[\"\(item.key)\"]?.value",
            defaultValue: item.value,
            instance: "actionContext.instanceName"
        )
    }
}
