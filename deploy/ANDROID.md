# Deploying the Android SDK

Publishes `runtime-android`, `foundation-android`, `compiler-android`, `devkit-android`,
`gradle-plugin-android` and `bom-android` to the Maven repository on R2
(`maven.nativeblocks.io`). Run every step from the repo root, in this order.

After steps 0–2, steps 3–6 run as one command: `deploy/release.sh android` (add `--dry-run` to skip the upload).

## 0. One-time setup

- Tools: `java` 17, Android SDK (`ANDROID_HOME`), `cargo-ndk`, `aws` CLI, `curl`, `python3`.
- `cp deploy/.env.example deploy/.env` and fill in `R2_URL`, `R2_BUCKET`,
  `R2_ACCESS_KEY_ID`, `R2_SECRET_ACCESS_KEY`, `MAVEN_PUBLIC_URL`.

## 1. Set the version

Edit `versions.toml`:

```toml
android = "0.2.0"   # the release you are cutting
core    = "0.1.0"   # must match runtime/Cargo.toml
```

A published version is immutable, so always bump. Don't overwrite one.

## 2. Build the Rust core and copy it in

```sh
runtime/scripts/release.sh android
```

Then copy the output by hand:

| From | To |
|---|---|
| `runtime/dist/android/jniLibs/*` | `platforms/android/runtime/core/src/main/jniLibs/` |
| `runtime/dist/android/java/*` | `platforms/android/runtime/core/src/main/java/` |

Skip this step if the core hasn't changed since the last copy.

## 3. Check versions

```sh
deploy/check-version.sh android/v0.2.0
```

Fails if the tag, `versions.toml`, the Gradle catalogs or `Cargo.toml` disagree.

## 4. Build

```sh
deploy/build-android.sh
```

Syncs the version catalogs, then publishes compiler → runtime → foundation → devkit
to `mavenLocal` and to the staging repo at `deploy/build/maven`, and generates the BOM.
Nothing is uploaded.

## 5. Verify

```sh
deploy/verify-android.sh
```

Builds a throwaway app against `deploy/build/maven` only (no `mavenLocal`, clean
Gradle cache). It checks that KSP ran, all ABIs are in the APK, and the 64-bit
`.so` files use 16 KB page alignment.

## 6. Deploy

```sh
deploy/deploy-android.sh --dry-run   # show what would upload
deploy/deploy-android.sh             # upload and check the public URLs
```

Refuses if the version already exists on R2 (`--force` overrides; avoid it).
Uploads only what step 4 staged; it never rebuilds.

## 7. Tag

```sh
git tag android/v0.2.0 && git push origin android/v0.2.0
```
