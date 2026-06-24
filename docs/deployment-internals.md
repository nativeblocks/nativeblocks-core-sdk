# Deployment internals — how the process works

This explains, step by step, what `./scripts/release.sh` (and the per-platform
`build-*.sh` scripts) actually do and **why**. If you just want the commands to
run each release, use the Deploy section of [../README.md](../README.md); read this when
something breaks or you need to change the pipeline.

---

## 1. The big picture

There is **one** Rust crate. It compiles to a different binary shape per
platform, and a different language binding is generated to call into it. The
binding is the typed API the host app uses; the compiled library is the machine
code it calls.

```
                         ┌────────────────────────────┐
                         │   nativeblocks_core_sdk      │   (this crate)
                         │   #[uniffi::export] surface  │
                         └───────────────┬──────────────┘
            cargo build (per target)     │     uniffi-bindgen (per language)
        ┌───────────────┬────────────────┼────────────────┐
        ▼               ▼                                  ▼
   libn<…>.so      libn<…>.a                          <name>.kt / .swift / .dart
   (cdylib)        (staticlib)                        (the typed API)
        │               │                                  │
   Android/Flutter   iOS (xcframework)                 host app code
```

Two halves always ship **together** and must come from the **same build**:

1. the **compiled library** (machine code), and
2. the **generated binding** (the Kotlin/Swift/Dart that knows the function
   signatures and how to lay out arguments in memory).

If they drift (e.g. you regenerate bindings but ship an old `.so`), calls
corrupt memory or fail to link. That's why `release.sh` always does both.

### Why three binary shapes? (`crate-type`)

`Cargo.toml` sets `crate-type = ["lib", "cdylib", "staticlib"]`:

| Shape       | File                | Who consumes it                                    |
| ----------- | ------------------- | -------------------------------------------------- |
| `cdylib`    | `.so` / `.dylib`    | Android (loaded by JNA) and Flutter (Dart FFI)     |
| `staticlib` | `.a`                | iOS (linked into the app via an `.xcframework`)    |
| `lib`       | (rlib)              | the `uniffi-bindgen` binary + Rust unit tests      |

Android/Flutter load a library **dynamically at runtime**, so they need the
self-contained `cdylib`. iOS prefers to **statically link** the core into the
app binary, so it needs the `staticlib`.

### UniFFI: bindings are generated, never hand-written

The public API is declared once in Rust with `#[uniffi::export]` /
`#[derive(uniffi::Record)]` / `#[derive(uniffi::Enum)]` /
`#[uniffi::export(callback_interface)]`. `uniffi-bindgen` reads the **compiled
library's** metadata and emits the matching Kotlin/Swift/Dart. There is no `.udl`
file to maintain. Regenerate with `./scripts/generate-bindings.sh`; never edit
files under `bindings/` by hand.

---

## 2. Cross-compilation targets

`cargo build` defaults to your host architecture. To produce mobile binaries you
build for explicit **target triples** (installed once with `rustup target add`):

| Platform | Target triple                | Output            | Notes                          |
| -------- | ---------------------------- | ----------------- | ------------------------------ |
| Android  | `aarch64-linux-android`      | arm64 `.so`       | most modern phones             |
| Android  | `armv7-linux-androideabi`    | arm32 `.so`       | older phones                   |
| Android  | `x86_64-linux-android`       | x86_64 `.so`      | emulator on Intel/AMD          |
| Android  | `i686-linux-android`         | x86 `.so`         | older emulators                |
| iOS      | `aarch64-apple-ios`          | arm64 `.a`        | physical devices               |
| iOS      | `aarch64-apple-ios-sim`      | arm64 `.a`        | simulator on Apple Silicon     |

You ship **all** the ABIs a platform might run on; the OS/loader picks the right
one at install/run time.

---

## 3. Android, step by step

What `scripts/build-android.sh` does:

### 3.1 Build a `.so` per ABI with `cargo-ndk`

```bash
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -t x86 \
  -o ./dist/android/jniLibs build --release
```

- Plain `cargo build --target aarch64-linux-android` fails because the linker
  needs the Android **NDK** toolchain (the right `clang`, sysroot, and linker
  flags for each API level). `cargo-ndk` configures all of that.
- `-o ./dist/android/jniLibs` lays the outputs into the **exact directory
  structure Android expects** — `jniLibs/<abi>/lib<name>.so`. Gradle bundles
  anything under `src/main/jniLibs/` into the APK automatically, and at runtime
  `System.loadLibrary`/JNA finds it there.
