package io.nativeblocks.devkit.feature.auth.presenter

import android.os.Bundle
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.runtime.getValue
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import io.nativeblocks.devkit.DevKitInjector
import io.nativeblocks.devkit.lib.permission.PermissionContractor
import io.nativeblocks.devkit.lib.ui.theme.AdminTheme
import io.nativeblocks.devkit.util.collectAsUiEffectWithLifecycle
import org.koin.androidx.compose.koinViewModel
import org.koin.compose.KoinIsolatedContext

internal class AuthActivity : ComponentActivity() {
    private val permissionContractor: PermissionContractor by DevKitInjector.get().koin.inject()
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            KoinIsolatedContext(context = DevKitInjector.get()) {
                AdminTheme {
                    permissionContractor.PermissionContractorCollector()
                    val viewModel = koinViewModel<AuthViewModel>()
                    val uiState by viewModel.uiState.collectAsStateWithLifecycle()
                    val context = LocalContext.current
                    viewModel.uiEffect.collectAsUiEffectWithLifecycle { effect ->
                        when (effect) {
                            is AuthContract.UIEffect.ShowToast -> {
                                Toast.makeText(context, effect.message, Toast.LENGTH_SHORT).show()
                            }

                            AuthContract.UIEffect.Close -> {
                                finish()
                            }
                        }
                    }
                    AuthScreen(uiState, viewModel::onAction)
                }
            }
        }
    }
}
