---
name: core-sdk-architecture
description: Architecture and technical constraints for the nativeblocks-core-sdk Rust engine. Read this BEFORE writing or reviewing any code in this repo — it defines the feature-package layout, the Client-trait boundary between features, the headless engine⇄UI contract, the chosen crates, the app-size rules, and the UniFFI/FFI conventions. Applies whenever implementing or changing a feature (frame, scaffold, localization, …) or the FFI surface.
---

# nativeblocks-core-sdk — architecture & constraints

## What this SDK is

One **Rust** engine, many native UIs. It extracts the DB / network / business-logic
core of the Nativeblocks engine into a single Rust library that compiles to a native
artifact per platform and runs unchanged behind Android, iOS, Flutter, and React
Native. **Rendering stays per-platform** (Compose / SwiftUI / Flutter widgets / RN
components). The engine never renders — it owns all non-UI logic and outputs _state_.

- Distribution: Android `.aar` (JNI), iOS `.xcframework`, Flutter via
  `flutter_rust_bridge`, RN via `uniffi-react-native`.
- Web React is **out of scope** — it keeps its own TS engine (a browser can't consume
  `.aar`/`.framework`). A future `wasm32` build _could_ reach it; not a goal now.
- Source of truth for the full rationale: `multiplatform-engine-export.md` at repo root.
  The Kotlin reference engine to port lives under `exm/core/` (read it for behavior; it
  is the behavioral spec, not a structure to copy verbatim).

## Hard constraints (do not violate without explicit sign-off)

1. **App size is priority #1.** Rust chosen because it has no runtime / no GC. Every
   dependency is binary weight — justify each crate, prefer lean ones, audit with
   `cargo bloat`. Release profile must use `opt-level = "z"`, `lto = true`,
   `codegen-units = 1`, `panic = "abort"`, `strip = true`.
2. **The engine is headless.** No rendering, no `@Composable`/SwiftUI/widget concepts in
   core. It emits a serializable state tree and accepts events. Nothing platform-UI
   ever enters a core crate.
3. **FFI is generated, never hand-written.** UniFFI for Kotlin/Swift,
   `flutter_rust_bridge` for Dart, `uniffi-react-native` for RN. Public items are
   exposed with `#[uniffi::export]` (see `src/lib.rs`,
   `uniffi::setup_scaffolding!()`). Regenerate via `scripts/generate-bindings.sh`;
   never edit files under `bindings/` by hand.
4. **Keep the FFI boundary coarse.** Emit whole frame-state snapshots (diffed), not
   chatty per-variable getters. Every crossing is serialized and costs — a
   per-variable read pattern dies by a thousand crossings.
5. **Platform I/O is delegated up, not reached down.** Core owns deterministic /
   local-I/O work (SQLite cache, file cache, checksums, variable & JSONPath
   resolution, action sequencing). Truly platform-specific concerns (logging, secure
   storage, navigation/dialog/contractor events, locale/install-id reads) are
   **host-provided callback traits** passed in at init.

## Feature-package architecture (the core rule)

Mirror the Go sample in `exm/auth/`. The codebase is **feature-based**: each feature is
its own package/module that owns its models and its data layer, and exposes itself to
the outside world **only through a `Client` interface**. Features never reach into each
other's internals — a feature that needs another feature calls that feature's `Client`.

The Go pattern (`exm/auth/`) and its Rust translation:

| Go (`exm/auth/`)                                              | Rust (this repo)                                                                                    |
| ------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `client.go` — `Client` interface, `clientImpl`, `NewClient()` | `client.rs` — `pub trait Client`, private `ClientImpl`, `pub fn new_client(...) -> Arc<dyn Client>` |
| `model.go` — feature's own structs                            | `model.rs` — `pub` serde structs (the wire contract)                                                |
| `auth.go` — internal business logic (unexported fns)          | internal modules — `pub(crate)`/private fns, the data layer + use cases                             |
| `key.go` — constants (error keys)                             | `key.rs` — `const` keys / error message constants                                                   |
| cross-feature call: `admin.NewClient()`                       | cross-feature call: `other_feature::new_client(...)`                                                |

**Rules:**

- The `Client` trait is the **only** public surface of a feature. Models referenced by
  the trait are public; everything else (data sources, mappers, DTOs, use-case fns) is
  `pub(crate)` or private.
- A feature owns its full vertical: models → DTO/entity → data source → use case →
  client. No shared "god" layer reaching across features.
- One feature depends on another **only** via that feature's `Client` trait +
  constructor, never by importing its internal modules.
- Constructors return a trait object/handle (`Arc<dyn Client>`), so impls stay private
  and swappable (and testable via fakes).
- Don't write a comment for a function unless you find it necessary, and don't use any special character
- Don't put the test in the same file, create a new file and put them there.

### Per-feature file layout (convention)

```
src/<feature>/
  mod.rs        // module wiring; re-exports Client + public models only
  client.rs     // pub trait Client, private ClientImpl, pub fn new_client(...)
  model.rs      // pub serde structs = the feature's wire contract
  key.rs        // const keys / error constants
  data/         // internal data layer: dto, entity, mappers, sources (cache/net), use cases
```

