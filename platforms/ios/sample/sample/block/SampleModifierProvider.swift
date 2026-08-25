import NativeblocksRuntime
import SwiftUI

public class SampleModifierProvider {
    public static func provideModifiers(name: String = "default") {
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/opacity") { content, modifierContext in
            AnyView(content.modifier(NativeOpacityModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "SAMPLE_TAP") { content, modifierContext in
            AnyView(content.modifier(SampleTapModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "SAMPLE_SHADOW") { content, modifierContext in
            AnyView(content.modifier(SampleShadowModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/padding") { content, modifierContext in
            AnyView(content.modifier(NativePaddingModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/shape") { content, modifierContext in
            AnyView(content.modifier(NativeShapeModifier(modifierContext: modifierContext)))
        }
    }
}
