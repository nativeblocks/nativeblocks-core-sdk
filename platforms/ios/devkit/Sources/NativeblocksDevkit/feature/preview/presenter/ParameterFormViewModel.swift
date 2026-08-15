import Foundation
import Combine
import NativeblocksRuntime

internal final class ParameterFormViewModel: ObservableObject {

    @Published private(set) var uiState: ParameterFormContract.UIState = .initial()

    let uiEffect = PassthroughSubject<ParameterFormContract.UIEffect, Never>()

    private let parameterRepository: ParameterRepository
    private let instanceName: String

    init(instanceName: String) {
        self.instanceName = instanceName
        self.parameterRepository = ParameterRepositoryImpl(instanceName: instanceName)
        loadParameters()
    }

    func onAction(_ action: ParameterFormContract.UIAction) {
        switch action {
        case .loadParameters:
            loadParameters()

        case .startAddParameter:
            uiState.isAddingNew = true
            uiState.newKey = ""
            uiState.newValue = ""

        case .updateNewKey(let key):
            uiState.newKey = key

        case .updateNewValue(let value):
            uiState.newValue = value

        case .confirmAddParameter:
            if !uiState.newKey.trimmingCharacters(in: .whitespaces).isEmpty {
                parameterRepository.add(key: uiState.newKey, value: uiState.newValue)
                uiState.isAddingNew = false
                uiState.newKey = ""
                uiState.newValue = ""
                uiState.parameters = parameterRepository.getAll()
            } else {
                uiEffect.send(.showToast("Key cannot be empty"))
            }

        case .cancelAddParameter:
            uiState.isAddingNew = false
            uiState.newKey = ""
            uiState.newValue = ""

        case .startEditParameter(let parameter):
            uiState.editingParameter = parameter

        case .updateEditKey(let key):
            if var editing = uiState.editingParameter {
                editing = PreviewParameter(id: editing.id, name: key, value: editing.value)
                uiState.editingParameter = editing
            }

        case .updateEditValue(let value):
            if var editing = uiState.editingParameter {
                editing = PreviewParameter(id: editing.id, name: editing.name, value: value)
                uiState.editingParameter = editing
            }

        case .confirmEditParameter:
            if let editing = uiState.editingParameter,
               !editing.name.trimmingCharacters(in: .whitespaces).isEmpty {
                parameterRepository.update(id: editing.id, key: editing.name, value: editing.value)
                uiState.editingParameter = nil
                uiState.parameters = parameterRepository.getAll()
            }

        case .cancelEditParameter:
            uiState.editingParameter = nil

        case .deleteParameter(let id):
            parameterRepository.delete(id: id)
            uiState.parameters = parameterRepository.getAll()

        case .applyParameters:
            let params = parameterRepository.toMap()
            NativeblocksManager.getInstance(name: instanceName).setGlobalParameters(params)
            uiEffect.send(.showToast("Parameters applied"))
            uiEffect.send(.parametersApplied)

        case .clearAll:
            parameterRepository.clear()
            NativeblocksManager.getInstance(name: instanceName).setGlobalParameters([:])
            uiState.parameters = []
            uiEffect.send(.showToast("All parameters cleared"))

        case .close:
            uiEffect.send(.dismissed)
        }
    }

    private func loadParameters() {
        uiState.parameters = parameterRepository.getAll()
    }
}
