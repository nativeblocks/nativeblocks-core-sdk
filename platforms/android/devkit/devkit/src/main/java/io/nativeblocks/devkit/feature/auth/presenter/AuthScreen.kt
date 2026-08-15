package io.nativeblocks.devkit.feature.auth.presenter

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import androidx.activity.compose.BackHandler
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.compose.animation.core.animateDpAsState
import androidx.compose.animation.core.tween
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Card
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.journeyapps.barcodescanner.ScanContract
import com.journeyapps.barcodescanner.ScanOptions
import io.nativeblocks.devkit.R
import io.nativeblocks.devkit.lib.ui.component.AdminButton
import io.nativeblocks.devkit.lib.ui.component.AdminText
import io.nativeblocks.devkit.lib.ui.component.ButtonType
import io.nativeblocks.devkit.lib.ui.theme.AdminTheme

@Composable
internal fun AuthScreen(uiState: AuthContract.UIState, onAction: (AuthContract.UIAction) -> Unit) {
    val bottomSheetHeight = animateDpAsState(
        targetValue = if (uiState.hasPermission.not()) 250.dp else 200.dp,
        animationSpec = tween(durationMillis = 500), label = ""
    )
    val context = LocalContext.current
    val clipboardManager = remember {
        context.getSystemService(Context.CLIPBOARD_SERVICE) as ClipboardManager
    }

    LaunchedEffect(uiState.pasteFromClipboard) {
        if (uiState.pasteFromClipboard) {
            val clip: ClipData? = clipboardManager.primaryClip
            val text = clip?.getItemAt(0)?.text?.toString()
            onAction(AuthContract.UIAction.OnCopyFromClipboardFinished(text))
        }
    }

    val scanLauncher = rememberLauncherForActivityResult(
        contract = ScanContract(),
        onResult = { result ->
            if (result.contents.isNullOrEmpty().not()) {
                onAction.invoke(AuthContract.UIAction.OnScan(result.contents))
            }
        },
    )
    val scanOptions = remember {
        ScanOptions()
            .setPrompt("Please scan the QR code")
            .setBeepEnabled(true)
            .setOrientationLocked(false)
    }

    BackHandler(onBack = { onAction.invoke(AuthContract.UIAction.OnClose) })
    Box(
        modifier = Modifier
            .background(Color.Transparent)
            .fillMaxSize()
    ) {
        Column {
            Spacer(modifier = Modifier.weight(0.5f))
            Card(
                modifier = Modifier
                    .fillMaxWidth()
                    .height(bottomSheetHeight.value),
                backgroundColor = AdminTheme.colors.surface,
                elevation = AdminTheme.dimensions.space4,
                shape = RoundedCornerShape(
                    topStart = AdminTheme.radius.radius8,
                    topEnd = AdminTheme.radius.radius8
                )
            ) {
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(bottomSheetHeight.value)
                        .padding(AdminTheme.dimensions.space4),
                ) {
                    Column(
                        modifier = Modifier.fillMaxSize(),
                        verticalArrangement = Arrangement.SpaceBetween,
                        horizontalAlignment = Alignment.CenterHorizontally,
                    ) {
                        AdminText(
                            text = stringResource(id = R.string.auth_title),
                            textStyle = AdminTheme.typography.h2Bold,
                            textColor = AdminTheme.colors.foregroundEmphasize,
                            modifier = Modifier.fillMaxWidth()
                        )
                        if (uiState.hasPermission.not()) {
                            Column {
                                AdminText(
                                    text = stringResource(id = R.string.auth_camera_access_required),
                                    textStyle = AdminTheme.typography.b2Regular,
                                    textColor = AdminTheme.colors.foregroundRegular,
                                    textAlign = TextAlign.Start,
                                    modifier = Modifier.padding(vertical = AdminTheme.dimensions.space2)
                                )
                                AdminButton(
                                    buttonText = stringResource(id = R.string.auth_enable_camera_access),
                                    modifier = Modifier.fillMaxWidth()
                                ) {
                                    onAction.invoke(AuthContract.UIAction.OnPermissionRequest)
                                }
                                Spacer(modifier = Modifier.size(AdminTheme.dimensions.space4))
                                PasteFromClipboardButton(onAction = onAction)
                            }
                        } else {
                            Column(
                                verticalArrangement = Arrangement.Center,
                                horizontalAlignment = Alignment.CenterHorizontally,
                                modifier = Modifier.fillMaxSize()
                            ) {
                                AdminButton(
                                    buttonText = stringResource(id = R.string.auth_start_scan),
                                    modifier = Modifier.fillMaxWidth()
                                ) {
                                    scanLauncher.launch(scanOptions)
                                }
                                Spacer(modifier = Modifier.size(AdminTheme.dimensions.space4))
                                PasteFromClipboardButton(onAction = onAction)
                            }
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun PasteFromClipboardButton(onAction: (AuthContract.UIAction) -> Unit) {
    AdminButton(
        modifier = Modifier
            .fillMaxWidth(),
        type = ButtonType.OUTLINE,
        onClick = {
            onAction.invoke(AuthContract.UIAction.CopyFromClipboard)
        },
        buttonText = stringResource(id = R.string.auth_copy_from_clipboard),
        enable = true
    )
}


@Preview
@Composable
internal fun AuthPreview() {
    AuthScreen(uiState = AuthContract.UIState.init()) {

    }
}