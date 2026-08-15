import Foundation

public protocol Kit {

    func attach(instanceName: String, edition: NativeblocksEdition)

    func detach(instanceName: String)
}
