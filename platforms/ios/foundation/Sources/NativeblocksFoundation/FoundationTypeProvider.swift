import NativeblocksRuntime
import SwiftUI

public class NativeblocksFoundationTypeProvider {
    public static func provideTypes(name: String = "default") {
        _ = NativeblocksManager.getInstance(name: name).provideTypeConverter(Color.self, converter: ColorNativeType())
        _ = NativeblocksManager.getInstance(name: name).provideTypeConverter(Font.Design.self, converter: FontDesignNativeType())
        _ = NativeblocksManager.getInstance(name: name).provideTypeConverter(Font.Weight.self, converter: FontWeightNativeType())
        _ = NativeblocksManager.getInstance(name: name).provideTypeConverter(
            HorizontalAlignment.self, converter: HorizontalAlignmentNativeType())
        _ = NativeblocksManager.getInstance(name: name).provideTypeConverter(LayoutDirection.self, converter: LayoutDirectionNativeType())
        _ = NativeblocksManager.getInstance(name: name).provideTypeConverter(Axis.Set.self, converter: ScrollableNativeType())
        _ = NativeblocksManager.getInstance(name: name).provideTypeConverter(TextAlignment.self, converter: TextAlignmentNativeType())
        _ = NativeblocksManager.getInstance(name: name).provideTypeConverter(
            VerticalAlignment.self, converter: VerticalAlignmentNativeType())
    }
}