- `--release` applies the size-optimized profile (`opt-level=z`, `lto`, `strip`).

The Android ABI names (`arm64-v8a`, …) map to the Rust triples above; `cargo-ndk`
does that translation.

### 3.2 Generate the Kotlin binding

`generate-bindings.sh` runs `uniffi-bindgen … --language kotlin`, producing
`bindings/kotlin/io/nativeblocks/core/engine/nativeblocks_core_sdk.kt`. The
package (`io.nativeblocks.core.engine`) is configured in `uniffi.toml`
(`[bindings.kotlin] package_name`); without it UniFFI would default to
`uniffi.nativeblocks_core_sdk`. The `io/nativeblocks/core/engine/` path **is the
Kotlin package**, so it must be preserved when you copy it into `src/main/java/`.

### 3.3 Why the JNA dependency?

UniFFI's Kotlin runtime calls the native `.so` through **JNA** (Java Native
Access). Without `net.java.dev.jna:jna:5.14.0@aar` on the classpath you get
`ClassNotFoundException: com.sun.jna.*` at runtime. The `@aar` classifier matters
— it's the Android-packaged build of JNA that includes its own native helper.

> Once you export **async** functions over FFI you'll also need
> `org.jetbrains.kotlinx:kotlinx-coroutines-core`. The current surface (records +
> the `Logger` callback) does not, so it's omitted.

### 3.4 Packaging as an `.aar` (optional)

Putting the `.so` files and the `.kt` into an `android-library` module and running
`./gradlew :core-sdk:assembleRelease` produces a single `.aar` that bundles both
— the cleanest way to version and distribute to app teams.

---

## 4. iOS, step by step

What `scripts/build-ios.sh` does:

### 4.1 Build the static lib for device + simulator

```bash
cargo build --release --target aarch64-apple-ios       # device
cargo build --release --target aarch64-apple-ios-sim   # simulator (Apple Silicon)
```

The simulator slice is **arm64-only** — Intel-Mac simulators are intentionally
not supported, so there is no `x86_64-apple-ios` build and no `lipo` fuse step.
The `aarch64-apple-ios-sim` archive is used directly.

### 4.2 No `lipo` fuse (arm64-only simulator)

Because the simulator slice is arm64-only, there is nothing to fuse — the
`aarch64-apple-ios-sim` archive goes straight into the xcframework. (If you ever
re-add Intel-Mac support, the arm64 + x86_64 simulator archives would first need
merging into one fat `.a` with `lipo`, since "iOS Simulator" is a single
platform variant.) Device (`aarch64-apple-ios`) is a *separate* variant and
stays its own slice — you must **not** lipo device + simulator together (that's
the classic "building for iOS but linking for iOS Simulator" error).

### 4.3 Generate the Swift binding + assemble headers

`uniffi-bindgen … --language swift` emits three files:

| File                                  | Role                                              |
| ------------------------------------- | ------------------------------------------------- |
| `NativeblocksCoreEngine.swift`        | the typed Swift API you call                      |
| `NativeblocksCoreEngineFFI.h`         | the C header for the raw FFI symbols              |
| `NativeblocksCoreEngineFFI.modulemap` | tells the compiler the header forms a Clang module |

The module name (`NativeblocksCoreEngine`) is set in `uniffi.toml`
(`[bindings.swift] module_name`); without it the files default to
`nativeblocks_core_sdk*`. The Swift file imports the FFI module defined by the
modulemap. For an
`.xcframework`, each slice needs a **headers directory** whose module map is
named exactly `module.modulemap` — so the script copies the header and copies the
modulemap **renamed**:

```bash
cp …FFI.h         dist/ios/headers/
cp …FFI.modulemap dist/ios/headers/module.modulemap
```

