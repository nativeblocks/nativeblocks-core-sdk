import Foundation
import SwiftUI

internal struct RootBlock: View {
    var blockProps: BlockProps
    @Environment(\.nativeWindowWidthClass) var windowManager

    var body: some View {
        let properties = blockProps.block.properties
        let slots = blockProps.block.slots

        let paddingStart = findWindowSizeClass(properties["paddingStart"], windowManager)?.toCGFloat() ?? 0.0
        let paddingTop = findWindowSizeClass(properties["paddingTop"], windowManager)?.toCGFloat() ?? 0.0
        let paddingEnd = findWindowSizeClass(properties["paddingEnd"], windowManager)?.toCGFloat() ?? 0.0
        let paddingBottom = findWindowSizeClass(properties["paddingBottom"], windowManager)?.toCGFloat() ?? 0.0

        let alignmentHorizontal = findWindowSizeClass(properties["alignmentHorizontal"], windowManager) ?? "leading"
        let spacing = (findWindowSizeClass(properties["spacing"], windowManager) ?? "").toCGFloat() ?? 0

        let backgroundColor = findWindowSizeClass(properties["backgroundColor"], windowManager) ?? "#FFFFFFFF"
        let contentSlot = slots["content"]

        VStack(
            alignment: findAlignmentHorizontal(alignmentHorizontal),
            spacing: spacing
        ) {
            if let contentSlot {
                blockProps.onSubBlock(blockProps.block.subBlocks ?? [:], contentSlot, NONE_INDEX, nil)
            }
        }
        .padding(EdgeInsets(top: paddingTop, leading: paddingStart, bottom: paddingBottom, trailing: paddingEnd))
        .background(Color(hex: backgroundColor))
    }
}

private func findAlignmentHorizontal(_ alignment: String) -> HorizontalAlignment {
    switch alignment.lowercased() {
    case "leading": return .leading
    case "trailing": return .trailing
    case "listrowseparatorleading":
        if #available(iOS 16.0, *) {
            return .listRowSeparatorLeading
        } else {
            return .leading
        }
    case "listrowseparatortrailing":
        if #available(iOS 16.0, *) {
            return .listRowSeparatorTrailing
        } else {
            return .leading
        }
    case "center": return .center
    default: return .leading
    }
}
