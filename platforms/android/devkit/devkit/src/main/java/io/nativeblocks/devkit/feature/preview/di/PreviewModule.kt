package io.nativeblocks.devkit.feature.preview.di

import io.nativeblocks.devkit.feature.preview.data.ParameterRepositoryImpl
import io.nativeblocks.devkit.feature.preview.domain.repository.ParameterRepository
import io.nativeblocks.devkit.feature.preview.presenter.ParameterFormViewModel
import org.koin.core.module.dsl.bind
import org.koin.core.module.dsl.singleOf
import org.koin.core.module.dsl.viewModel
import org.koin.dsl.module

internal val PreviewModule = module {
    singleOf(::ParameterRepositoryImpl) { bind<ParameterRepository>() }
    viewModel { (instanceName: String) ->
        ParameterFormViewModel(get(), instanceName)
    }
}
