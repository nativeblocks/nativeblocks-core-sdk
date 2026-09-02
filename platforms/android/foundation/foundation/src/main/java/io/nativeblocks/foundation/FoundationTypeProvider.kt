package io.nativeblocks.foundation

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.ui.Alignment
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.LayoutDirection
import androidx.compose.ui.unit.TextUnit
import io.nativeblocks.runtime.api.NativeblocksManager
import io.nativeblocks.foundation.types.AlignmentNativeType
import io.nativeblocks.foundation.types.ColorNativeType
import io.nativeblocks.foundation.types.ContentScaleNativeType
import io.nativeblocks.foundation.types.DpNativeType
import io.nativeblocks.foundation.types.FontWeightNativeType
import io.nativeblocks.foundation.types.HorizontalAlignmentNativeType
import io.nativeblocks.foundation.types.HorizontalArrangementNativeType
import io.nativeblocks.foundation.types.KeyboardTypeNativeType
import io.nativeblocks.foundation.types.LayoutDirectionNativeType
import io.nativeblocks.foundation.types.TextAlignNativeType
import io.nativeblocks.foundation.types.TextOverflowNativeType
import io.nativeblocks.foundation.types.TextUnitNativeType
import io.nativeblocks.foundation.types.VerticalAlignmentNativeType
import io.nativeblocks.foundation.types.VerticalArrangementNativeType

object FoundationTypeProvider {
    fun provideTypes(instanceName: String = "default") {
        NativeblocksManager.getInstance(instanceName)
            .provideTypeConverter(Dp::class, DpNativeType())
            .provideTypeConverter(Color::class, ColorNativeType())
            .provideTypeConverter(LayoutDirection::class, LayoutDirectionNativeType())
            .provideTypeConverter(TextUnit::class, TextUnitNativeType())
            .provideTypeConverter(Alignment::class, AlignmentNativeType())
            .provideTypeConverter(FontWeight::class, FontWeightNativeType())
            .provideTypeConverter(TextAlign::class, TextAlignNativeType())
            .provideTypeConverter(ContentScale::class, ContentScaleNativeType())
            .provideTypeConverter(Arrangement.Horizontal::class, HorizontalArrangementNativeType())
            .provideTypeConverter(Alignment.Horizontal::class, HorizontalAlignmentNativeType())
            .provideTypeConverter(Arrangement.Vertical::class, VerticalArrangementNativeType())
            .provideTypeConverter(Alignment.Vertical::class, VerticalAlignmentNativeType())
            .provideTypeConverter(TextOverflow::class, TextOverflowNativeType())
            .provideTypeConverter(KeyboardType::class, KeyboardTypeNativeType())
    }
}