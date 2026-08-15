import SwiftUI

internal struct ParameterRowView: View {

    let parameter: PreviewParameter
    let isEditing: Bool
    let editingParameter: PreviewParameter?
    let onEdit: () -> Void
    let onDelete: () -> Void
    let onUpdateKey: (String) -> Void
    let onUpdateValue: (String) -> Void
    let onConfirmEdit: () -> Void
    let onCancelEdit: () -> Void

    var body: some View {
        if isEditing, let editing = editingParameter {
            EditingParameterRow(
                parameter: editing,
                onUpdateKey: onUpdateKey,
                onUpdateValue: onUpdateValue,
                onConfirm: onConfirmEdit,
                onCancel: onCancelEdit
            )
        } else {
            DisplayParameterRow(
                parameter: parameter,
                onEdit: onEdit,
                onDelete: onDelete
            )
        }
    }
}

private struct DisplayParameterRow: View {
    let parameter: PreviewParameter
    let onEdit: () -> Void
    let onDelete: () -> Void

    var body: some View {
        HStack(alignment: .center, spacing: 8) {
            VStack(alignment: .leading, spacing: 4) {
                Text(parameter.name)
                    .font(.system(size: 14, weight: .semibold))
                    .foregroundColor(.primary)

                Text(parameter.value.isEmpty ? "(empty)" : parameter.value)
                    .font(.system(size: 14))
                    .foregroundColor(.secondary)
            }

            Spacer()

            Button(action: onEdit) {
                Image(systemName: "pencil")
                    .foregroundColor(.blue)
            }
            .buttonStyle(BorderlessButtonStyle())

            Button(action: onDelete) {
                Image(systemName: "trash")
                    .foregroundColor(.red)
            }
            .buttonStyle(BorderlessButtonStyle())
        }
        .padding(12)
        #if os(iOS)
        .background(Color(.systemGray6))
        #else
        .background(Color(nsColor: .controlBackgroundColor))
        #endif
        .cornerRadius(8)
    }
}

private struct EditingParameterRow: View {
    let parameter: PreviewParameter
    let onUpdateKey: (String) -> Void
    let onUpdateValue: (String) -> Void
    let onConfirm: () -> Void
    let onCancel: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            TextField("Key", text: Binding(
                get: { parameter.name },
                set: { onUpdateKey($0) }
            ))
            .textFieldStyle(RoundedBorderTextFieldStyle())

            TextField("Value", text: Binding(
                get: { parameter.value },
                set: { onUpdateValue($0) }
            ))
            .textFieldStyle(RoundedBorderTextFieldStyle())

            HStack {
                Spacer()

                Button(action: onCancel) {
                    Image(systemName: "xmark")
                        .foregroundColor(.red)
                        .padding(8)
                }
                .buttonStyle(BorderlessButtonStyle())

                Button(action: onConfirm) {
                    Image(systemName: "checkmark")
                        .foregroundColor(.green)
                        .padding(8)
                }
                .buttonStyle(BorderlessButtonStyle())
            }
        }
        .padding(12)
        #if os(iOS)
        .background(Color(.systemGray6))
        #else
        .background(Color(nsColor: .controlBackgroundColor))
        #endif
        .cornerRadius(8)
    }
}

internal struct NewParameterRow: View {
    let key: String
    let value: String
    let onKeyChange: (String) -> Void
    let onValueChange: (String) -> Void
    let onConfirm: () -> Void
    let onCancel: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("New Parameter")
                .font(.system(size: 14, weight: .semibold))
                .foregroundColor(.primary)

            TextField("Key", text: Binding(
                get: { key },
                set: { onKeyChange($0) }
            ))
            .textFieldStyle(RoundedBorderTextFieldStyle())

            TextField("Value", text: Binding(
                get: { value },
                set: { onValueChange($0) }
            ))
            .textFieldStyle(RoundedBorderTextFieldStyle())

            HStack {
                Spacer()

                Button(action: onCancel) {
                    Image(systemName: "xmark")
                        .foregroundColor(.red)
                        .padding(8)
                }
                .buttonStyle(BorderlessButtonStyle())

                Button(action: onConfirm) {
                    Image(systemName: "checkmark")
                        .foregroundColor(.green)
                        .padding(8)
                }
                .buttonStyle(BorderlessButtonStyle())
            }
        }
        .padding(12)
        #if os(iOS)
        .background(Color(.systemGray6))
        #else
        .background(Color(nsColor: .controlBackgroundColor))
        #endif
        .cornerRadius(8)
    }
}
