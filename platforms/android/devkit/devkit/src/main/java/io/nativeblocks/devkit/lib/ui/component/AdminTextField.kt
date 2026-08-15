package io.nativeblocks.devkit.lib.ui.component

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.OutlinedTextField
import androidx.compose.material.Text
import androidx.compose.material.TextFieldDefaults
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import io.nativeblocks.devkit.lib.ui.theme.AdminTheme

@Composable
internal fun AdminTextField(
    value: String,
    onValueChange: (String) -> Unit,
    modifier: Modifier = Modifier,
    placeholder: String = "",
    label: String? = null,
    enabled: Boolean = true,
    singleLine: Boolean = true,
    keyboardType: KeyboardType = KeyboardType.Text,
    imeAction: ImeAction = ImeAction.Done,
    onImeAction: (() -> Unit)? = null
) {
    OutlinedTextField(
        value = value,
        onValueChange = onValueChange,
        modifier = modifier.fillMaxWidth(),
        enabled = enabled,
        singleLine = singleLine,
        textStyle = AdminTheme.typography.b2Regular,
        label = label?.let { { Text(text = it, style = AdminTheme.typography.overLineRegular) } },
        placeholder = {
            Text(
                text = placeholder,
                style = AdminTheme.typography.b2Regular,
                color = AdminTheme.colors.foregroundRegular
            )
        },
        colors = TextFieldDefaults.outlinedTextFieldColors(
            textColor = AdminTheme.colors.foregroundEmphasize,
            backgroundColor = AdminTheme.colors.surface,
            focusedBorderColor = AdminTheme.colors.primary,
            unfocusedBorderColor = AdminTheme.colors.outline,
            disabledBorderColor = AdminTheme.colors.outline,
            cursorColor = AdminTheme.colors.primary,
            placeholderColor = AdminTheme.colors.foregroundRegular,
            focusedLabelColor = AdminTheme.colors.primary,
            unfocusedLabelColor = AdminTheme.colors.foregroundRegular
        ),
        shape = RoundedCornerShape(AdminTheme.radius.radius4),
        keyboardOptions = KeyboardOptions(
            keyboardType = keyboardType,
            imeAction = imeAction
        ),
        keyboardActions = KeyboardActions(
            onDone = { onImeAction?.invoke() },
            onNext = { onImeAction?.invoke() },
            onSearch = { onImeAction?.invoke() }
        )
    )
}
