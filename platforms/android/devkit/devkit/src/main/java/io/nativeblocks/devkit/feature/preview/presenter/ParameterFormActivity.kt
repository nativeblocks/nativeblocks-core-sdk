package io.nativeblocks.devkit.feature.preview.presenter

import android.os.Bundle
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.runtime.getValue
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import io.nativeblocks.devkit.PreviewKitInjector
import io.nativeblocks.devkit.lib.ui.theme.AdminTheme
import io.nativeblocks.devkit.util.collectAsUiEffectWithLifecycle
import org.koin.androidx.compose.koinViewModel
import org.koin.compose.KoinIsolatedContext
import org.koin.core.parameter.parametersOf

internal class ParameterFormActivity : ComponentActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val instanceName = intent.getStringExtra(EXTRA_INSTANCE_NAME) ?: ""

        setContent {
            KoinIsolatedContext(context = PreviewKitInjector.get()) {
                AdminTheme {
                    val viewModel = koinViewModel<ParameterFormViewModel> {
                        parametersOf(instanceName)
                    }
                    val uiState by viewModel.uiState.collectAsStateWithLifecycle()
                    val context = LocalContext.current

                    viewModel.uiEffect.collectAsUiEffectWithLifecycle { effect ->
                        when (effect) {
                            is ParameterFormContract.UIEffect.ShowToast -> {
                                Toast.makeText(context, effect.message, Toast.LENGTH_SHORT).show()
                            }

                            ParameterFormContract.UIEffect.Dismissed,
                            ParameterFormContract.UIEffect.ParametersApplied -> {
                                finish()
                            }
                        }
                    }

                    ParameterFormScreen(
                        uiState = uiState,
                        onAction = viewModel::onAction
                    )
                }
            }
        }
    }

    companion object {
        const val EXTRA_INSTANCE_NAME = "instance_name"
    }
}
