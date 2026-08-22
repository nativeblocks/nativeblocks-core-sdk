# nativeblocks-core-sdk

The Rust core of the NativeBlocks runtime. It ships inside the Android and iOS host SDKs over UniFFI bindings that are
generated, never committed.

## Setup

```bash
cargo build     # debug, default features (reqwest transport + bundled SQLite)
cargo test
```

> The first build compiles bundled SQLite from source, so it needs a C compiler
> (`cc`/Clang) on `PATH` (default on macOS/Linux).

## Deploy

### 1. One-time setup (per machine)

```bash
rustup target add \
  aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android \
  aarch64-apple-ios aarch64-apple-ios-sim

cargo install cargo-ndk                              # + install the Android NDK
```

### 2. Build a release

```bash
./scripts/release.sh                  # all platforms your machine supports
# or: ./scripts/release.sh android ios
```

This cleans `dist/`, builds the core per platform, generates and seals the bindings, and stages everything under
`dist/`. Toolchains you don't have are **skipped with a hint** (e.g. iOS off macOS), not errors.

```
dist/
├── android/  jniLibs/<abi>/libnativeblocks_runtime.so
│             java/io/nativeblocks/runtime/ffi/NativeblocksRuntime.kt
└── ios/      NativeblocksRuntimeCFFI.xcframework/ + NativeblocksRuntimeFFI.swift
```

Single-platform rebuilds: `scripts/build-android.sh`, `build-ios.sh`. Each
`dist/<platform>/COPY-INSTRUCTIONS.txt` repeats the copy steps next to the files.

### 3. Copy into the host SDK

Every release: re-run `./scripts/release.sh` and re-copy — a Rust change moves the native libs _and_ the bindings
together. Pass `--sync-platforms` to skip the manual copy: it drops the freshly built artifacts straight into
`platforms/android/runtime/core/` and `platforms/ios/runtime/` (only for the platforms that actually built).

**Android** — both folders go into the **same** Gradle module as the hand-written wrapper (the binding is `internal`,
which is module-scoped):

| Copy from                | Into                         |
|--------------------------|------------------------------|
| `dist/android/jniLibs/*` | `<module>/src/main/jniLibs/` |
| `dist/android/java/*`    | `<module>/src/main/java/`    |

```kotlin
// <module>/build.gradle.kts
kotlin { explicitApi() }
dependencies { implementation("net.java.dev.jna:jna:5.14.0@aar") }
```

**iOS** — one SwiftPM package, two targets (the binding is `package`-visible, so it must live in the same package as the
wrapper). Xcode 15+:

| Copy from                                      | Into                              |
|------------------------------------------------|-----------------------------------|
| `dist/ios/NativeblocksRuntimeCFFI.xcframework` | `Frameworks/` → **Embed & Sign**  |
| `dist/ios/NativeblocksRuntimeFFI.swift`        | `Sources/NativeblocksRuntimeFFI/` |

```swift
products: [
    .library(name: "NativeblocksRuntime", targets: ["NativeblocksRuntime"]), // the only product
],
targets: [
    .target(name: "NativeblocksRuntime", dependencies: ["NativeblocksRuntimeFFI"]),
    .target(name: "NativeblocksRuntimeFFI", dependencies: ["NativeblocksRuntimeCFFI"]),
    .binaryTarget(name: "NativeblocksRuntimeCFFI",
                  path: "Frameworks/NativeblocksRuntimeCFFI.xcframework"),
]
```

## Host SDK boundary

App developers depend on the host SDK and must never name anything UniFFI generated. Two things enforce that, by the
compiler rather than by review.

**Names.** Everything generated carries an FFI postfix, leaving the clean names free for the host's public API:

|         | generated (private)                                           | host (public)             |
|---------|---------------------------------------------------------------|---------------------------|
| Android | package `io.nativeblocks.runtime.ffi`                         | `io.nativeblocks.runtime` |
| iOS     | `NativeblocksRuntimeFFI` + `NativeblocksRuntimeCFFI` (C shim) | `NativeblocksRuntime`     |
