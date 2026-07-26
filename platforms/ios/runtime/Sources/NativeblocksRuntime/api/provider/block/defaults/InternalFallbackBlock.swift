import Foundation
import SwiftUI

internal struct InternalFallbackBlock: View {
    var key: String

    var body: some View {
        Text("The \(key) block isn’t available in this app version")
            .multilineTextAlignment(.center)
            .foregroundColor(.gray)
    }
}
