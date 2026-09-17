# Deploying the iOS SDK

Publishes one SwiftPM package, `nativeblocks.sdk`. It contains `NativeblocksRuntime`,
`NativeblocksFoundation` and `NativeblocksDevkit` as XCFrameworks, and
`NativeblocksCompiler` plus its plugins as source. The XCFrameworks go to
`dist.nativeblocks.io` and the Swift Package Registry to `spm.nativeblocks.io`.
Run every step from the repo root, in this order.

After steps 0–2, steps 3–6 run as one command: `deploy/release.sh ios` (add `--dry-run` to skip the upload).

## 0. One-time setup

- Tools: Xcode, `swift`, `xccache` (`gem install xccache`), Rust with the iOS targets,
  `aws` CLI, `curl`, `python3`, `zip`.
- `cp deploy/.env.example deploy/.env` and fill in `R2_URL`, `R2_BUCKET`,
  `R2_ACCESS_KEY_ID`, `R2_SECRET_ACCESS_KEY`, `DIST_PUBLIC_URL`, `SWIFT_PUBLIC_URL`.
- The registry Worker (`deploy/worker/index.js`) must be deployed on
  `spm.nativeblocks.io` and bound to the same bucket. Only needed once, or when the
  Worker changes.
- `swift package resolve` in `platforms/ios/compiler` so `Package.resolved` pins swift-syntax.

## 1. Set the version

Edit `versions.toml`:

```toml
ios  = "0.2.0"   # the release you are cutting
core = "0.1.0"   # must match runtime/Cargo.toml
```

A published version is immutable, so always bump. Don't overwrite one.

## 2. Build the Rust core and copy it in

```sh
runtime/scripts/release.sh ios
```

Then copy the output by hand:

| From | To |
|---|---|
| `runtime/dist/ios/NativeblocksRuntimeCFFI.xcframework` | `platforms/ios/runtime/Frameworks/` |
| `runtime/dist/ios/NativeblocksRuntimeFFI.swift` | `platforms/ios/runtime/Sources/NativeblocksRuntimeFFI/` |

Skip this step if the core hasn't changed since the last copy.

## 3. Check versions

```sh
deploy/check-version.sh ios/v0.2.0
```

Fails if the tag, `versions.toml`, `SDKConfig.swift` or `Cargo.toml` disagree.

## 4. Build

```sh
deploy/build-ios.sh
```

Syncs `SDKConfig.swift`, builds the three XCFrameworks, link-checks every slice and
stages a local package at `deploy/build/ios/<version>/Nativeblocks`. Nothing is uploaded.
Use `--skip-build` to reuse existing `platforms/ios/*/output`.

## 5. Verify

```sh
deploy/verify-ios.sh
```

Resolves the staged package and builds `NativeblocksFoundation` for a device, with the
macro. It also checks that the Rust FFI module isn't importable.

## 6. Deploy

```sh
deploy/deploy-ios.sh --dry-run   # build archive + registry metadata, upload nothing
deploy/deploy-ios.sh             # upload and check the public URLs
```

This regenerates `Package.swift` with remote `url:` + `checksum:` binary targets and
builds the source archive. Uploads go in order: binaries, archive, manifest, metadata,
and the release list last. It refuses if the version already exists (`--force`
overrides; avoid it).

## 7. Tag

```sh
git tag ios/v0.2.0 && git push origin ios/v0.2.0
```

Consumers then run `swift package-registry set https://spm.nativeblocks.io` and
depend on `.package(id: "nativeblocks.sdk", exact: "0.2.0")`.