Keep names and granularity proportional to the feature. Small features may collapse
`data/` into a single file; large ones (frame) split it.

## Headless engine ⇄ UI contract

**Engine → UI: observable frame state** (the Rust core publishes these as
`tokio::sync::watch` channels — `watch` is the `StateFlow` replacement):

```
nativeFrameState      : loading / success / error
blocksState           : Map<String, NativeBlockModel>      (the block tree)
variablesState        : Map<String, NativeVariableModel>   (resolved variables)
actionState           : Map<String, List<NativeActionModel>>
frameUpdateGeneration : Int                                (onAppear/onDisappear gen)
```

**UI → Engine: events** (a tiny inbound surface):

```
setup_frame(route, route_arguments)
handle_action(index, action, event_type)
handle_variable(variable, need_to_log)
localize(key) -> String?
```

**Stays UI-side (never in core):** `BlockProps`/`ActionProps` (carry UI callbacks),
block providers, contractor/navigation/dialog actions (core emits these as **outbound
events**, UI interprets). **Moves into core:** logic actions (scripting, network calls).

The `NativeblocksManager` splits: engine half (`syncFrame`, `getExperiment`,
`setGlobalParameters`, `setLocalization`, `getScaffold`, init) → core; UI-registration
half (`provideBlock`, `provideActionContractor`, …) → each platform's UI layer.

## Chosen crates (the four pillars)

- **HTTP:** `reqwest` (async) or `ureq` (blocking, tiny — favors size); decide by
  measurement. TLS: `rustls` (no OpenSSL). GraphQL: `graphql-client` or `cynic` (SDK is
  Apollo/GraphQL today). JSON: `serde`/`serde_json`; JSONPath: `serde_json_path`.
- **State streaming:** `tokio::sync::watch` (= `StateFlow`); `mpsc`/`broadcast` for
  events; `futures::Stream` for derived streams.
- **SQLite cache:** `rusqlite` (`bundled`, or system SQLite to shave size — measure).
  Immutable snapshots: the `im` crate (mirrors `persistentMapOf`).
- **Frame state = actor pattern:** one async task owns the state struct, receives events
  over `mpsc`, mutates, publishes snapshots via `watch`. This replaces
  `NativeFrameViewModel`; the borrow checker makes concurrent event handling safe.
- **Scripting:** QuickJS via `rquickjs` (replaces Rhino). Preserve `NativeScriptAction`'s
  synchronous host fns (`getVariable`, `changeVariable`, `updateBlockProperties`).

## Host-provided callback traits (passed in at init)

```rust
trait Logger      { fn log(&self, level: u8, event: String, message: String, params: Map); }
trait SecureStore { fn get(&self, key: String) -> Option<String>; fn set(&self, k: String, v: String); }
trait HostEvents  { fn emit(&self, event: HostEvent); }   // navigation, dialogs, contractors
// + small platform reads: install-id, locale, …
```

UniFFI **callback interfaces** are the mechanism: the host implements, the core calls up.

## Features to implement (this repo's roadmap)

Port from the Kotlin reference under `exm/core/.../frame/` — but split into independent
feature packages per the architecture above:

- **frame** — fetch/sync frames, cache (checksum-gated), the `FrameEngine` actor +
  state streams, variable/JSONPath resolution, action sequencing, scripting.
  Ref: `frame/domain/repository/FrameRepository.kt`, `presenter/NativeFrameViewModel.kt`.
- **scaffold** — fetch the scaffold (route table) for a project.
  Ref: `frame/domain/repository/ScaffoldRepository.kt`, `data/dto/NativeScaffoldDto.kt`.
- **localization** — sync/get localizations, language-code state, `translate(key)`.
  Ref: `frame/domain/repository/LocalizationRepository.kt`,
  `domain/model/NativeLocalizationModel.kt`.

Each exposes a `Client` trait; shared concerns (network result type, config gateway
model, cache) are themselves features or core utilities consumed via their public APIs.

## Parity / testing

Behavior must match the Kotlin engine. Build a cross-platform **conformance golden-test
suite** (sync algorithm, checksum caching, variable resolution, JSONPath, script
evaluation, action sequencing). The Kotlin `NativeblocksScriptTest` cases become the JS
conformance tests. This is the safety net that lets the Kotlin/Swift engines be retired
once parity holds — write tests alongside each feature, not after.

## Current state of the repo

Single crate `nativeblocks_core_sdk` (`Cargo.toml`, edition 2024), `crate-type =
["lib", "cdylib", "staticlib"]`, with a placeholder `plus`/`minus` in `src/lib.rs` and a
working UniFFI binding pipeline (`src/bin/uniffi-bindgen.rs`,
`scripts/generate-bindings.sh`, generated `bindings/{kotlin,swift,dart}`). Features get
built out under `src/<feature>/` from here. The multi-crate workspace split
(`nb-models`, `nb-core`, `nb-net`, `nb-cache`, `nb-script`, `nb-ffi`) described in the
export guide is the eventual target; start with feature modules in the single crate and
promote to crates when boundaries are proven.
