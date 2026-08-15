package io.nativeblocks.devkit.feature.preview.presenter

import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Card
import androidx.compose.material.Divider
import androidx.compose.material.Icon
import androidx.compose.material.IconButton
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.Edit
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.nativeblocks.devkit.feature.preview.domain.model.Parameter
import io.nativeblocks.devkit.feature.preview.presenter.ParameterFormContract.UIAction
import io.nativeblocks.devkit.feature.preview.presenter.ParameterFormContract.UIState
import io.nativeblocks.devkit.lib.ui.component.AdminButton
import io.nativeblocks.devkit.lib.ui.component.AdminText
import io.nativeblocks.devkit.lib.ui.component.AdminTextField
import io.nativeblocks.devkit.lib.ui.component.ButtonType
import io.nativeblocks.devkit.lib.ui.theme.AdminTheme

@Composable
internal fun ParameterFormScreen(
    uiState: UIState,
    onAction: (UIAction) -> Unit
) {
    Card(
        modifier = Modifier
            .fillMaxSize()
            .clickable(
                interactionSource = remember { MutableInteractionSource() },
                indication = null
            ) {},
        backgroundColor = AdminTheme.colors.background,
        elevation = AdminTheme.dimensions.space1,
        shape = RoundedCornerShape(AdminTheme.radius.radius6)
    ) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(AdminTheme.dimensions.space4)
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                AdminText(
                    text = "Preview Parameters",
                    textStyle = AdminTheme.typography.s1SemiBold,
                    textColor = AdminTheme.colors.foregroundEmphasize
                )
                IconButton(
                    onClick = { onAction(UIAction.Close) },
                    modifier = Modifier.size(AdminTheme.dimensions.space8)
                ) {
                    Icon(
                        imageVector = Icons.Default.Close,
                        contentDescription = "Close",
                        tint = AdminTheme.colors.foregroundRegular
                    )
                }
            }

            Spacer(modifier = Modifier.height(AdminTheme.dimensions.space3))
            Divider(color = AdminTheme.colors.outline)
            Spacer(modifier = Modifier.height(AdminTheme.dimensions.space3))

            LazyColumn(
                modifier = Modifier.weight(1f),
                verticalArrangement = Arrangement.spacedBy(AdminTheme.dimensions.space2)
            ) {
                items(uiState.parameters, key = { it.id }) { param ->
                    ParameterRowItem(
                        parameter = param,
                        isEditing = uiState.editingParameter?.id == param.id,
                        editingParameter = uiState.editingParameter,
                        onEdit = { onAction(UIAction.StartEditParameter(param)) },
                        onDelete = { onAction(UIAction.DeleteParameter(param.id)) },
                        onUpdateKey = { onAction(UIAction.UpdateEditKey(it)) },
                        onUpdateValue = { onAction(UIAction.UpdateEditValue(it)) },
                        onConfirmEdit = { onAction(UIAction.ConfirmEditParameter) },
                        onCancelEdit = { onAction(UIAction.CancelEditParameter) }
                    )
                }

                if (uiState.isAddingNew) {
                    item {
                        NewParameterRow(
                            key = uiState.newKey,
                            value = uiState.newValue,
                            onKeyChange = { onAction(UIAction.UpdateNewKey(it)) },
                            onValueChange = { onAction(UIAction.UpdateNewValue(it)) },
                            onConfirm = { onAction(UIAction.ConfirmAddParameter) },
                            onCancel = { onAction(UIAction.CancelAddParameter) }
                        )
                    }
                }

                if (uiState.parameters.isEmpty() && !uiState.isAddingNew) {
                    item {
                        Box(
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(vertical = AdminTheme.dimensions.space8),
                            contentAlignment = Alignment.Center
                        ) {
                            AdminText(
                                text = "No parameters yet. Tap \"Add\" to create one.",
                                textStyle = AdminTheme.typography.b2Regular,
                                textColor = AdminTheme.colors.foregroundRegular
                            )
                        }
                    }
                }
            }

            Spacer(modifier = Modifier.height(AdminTheme.dimensions.space3))
            Divider(color = AdminTheme.colors.outline)
            Spacer(modifier = Modifier.height(AdminTheme.dimensions.space3))

            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(AdminTheme.dimensions.space2)
            ) {
                AdminButton(
                    buttonText = "Add",
                    type = ButtonType.OUTLINE,
                    modifier = Modifier.weight(1f),
                    enable = !uiState.isAddingNew && uiState.editingParameter == null,
                    onClick = { onAction(UIAction.StartAddParameter) }
                )
                AdminButton(
                    buttonText = "Clear All",
                    type = ButtonType.OUTLINE,
                    modifier = Modifier.weight(1f),
                    enable = uiState.parameters.isNotEmpty(),
                    onClick = { onAction(UIAction.ClearAll) }
                )
            }

            Spacer(modifier = Modifier.height(AdminTheme.dimensions.space2))

            AdminButton(
                buttonText = "Apply Parameters",
                modifier = Modifier.fillMaxWidth(),
                onClick = { onAction(UIAction.ApplyParameters) }
            )
        }
    }
}

