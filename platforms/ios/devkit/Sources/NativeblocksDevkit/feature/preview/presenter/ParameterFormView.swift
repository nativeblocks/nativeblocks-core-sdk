import SwiftUI
import Combine

internal struct ParameterFormView: View {

    let instanceName: String
    let onDismiss: () -> Void

    @StateObject private var viewModel: ParameterFormViewModel
    @State private var toastMessage: String?
    @State private var showToast: Bool = false

    init(instanceName: String, onDismiss: @escaping () -> Void) {
        self.instanceName = instanceName
        self.onDismiss = onDismiss
        _viewModel = StateObject(wrappedValue: ParameterFormViewModel(instanceName: instanceName))
    }

    var body: some View {
        ZStack {
            VStack(spacing: 0) {
                headerView
                Divider().padding(.vertical, 12)
                contentView
                Divider().padding(.vertical, 12)
                footerView
            }
            .padding(16)
            #if os(iOS)
            .background(Color(.systemBackground))
            #else
            .background(Color(nsColor: .windowBackgroundColor))
            #endif
            .cornerRadius(24)
            .shadow(color: Color.black.opacity(0.1), radius: 10, x: 0, y: 4)

            if showToast, let message = toastMessage {
                VStack {
                    Spacer()
                    Text(message)
                        .font(.system(size: 14))
                        .foregroundColor(.white)
                        .padding(.horizontal, 16)
                        .padding(.vertical, 10)
                        .background(Color.black.opacity(0.8))
                        .cornerRadius(8)
                        .padding(.bottom, 100)
                }
                .transition(.opacity)
                .animation(.easeInOut(duration: 0.3), value: showToast)
            }
        }
        .onReceive(viewModel.uiEffect) { effect in
            handleEffect(effect)
        }
    }

    private var headerView: some View {
        HStack {
            Text("Preview Parameters")
                .font(.system(size: 18, weight: .semibold))
                .foregroundColor(.primary)

            Spacer()

            Button(action: { viewModel.onAction(.close) }) {
                Image(systemName: "xmark")
                    .font(.system(size: 16, weight: .medium))
                    .foregroundColor(.secondary)
                    .frame(width: 32, height: 32)
            }
        }
    }

    private var contentView: some View {
        Group {
            if viewModel.uiState.parameters.isEmpty && !viewModel.uiState.isAddingNew {
                emptyStateView
            } else {
                parameterListView
            }
        }
        .frame(maxHeight: .infinity)
    }

    private var emptyStateView: some View {
        VStack {
            Spacer()
            Text("No parameters yet. Tap \"Add\" to create one.")
                .font(.system(size: 14))
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
            Spacer()
        }
        .frame(maxWidth: .infinity)
        .padding(.vertical, 32)
    }

    private var parameterListView: some View {
        ScrollView {
            LazyVStack(spacing: 8) {
                ForEach(viewModel.uiState.parameters) { param in
                    ParameterRowView(
                        parameter: param,
                        isEditing: viewModel.uiState.editingParameter?.id == param.id,
                        editingParameter: viewModel.uiState.editingParameter,
                        onEdit: { viewModel.onAction(.startEditParameter(param)) },
                        onDelete: { viewModel.onAction(.deleteParameter(param.id)) },
                        onUpdateKey: { viewModel.onAction(.updateEditKey($0)) },
                        onUpdateValue: { viewModel.onAction(.updateEditValue($0)) },
                        onConfirmEdit: { viewModel.onAction(.confirmEditParameter) },
                        onCancelEdit: { viewModel.onAction(.cancelEditParameter) }
                    )
                }

                if viewModel.uiState.isAddingNew {
                    NewParameterRow(
                        key: viewModel.uiState.newKey,
                        value: viewModel.uiState.newValue,
                        onKeyChange: { viewModel.onAction(.updateNewKey($0)) },
                        onValueChange: { viewModel.onAction(.updateNewValue($0)) },
                        onConfirm: { viewModel.onAction(.confirmAddParameter) },
                        onCancel: { viewModel.onAction(.cancelAddParameter) }
                    )
                }
            }
        }
    }

    private var footerView: some View {
        VStack(spacing: 8) {
            HStack(spacing: 8) {
                Button(action: { viewModel.onAction(.startAddParameter) }) {
                    Text("Add")
                        .font(.system(size: 14, weight: .medium))
                        .frame(maxWidth: .infinity)
                        .padding(.vertical, 12)
                        .foregroundColor(addButtonEnabled ? .blue : .gray)
                        .overlay(
                            RoundedRectangle(cornerRadius: 8)
                                .stroke(addButtonEnabled ? Color.blue : Color.gray, lineWidth: 1)
                        )
                }
                .disabled(!addButtonEnabled)

                Button(action: { viewModel.onAction(.clearAll) }) {
                    Text("Clear All")
                        .font(.system(size: 14, weight: .medium))
                        .frame(maxWidth: .infinity)
                        .padding(.vertical, 12)
                        .foregroundColor(clearAllEnabled ? .blue : .gray)
                        .overlay(
                            RoundedRectangle(cornerRadius: 8)
                                .stroke(clearAllEnabled ? Color.blue : Color.gray, lineWidth: 1)
                        )
                }
                .disabled(!clearAllEnabled)
            }

            Button(action: { viewModel.onAction(.applyParameters) }) {
                Text("Apply Parameters")
                    .font(.system(size: 14, weight: .semibold))
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 12)
                    .foregroundColor(.white)
                    .background(Color.blue)
                    .cornerRadius(8)
            }
        }
    }

    private var addButtonEnabled: Bool {
        !viewModel.uiState.isAddingNew && viewModel.uiState.editingParameter == nil
    }

    private var clearAllEnabled: Bool {
        !viewModel.uiState.parameters.isEmpty
    }

    private func handleEffect(_ effect: ParameterFormContract.UIEffect) {
        switch effect {
        case .parametersApplied, .dismissed:
            onDismiss()

        case .showToast(let message):
            showToastMessage(message)
        }
    }

    private func showToastMessage(_ message: String) {
        toastMessage = message
        showToast = true

        DispatchQueue.main.asyncAfter(deadline: .now() + 2.0) {
            showToast = false
            toastMessage = nil
        }
    }
}
