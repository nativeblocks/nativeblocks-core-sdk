import Foundation
import NativeblocksRuntime

public class FoundationProvider {

    public static func provide(name: String = "default") {
        NativeblocksFoundationBlockProvider.provideBlocks(name: name)
        NativeblocksFoundationModifierProvider.provideModifiers(name: name)
        NativeblocksFoundationTypeProvider.provideTypes(name: name)
    }
}