@Composable
private fun ParameterRowItem(
    parameter: Parameter,
    isEditing: Boolean,
    editingParameter: Parameter?,
    onEdit: () -> Unit,
    onDelete: () -> Unit,
    onUpdateKey: (String) -> Unit,
    onUpdateValue: (String) -> Unit,
    onConfirmEdit: () -> Unit,
    onCancelEdit: () -> Unit
) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        backgroundColor = AdminTheme.colors.surface,
        elevation = 0.dp,
        shape = RoundedCornerShape(AdminTheme.radius.radius4)
    ) {
        if (isEditing && editingParameter != null) {
            Column(
                modifier = Modifier.padding(AdminTheme.dimensions.space3),
                verticalArrangement = Arrangement.spacedBy(AdminTheme.dimensions.space2)
            ) {
                AdminTextField(
                    value = editingParameter.key,
                    onValueChange = onUpdateKey,
                    placeholder = "Key"
                )
                AdminTextField(
                    value = editingParameter.value,
                    onValueChange = onUpdateValue,
                    placeholder = "Value"
                )
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.End,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    IconButton(onClick = onCancelEdit) {
                        Icon(
                            imageVector = Icons.Default.Close,
                            contentDescription = "Cancel",
                            tint = AdminTheme.colors.error
                        )
                    }
                    IconButton(onClick = onConfirmEdit) {
                        Icon(
                            imageVector = Icons.Default.Check,
                            contentDescription = "Confirm",
                            tint = AdminTheme.colors.success
                        )
                    }
                }
            }
        } else {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(AdminTheme.dimensions.space3),
                verticalAlignment = Alignment.CenterVertically
            ) {
                Column(modifier = Modifier.weight(1f)) {
                    AdminText(
                        text = parameter.key,
                        textStyle = AdminTheme.typography.b2SemiBold,
                        textColor = AdminTheme.colors.foregroundEmphasize
                    )
                    Spacer(modifier = Modifier.height(AdminTheme.dimensions.space1))
                    AdminText(
                        text = parameter.value.ifEmpty { "(empty)" },
                        textStyle = AdminTheme.typography.b2Regular,
                        textColor = AdminTheme.colors.foregroundRegular
                    )
                }
                IconButton(onClick = onEdit) {
                    Icon(
                        imageVector = Icons.Default.Edit,
                        contentDescription = "Edit",
                        tint = AdminTheme.colors.primary
                    )
                }
                IconButton(onClick = onDelete) {
                    Icon(
                        imageVector = Icons.Default.Delete,
                        contentDescription = "Delete",
                        tint = AdminTheme.colors.error
                    )
                }
            }
        }
    }
}

@Composable
private fun NewParameterRow(
    key: String,
    value: String,
    onKeyChange: (String) -> Unit,
    onValueChange: (String) -> Unit,
    onConfirm: () -> Unit,
    onCancel: () -> Unit
) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        backgroundColor = AdminTheme.colors.surface,
        elevation = 0.dp,
        shape = RoundedCornerShape(AdminTheme.radius.radius4)
    ) {
        Column(
            modifier = Modifier.padding(AdminTheme.dimensions.space3),
            verticalArrangement = Arrangement.spacedBy(AdminTheme.dimensions.space2)
        ) {
            AdminText(
                text = "New Parameter",
                textStyle = AdminTheme.typography.b2SemiBold,
                textColor = AdminTheme.colors.foregroundEmphasize
            )
            AdminTextField(
                value = key,
                onValueChange = onKeyChange,
                placeholder = "Key"
            )
            AdminTextField(
                value = value,
                onValueChange = onValueChange,
                placeholder = "Value"
            )
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.End,
                verticalAlignment = Alignment.CenterVertically
            ) {
                IconButton(onClick = onCancel) {
                    Icon(
                        imageVector = Icons.Default.Close,
                        contentDescription = "Cancel",
                        tint = AdminTheme.colors.error
                    )
                }
                IconButton(onClick = onConfirm) {
                    Icon(
                        imageVector = Icons.Default.Check,
                        contentDescription = "Confirm",
                        tint = AdminTheme.colors.success
                    )
                }
            }
        }
    }
}
