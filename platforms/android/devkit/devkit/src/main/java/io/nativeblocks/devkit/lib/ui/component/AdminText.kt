package io.nativeblocks.devkit.lib.ui.component

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.size
import androidx.compose.material.ButtonDefaults
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.tooling.preview.Preview
import io.nativeblocks.devkit.lib.ui.theme.AdminTheme

@Composable
internal fun AdminText(
    text: String,
    textStyle: TextStyle,
    modifier: Modifier = Modifier,
    textColor: Color = Color.Unspecified,
    backgroundColor: Color = Color.Transparent,
    textAlign: TextAlign = TextAlign.Start,
    overflow: TextOverflow = TextOverflow.Clip,
    maxLines: Int = Int.MAX_VALUE,
    leadingIcon: @Composable (() -> Unit)? = null,
    trailingIcon: @Composable (() -> Unit)? = null,
) {
    Row(
        modifier = Modifier
            .then(modifier)
            .background(backgroundColor),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        if (leadingIcon != null) {
            leadingIcon()
            Spacer(modifier = Modifier.size(ButtonDefaults.IconSpacing))
        }
        Text(
            modifier = modifier,
            text = text,
            color = textColor,
            style = textStyle,
            textAlign = textAlign,
            overflow = overflow,
            maxLines = maxLines,
        )
        if (trailingIcon != null) {
            Spacer(modifier = Modifier.size(ButtonDefaults.IconSpacing))
            trailingIcon()
        }
    }
}

@Preview(showBackground = false)
@Composable
internal fun DefaultTextPreview() {
    Column {
        AdminText(
            text = "headline 1",
            textColor = AdminTheme.colors.foregroundRegular,
            textStyle = AdminTheme.typography.h1Regular
        )
        AdminText(
            text = "headline 2",
            textColor = AdminTheme.colors.foregroundRegular,
            textStyle = AdminTheme.typography.h2Regular
        )
        AdminText(
            text = "subtitle",
            textColor = AdminTheme.colors.foregroundRegular,
            textStyle = AdminTheme.typography.s1Regular
        )
        AdminText(
            text = "body 1",
            textColor = AdminTheme.colors.foregroundRegular,
            textStyle = AdminTheme.typography.b1Regular
        )
        AdminText(
            text = "body 2",
            textColor = AdminTheme.colors.foregroundRegular,
            textStyle = AdminTheme.typography.b2Regular
        )
        AdminText(
            text = "caption",
            textColor = AdminTheme.colors.foregroundRegular,
            textStyle = AdminTheme.typography.overLineRegular,
        )
    }
}