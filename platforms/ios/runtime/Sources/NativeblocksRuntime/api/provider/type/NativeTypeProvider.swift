import Foundation

/// A class responsible for managing and providing type converters for Native types.
/// This class allows registering and retrieving type converters for specific types.
internal class NativeTypeProvider {
    private var types: [String: Any] = [:]

    ///
    ///Registers a type converter for a specific type.
    ///
    ///- Parameters:
    ///  - type: The type for which the converter is being registered.
    ///  - converter: The converter that implements `INativeType` for the given type.
    ///
    func provideTypeConverter<T>(_ type: T.Type, converter: INativeType<T>) {
        let typeName = String(describing: type)
        types[typeName] = MemoizedNativeType(delegate: converter)
    }

    ///
    ///Retrieves the type converter for a specific type.
    ///
    ///- Parameter type: The type for which the converter is being retrieved.
    ///- Returns: The converter that implements `INativeType` for the given type.
    ///- Precondition: A converter for the specified type must have been registered using `provideTypeConverter`.
    ///- Postcondition: If no converter is found, a fatal error will occur.
    ///
    func getTypeConverter<T>(_ type: T.Type) -> INativeType<T> {
        let typeName = String(describing: type)
        if let serializer = types[typeName] as? INativeType<T> {
            return serializer
        } else {
            fatalError("The \(typeName) converter not provided.")
        }
    }
}

/// A converter is called once per property, per block, per frame update, so the
/// same string is decoded over and over. This caches `fromString` results.
private final class MemoizedNativeType<T>: INativeType<T> {

    private let delegate: INativeType<T>
    private var cache: [String: T] = [:]
    private let lock = NSLock()

    init(delegate: INativeType<T>) {
        self.delegate = delegate
        super.init()
    }

    override func toString(_ input: T?) -> String {
        return delegate.toString(input)
    }

    override func fromString(_ input: String?) -> T {
        guard let input else { return delegate.fromString(nil) }

        lock.lock()
        defer { lock.unlock() }
        if let cached = cache[input] { return cached }
        let value = delegate.fromString(input)
        cache[input] = value
        return value
    }
}