(If it isn't renamed you get `module 'NativeblocksCoreEngineFFI' not found`.)

### 4.4 Build the `.xcframework`

```bash
xcodebuild -create-xcframework \
  -library <device>.a       -headers dist/ios/headers \
  -library <sim-arm64>.a    -headers dist/ios/headers \
  -output NativeblocksCoreSdk.xcframework
```

The `.xcframework` is a single bundle holding both slices + headers. Xcode picks
the device slice when building for a phone and the simulator slice when building
for the simulator — automatically.

### 4.5 Linking in the app

Add the `.xcframework` to the target as **Do Not Embed** (static libs are linked
into your binary, not embedded as a separate dynamic framework), and add the
`.swift` file to the target's sources.

---

## 5. Flutter, step by step

What `scripts/build-flutter.sh` does. Flutter uses **Dart FFI**, which loads a
**dynamic** library at runtime — so it uses the `cdylib` (not the iOS static lib).

### 5.1 Build the dynamic libraries

```bash
# Android: same cdylib .so per ABI as the native Android path
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -t x86 \
  -o ./dist/flutter/android/jniLibs build --release
# iOS (on macOS): cdylib for the device
cargo build --release --target aarch64-apple-ios
```

### 5.2 Generate the Dart binding

`uniffi-bindgen-dart generate …` (a separate generator from the Kotlin/Swift one,
hence the extra `cargo install`) emits `bindings/dart/nativeblocks_core_sdk.dart`.
It opens the native library via `dart:ffi`'s `DynamicLibrary`.

### 5.3 Stage into an FFI plugin

A Flutter plugin created with `flutter create --template=plugin_ffi` expects:

| File                        | Location in the plugin                          |
| --------------------------- | ----------------------------------------------- |
| Dart binding                | `<plugin>/lib/`                                 |
| Android `.so` per ABI       | `<plugin>/android/src/main/jniLibs/<abi>/`      |
| iOS dynamic library         | embedded in `<plugin>/ios/` (Podspec/framework) |

On Android the `.so` under `jniLibs` is found by name automatically. On **iOS**
the symbols must be present in the app process — this is the fiddliest part:
either embed the `cdylib` as a dynamic framework via the plugin's podspec, or
static-link and load with `DynamicLibrary.process()`. The exact load call lives
at the top of the generated `nativeblocks_core_sdk.dart` — check it for the
library name/path it expects, as this varies by `uniffi-bindgen-dart` version.

> The architecture doc mentions `flutter_rust_bridge` as an alternative Flutter
> codegen. It's a different toolchain (not wired here); the repo standardizes on
> `uniffi-bindgen-dart` so all three platforms share one `#[uniffi::export]`
> surface.

---

## 6. Binding with the host

The core is headless and calls **up** to host-provided callbacks (e.g. logging).
You implement the generated callback interface in the host language and pass the
instance into the API.

**Kotlin**

```kotlin
import io.nativeblocks.core.engine.Logger
import io.nativeblocks.core.engine.LoggerEventLevel

class LogcatLogger : Logger {
    override fun log(level: LoggerEventLevel, event: String,
                     message: String, parameters: Map<String, String>) {
        android.util.Log.d("NB/$event", "$message $parameters")
    }
}
```

**Swift**

```swift
import nativeblocks_core_sdk

final class OSLogLogger: Logger {
    func log(level: LoggerEventLevel, event: String,
             message: String, parameters: [String: String]) {
        print("NB/\(event): \(message) \(parameters)")
    }
}
```

**Dart** — implement the generated `Logger` class and pass it into the API the
same way (see the generated `nativeblocks_core_sdk.dart` for the exact type).

---

## 7. Keeping libs and bindings in sync

The single most common deployment bug is a **version mismatch** between the
compiled library and the generated binding. Rules:

- Treat the `(.so/.a/.dylib, binding file)` pair as **atomic** — always ship both
  from the same `release.sh` run.
- Any change to the Rust public surface ⇒ re-run `./scripts/release.sh` and
  re-copy every platform folder.
- Don't hand-edit `bindings/` or `dist/` — they're regenerated and `dist/` is
  git-ignored.

---

## 8. Troubleshooting

| Symptom | Cause / fix |
| ------- | ----------- |
| `UnsatisfiedLinkError` / `library "…so" not found` (Android) | The device's ABI `.so` is missing. Ensure all four ABIs are under `jniLibs/`. |
| `ClassNotFoundException: com.sun.jna.*` | Add `net.java.dev.jna:jna:5.14.0@aar`. |
| `building for iOS but linking for iOS Simulator` | Device and simulator libs got mixed. Keep them as **separate** xcframework slices; only `lipo` the two *simulator* arches together. |
| `module 'NativeblocksCoreEngineFFI' not found` (iOS) | The headers dir's module map must be named `module.modulemap`. |
| Dart can't find the library at runtime | Check the load line at the top of the generated `.dart`; ensure the `.so`/framework is bundled and named as expected. |
| Calls crash / return garbage after a Rust change | Stale binding vs library. Re-run `release.sh`; never mix versions. |
| Binary too large | Confirm `--release` (size profile). Audit with `cargo bloat --release --crates`. Drop built-ins with `--no-default-features` to supply a host transport/cache. |
| `release.sh` skips a platform | A toolchain is missing — read the printed hint (e.g. `cargo install cargo-ndk`, or iOS needs macOS+Xcode). |
