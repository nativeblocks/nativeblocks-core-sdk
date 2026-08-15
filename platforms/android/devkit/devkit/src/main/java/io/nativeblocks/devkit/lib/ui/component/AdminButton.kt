package io.nativeblocks.devkit.lib.ui.component

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Button
import androidx.compose.material.ButtonDefaults
import androidx.compose.material.OutlinedButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.nativeblocks.devkit.lib.ui.theme.AdminTheme

@Composable
internal fun AdminButton(
    buttonText: String,
    modifier: Modifier = Modifier,
    type: ButtonType = ButtonType.PRIMARY,
    enable: Boolean = true,
    leadingIcon: @Composable (() -> Unit)? = null,
    trailingIcon: @Composable (() -> Unit)? = null,
    onClick: (() -> Unit)? = null,
) {
    return if (type == ButtonType.PRIMARY) {
        Button(
            onClick = onClick ?: {},
            colors = ButtonDefaults.buttonColors(
                backgroundColor = AdminTheme.colors.primary,
                contentColor = AdminTheme.colors.onPrimary
            ),
            modifier = modifier.height(AdminTheme.dimensions.space11),
            shape = RoundedCornerShape(AdminTheme.radius.radius4),
            enabled = enable
        ) {
            AdminText(
                text = buttonText,
                textStyle = AdminTheme.typography.b2SemiBold,
                leadingIcon = leadingIcon,
                trailingIcon = trailingIcon
            )
        }
    } else {
        OutlinedButton(
            onClick = onClick ?: {},
            colors = ButtonDefaults.buttonColors(
                backgroundColor = Color.Transparent,
                contentColor = AdminTheme.colors.primary
            ),
            modifier = modifier.height(AdminTheme.dimensions.space11),
            border = BorderStroke(1.dp, AdminTheme.colors.primary),
            shape = RoundedCornerShape(AdminTheme.radius.radius4),
            enabled = enable
        ) {
            AdminText(
                text = buttonText,
                textStyle = AdminTheme.typography.b2SemiBold,
                leadingIcon = leadingIcon,
                trailingIcon = trailingIcon
            )
        }
    }
}


internal enum class ButtonType {
    PRIMARY,
    OUTLINE,
}

@Preview(showBackground = true)
@Composable
internal fun DefaultButtonPreview() {
    Column(
        verticalArrangement = Arrangement.spacedBy(12.dp),
        modifier = Modifier.padding(12.dp)
    ) {
        AdminButton(
            buttonText = "primary Button",
            modifier = Modifier.fillMaxWidth()
        ) {
            // handle click
        }
        AdminButton(
            buttonText = "primary Button",
            type = ButtonType.OUTLINE,
            modifier = Modifier.fillMaxWidth()
        ) {
            // handle click
        }
    }
}