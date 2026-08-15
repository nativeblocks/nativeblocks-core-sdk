package io.nativeblocks.devkit.feature.auth.di

import io.nativeblocks.devkit.feature.auth.data.AuthRepositoryImpl
import io.nativeblocks.devkit.feature.auth.domain.repository.AuthRepository
import io.nativeblocks.devkit.feature.auth.presenter.AuthViewModel
import io.nativeblocks.devkit.lib.permission.PermissionContractor
import org.koin.core.module.dsl.bind
import org.koin.core.module.dsl.factoryOf
import org.koin.core.module.dsl.singleOf
import org.koin.core.module.dsl.viewModelOf
import org.koin.dsl.module

internal val AuthModule = module {
    viewModelOf(::AuthViewModel)
    factoryOf(::AuthRepositoryImpl) { bind<AuthRepository>() }
    singleOf(::PermissionContractor)
}