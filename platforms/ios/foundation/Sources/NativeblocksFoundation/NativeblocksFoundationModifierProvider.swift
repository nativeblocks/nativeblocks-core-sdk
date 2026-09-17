import NativeblocksRuntime
import SwiftUI
public class NativeblocksFoundationModifierProvider {
    public static func provideModifiers(name: String = "default") {
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/width") { content, modifierContext in
            AnyView(content.modifier(WidthModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/scroll") { content, modifierContext in
            AnyView(content.modifier(ScrollModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/test_tag") { content, modifierContext in
            AnyView(content.modifier(TestTagModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/background") { content, modifierContext in
            AnyView(content.modifier(BackgroundModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/shadow") { content, modifierContext in
            AnyView(content.modifier(ShadowModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/gradient") { content, modifierContext in
            AnyView(content.modifier(GradientModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/rotate") { content, modifierContext in
            AnyView(content.modifier(RotateModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/clip") { content, modifierContext in
            AnyView(content.modifier(ClipModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/paginate") { content, modifierContext in
            AnyView(content.modifier(PaginateModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/opacity") { content, modifierContext in
            AnyView(content.modifier(OpacityModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/border") { content, modifierContext in
            AnyView(content.modifier(BorderModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/padding") { content, modifierContext in
            AnyView(content.modifier(PaddingModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/height") { content, modifierContext in
            AnyView(content.modifier(HeightModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/aspect_ratio") { content, modifierContext in
            AnyView(content.modifier(AspectRatioModifier(modifierContext: modifierContext)))
        }
        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "nativeblocks/clickable") { content, modifierContext in
            AnyView(content.modifier(ClickableModifier(modifierContext: modifierContext)))
        }
    }
}