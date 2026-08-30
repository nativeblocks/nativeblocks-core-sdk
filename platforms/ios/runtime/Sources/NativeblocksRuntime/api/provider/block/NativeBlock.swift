import SwiftUI

public enum NativeBlock {

    /// Produces UI where it sits.
    case rendering((BlockContext) -> any View)

    /// Says what will exist in the slot it sits in, into the scope that slot hands it.
    case describing((BlockContext, Any) -> any View)
}
