import Foundation

internal enum ParameterFormContract {

    struct UIState {
        var parameters: [PreviewParameter]
        var editingParameter: PreviewParameter?
        var isAddingNew: Bool
        var newKey: String
        var newValue: String

        static func initial() -> UIState {
            UIState(
                parameters: [],
                editingParameter: nil,
                isAddingNew: false,
                newKey: "",
                newValue: ""
            )
        }
    }

    enum UIAction {
        case loadParameters
        case startAddParameter
        case updateNewKey(String)
        case updateNewValue(String)
        case confirmAddParameter
        case cancelAddParameter
        case startEditParameter(PreviewParameter)
        case updateEditKey(String)
        case updateEditValue(String)
        case confirmEditParameter
        case cancelEditParameter
        case deleteParameter(String)
        case applyParameters
        case clearAll
        case close
    }

    enum UIEffect {
        case parametersApplied
        case dismissed
        case showToast(String)
    }
}
