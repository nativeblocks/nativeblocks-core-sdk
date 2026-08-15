package io.nativeblocks.devkit.feature.logging.di

import io.nativeblocks.devkit.feature.logging.DevLogger
import io.nativeblocks.devkit.feature.logging.data.LoggingRepositoryImpl
import io.nativeblocks.devkit.feature.logging.domain.repository.LoggingRepository
import org.koin.core.module.dsl.bind
import org.koin.core.module.dsl.factoryOf
import org.koin.dsl.module

internal val LoggingModule = module {
    factoryOf(::LoggingRepositoryImpl) { bind<LoggingRepository>() }
    factoryOf(::DevLogger)
}
