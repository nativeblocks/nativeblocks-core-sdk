# nativeblocks-core-sdk

Client **core SDK** for NativeBlocks, written in Rust. It is the headless engine
that fetches frames/scaffold/localization, caches them, resolves variables, and
manages frame state. It ships as a single cross-platform core that mobile apps
(Android, iOS, Flutter, React Native) consume over FFI — **rendering stays
per-platform**; the engine never renders, it owns all non-UI logic.

## Architecture

```
            ┌─────────────────────────────┐
            │   Rust core (this crate)     │   ← frames, cache, state, logic
            │   #[uniffi::export] surface  │
            └──────────────┬──────────────┘
                           │  UniFFI
        ┌──────────────────┼──────────────────┐
        ▼                  ▼                  ▼
   Kotlin binding     Swift binding       Dart binding
   (Android)          (iOS)               (Flutter)
        │                  │                  │
   Android app        iOS app            Flutter app
   (rendering + platform code written on top)
```

The library is built as `cdylib` (Android `.so`, Flutter), `staticlib` (iOS
xcframework), and `lib` (the bindgen binary + Rust tests). FFI bindings are
generated from the compiled library — they are build artifacts, not committed.

## Setup

```bash
# Build (debug) with the default features (reqwest transport + bundled SQLite).
cargo build

# Run the test suite.
cargo test
```

> The first build compiles bundled SQLite from source, so it needs a C compiler
> (`cc`/Clang) on `PATH` (default on macOS/Linux).

## Deploy

Run one script, then copy the generated folders into your host apps. For _why_
each step works the way it does, see
**[docs/deployment-internals.md](docs/deployment-internals.md)**.

### 1. One-time setup (per machine)

```bash
rustup target add \
  aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android \
  aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios

cargo install cargo-ndk                              # + install the Android NDK
```

### 2. Build a release

```bash
./scripts/release.sh                  # all platforms your machine supports
# or: ./scripts/release.sh android ios flutter
```

This cleans `dist/`, builds the Rust core per platform, generates the bindings,
and stages everything under `dist/`. Toolchains you don't have are **skipped with
a hint** (e.g. iOS off macOS), not errors. The layout:

```
dist/
├── android/  jniLibs/<abi>/libnativeblocks_core_sdk.so + java/io/nativeblocks/core/engine/nativeblocks_core_sdk.kt
├── ios/      NativeblocksCoreSdk.xcframework/ + NativeblocksCoreEngine.swift
└── flutter/  android/jniLibs/… + ios/… + lib/nativeblocks_core_sdk.dart
```

Each `dist/<platform>/COPY-INSTRUCTIONS.txt` repeats the copy steps next to the files.

### 3. Copy into the host project

**Android**

| Copy from                | Into                         |
| ------------------------ | ---------------------------- |
| `dist/android/jniLibs/*` | `<module>/src/main/jniLibs/` |
| `dist/android/java/*`    | `<module>/src/main/java/`    |

Add once to the module's `build.gradle.kts`, then `./gradlew :app:installDebug`:

```kotlin
dependencies {
    implementation("net.java.dev.jna:jna:5.14.0@aar")
}
```

**iOS** — then run from Xcode (`⌘R`):

| Add to the Xcode target                    | How                         |
| ------------------------------------------ | --------------------------- |
| `dist/ios/NativeblocksCoreSdk.xcframework` | drag in → **Do Not Embed**  |
| `dist/ios/NativeblocksCoreEngine.swift`    | add to the target's sources |

**Flutter** — then run `flutter run`:

| Copy from                        | Into                                         |
| -------------------------------- | -------------------------------------------- |
| `dist/flutter/android/jniLibs/*` | `<plugin>/android/src/main/jniLibs/`         |
| `dist/flutter/lib/*`             | `<plugin>/lib/`                              |
| `dist/flutter/ios/*`             | embed in `<plugin>/ios/` (see internals doc) |

> Every release: re-run `./scripts/release.sh` and re-copy — a Rust change moves
> the native libs _and_ the bindings together. Single-platform rebuilds:
> `scripts/build-android.sh`, `scripts/build-ios.sh`, `scripts/build-flutter.sh`.
