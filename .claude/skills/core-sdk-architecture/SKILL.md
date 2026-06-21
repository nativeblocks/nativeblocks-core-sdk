---
name: core-sdk-architecture
description: Architecture and technical constraints for the nativeblocks-core-sdk Rust engine. Read this BEFORE writing or reviewing any code in this repo — it defines the feature-package layout, the Client boundary between features, the headless engine⇄UI contract, the chosen crates, the app-size rules, and the UniFFI/FFI conventions. Applies whenever implementing or changing a feature (frame, scaffold, localization, …) or the FFI surface.
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
   resolution, action sequencing, and the **generated install id** — see below).
   Truly platform-specific concerns (logging, secure storage,
   navigation/dialog/contractor events, locale reads) are **host-provided callback
   traits** passed in at init.

   **Install id is core-owned**, not host-provided (parity with Kotlin
   `ConfigRepositoryImpl.installId()`). The `config` feature generates a UUID v7 once,
   persists it via the `CacheProvider` (SQLite `cache` table at the host's `db_path`),
   and returns it inside `ResolvedGateway` so every feature reads it from there — the FFI
   methods do **not** take an `install_id` parameter. The `uuid` crate (v7 feature)
   provides the id; `CacheProvider` is injected into `config::get_or_create` by the FFI
   composition root (the same place `new_http_client()` is built), keeping `config`
   feature-agnostic.

## Feature-package architecture (the core rule)

Mirror the Go sample in `exm/auth/`. The codebase is **feature-based**: each feature is
its own package/module that owns its models and its data layer, and exposes itself to
the outside world **only through a `Client` interface**. Features never reach into each
other's internals — a feature that needs another feature calls that feature's `Client`.

The Go pattern (`exm/auth/`) and its Rust translation:

| Go (`exm/auth/`)                                              | Rust (this repo)                                                                                    |
| ------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `client.go` — `Client` interface, `clientImpl`, `NewClient()` | `client.rs` — `pub(crate) struct Client` with `::new` (a `trait` only if a second impl exists); `di.rs` hands out `Arc<Client>` |
| `model.go` — feature's own structs                            | `model.rs` — `pub` serde structs (the wire contract)                                                |
| `auth.go` — internal business logic (unexported fns)          | internal modules — `pub(crate)`/private fns, the data layer + use cases                             |
| `key.go` — constants (error keys)                             | `key.rs` — `const` keys / error message constants                                                   |
| cross-feature call: `admin.NewClient()`                       | cross-feature call: `other_feature::new_client(...)`                                                |

**Rules:**

- A feature owns its full vertical: models → DTO/entity → data source → repository →
  use case → client. No shared "god" layer reaching across features.
- One feature depends on another **only** via that feature's `Client` trait +
  constructor, never by importing its internal modules.
- Constructors return a trait object/handle (`Arc<dyn Client>`), so impls stay private
  and swappable.
- Don't write a comment for a function unless you find it necessary, and don't use any special character
- Always use an explicit `return` keyword for a function's/method's returned value — write
  `return expr;`, never a bare tail expression. (This is a house style; it trips clippy's
  `needless_return`, so that lint stays off.)
- Don't write tests. This repo does not keep a test suite — no `#[test]`, no
  `*.test.rs`, no `#[cfg(test)]` modules or test-only helpers.

### Module boundary & visibility (strict — applies to every module)

Every module — each feature **and** the shared `common` submodules — exposes to the
rest of the crate **only two kinds of item: its `Client` trait (plus the constructor
that returns it) and its models.** Everything else is private to the module.

- **Open (may be re-exported from the module root):**
  - the `Client` trait and the constructor returning `Arc<dyn Client>`;
  - the module's models — the serde/wire structs and enums that appear in the trait's
    method signatures or cross the FFI boundary.
- **Hidden (module-private; `pub(crate)` or private, never re-exported from the root):**
  `ClientImpl` and every concrete impl, repositories, data sources, use-cases, mappers,
  DTOs, GraphQL query strings, `key`/constants, request/param structs, helper functions.
  Submodules holding these (`data`, `domain`, `key`, `graphql`, `script`) are declared
  `mod x;`, never `pub mod x;`.
- **The implementation is invisible.** Callers receive `Arc<dyn Client>` and may only
  call trait methods; `ClientImpl` is private to the module.
- **Every `Client` method is `async` and thread-safe** (the trait is `Send + Sync`), so
  it is safe to call from other modules and from the generated FFI layer with no
  external locking.
- **Need to share a capability? Put it on the `Client`.** If another module or the FFI
  layer needs something, expose it as a `Client` method (or a model it returns) — never
  by making the internal item `pub`. Reaching into another module's `key` / `data` /
  `graphql` / impl is forbidden; go through its `Client`.
- **Visibility tiers:** items used only inside the crate (Client types, constructors,
  request structs, cross-module models that never reach FFI) are `pub(crate)`. Only the
  generated FFI objects (`#[derive(uniffi::Object)]`) and the types in their signatures
  are `pub` — that uniffi surface is the true external boundary, not the Rust traits.
- **`common` is not exempt.** Its shared models are `NbResult`/`ErrorModel`/`NbError`/
  `ErrorType` and `SdkConfig`/`NativeblocksEnvironment`/`ProjectConfigGateway`; its
  "clients" are the service traits (`HttpClient`, logger, cache source) whose concrete
  impls (reqwest, SQLite) are private behind constructors (`new_http_client()`, …).
  **One unavoidable exception:** generic helpers that are not object-safe
  (`execute_graphql<D>`, `decode_envelope<D>` — generic over the response type, so they
  can never be `dyn`-dispatched trait methods) remain crate-internal (`pub(crate)`) free
  functions in `common::net`. They are still not part of any module's public surface.

The module-root re-export therefore looks like:

```rust
pub(crate) use client::Client;                 // the gate (concrete struct; trait only if 2nd impl)
pub(crate) use di::get_or_create;              // DI builds + caches the per-instance Arc<Client>
pub use ffi::FeatureClient;                     // the uniffi object = real external boundary
pub use model::{ /* only FFI-facing models */ };
pub(crate) use model::{ /* models used cross-module but not across FFI */ };
// nothing else — no key, no graphql, no data, no request structs
```

### Per-feature file layout (convention)

```
src/<feature>/
  mod.rs        // wiring; re-exports ONLY Client + models (+ FFI object). get_or_create from di.
  client.rs     // pub(crate) struct Client with pub(crate) ::new + methods (trait only if 2nd impl)
  di.rs         // wires source -> repository -> client; owns the per-instance registry of Arc<Client>
  ffi.rs        // #[derive(uniffi::Object)] FeatureClient — the real external boundary
  model.rs      // serde structs = the feature's wire contract (mod model; selectively re-exported)
  key.rs        // const keys / error constants — mod key; (private, never re-exported)
  graphql.rs    // query strings — mod graphql; (private)
  domain/       // use cases = pure async fns, one job per file, named <verb>_use_case — private
  data/         // dto, mappers, sources (cache/net), repository — private
```

Keep names and granularity proportional to the feature. Small features may collapse
`data/` into a single file; large ones (frame) split it.

### Layering inside a feature (client → use case → repository → source)

- **One job per client method.** Don't split a single capability into a fetch-then-use
  pair across the FFI; the client exposes the finished result. (`config` does not expose
  `project_config` + `gateway_for`; it exposes one `gateway_for(install_id, operation)
  -> ResolvedGateway` that resolves end to end.)
- **A trait earns its place only when the concrete type must vary; otherwise plain struct.**
  Hiding is NOT a reason for a trait — privacy does that: a `pub(crate) struct` with
  private fields and `pub(crate)` methods, handed out by `di.rs`, is fully opaque to
  callers. A trait buys exactly one thing — **polymorphism** (callers hold `dyn`, the impl
  can swap). Use it only where a second / swappable / feature-gated implementation really
  exists: `HttpClient` (reqwest, could be ureq/fake), a `#[cfg(feature)]` cache source.
  - The **`Client` gate is a concrete `pub(crate) struct`** by default (one impl, no test
    fakes). `di.rs` returns `Arc<Client>`; callers hold `Arc<feature::Client>`, not
    `Arc<dyn …>`. Promote it to a trait only when a real second impl appears (e.g.
    Cloud vs Community) — a 5-minute change.
  - The **repository and network source are plain structs** too — single impl, never
    crossed by another module. No `trait` + `…Impl` + `new_…` triple, no `Arc<dyn …>`
    for something there's only one of.
- **Use cases are pure functions** (functional style — no struct, no impl), one job per
  file in `domain/`, named with a `_use_case` suffix (`resolve_gateway_use_case`). They
  take their dependencies as parameters — inline DI at the call site — e.g.
  `resolve_gateway_use_case(repository: &ProjectConfigRepository, install_id, operation)`.
- **The client forwards each trait call to a pure use-case function**, passing the
  dependencies it was built with. `ClientImpl` holds those dependencies (e.g. the
  repository handle) and nothing else. "Don't expose the repository" means it stays
  `pub(crate)` and is never re-exported across the module / FFI boundary — not that the
  in-feature client can't hold the handle to feed a use case.
- **`di.rs` is the only place the dependency graph is assembled.** It builds
  `source -> repository -> client`, and owns the registry that caches one client per
  instance id (`environment.instance_name()`). `get_or_create` lives here, not in
  `client.rs`. This is the replacement for the old ad-hoc `OnceLock` registry.

### Multi-instance (parity with Kotlin `NativeblocksManager`)

The SDK supports multiple named instances, exactly like the Kotlin
`NativeblocksManager.initialize(name, …)` / `getInstance(name)` registry. The instance
identity is **first-class and separate from the edition**, mirroring Kotlin's
`name` + `NativeblocksEdition`:

```rust
pub struct NativeblocksEnvironment { pub instance_name: String, pub edition: NativeblocksEdition }
pub enum   NativeblocksEdition     { Cloud { endpoint, api_key, development_mode }, Community { frames_data } }
```

- `instance_name` is **not** duplicated inside the edition variants — it lives once on the
  environment. All per-instance registries (config gateway, logger, …) key off
  `environment.instance_name()`.
- The name is **validated** at every FFI constructor (`environment.validate()?`) with the
  same rule as Kotlin's `isValidInstanceName`: non-blank and `^[A-Za-z0-9_-]+$`.
- All edition matching is centralized on `NativeblocksEnvironment` methods
  (`endpoint()`, `api_key()`, `development_mode()`, `is_community()`) — features never
  `match` the enum themselves.

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
// + small platform reads: locale, … (install id is core-owned, not a host read)
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

Each exposes a `Client` (a concrete `pub(crate) struct` by default); shared concerns (network result type, config gateway
model, cache) live in `common` and obey the same boundary rule — consumed via `common`'s
models and service-trait "clients", never by reaching into their impls.

## Parity

Behavior must match the Kotlin engine (sync algorithm, checksum caching, variable
resolution, JSONPath, script evaluation, action sequencing). Parity is verified
manually against the Kotlin reference under `exm/core/`, not via an in-repo automated
test suite — this project does not keep tests in the repo.

## Current state of the repo

Single crate `nativeblocks_core_sdk` (`Cargo.toml`, edition 2024), `crate-type =
["lib", "cdylib", "staticlib"]`, with a working UniFFI binding pipeline
(`src/bin/uniffi-bindgen.rs`, `scripts/generate-bindings.sh`, generated
`bindings/{kotlin,swift,dart}`). All five features — `config`, `scaffold`, `experiment`,
`localization`, `frame` — are built out under `src/<feature>/` and follow the template
above (concrete `Client` struct, plain-struct data layer, `di.rs`, pure-fn use cases).
The multi-crate workspace split (`nb-models`, `nb-core`, `nb-net`, `nb-cache`,
`nb-script`, `nb-ffi`) described in the export guide is the eventual target; promote
feature modules to crates when boundaries are proven.

### Frame (the stateful engine) — template deviations

`frame` is the one stateful feature, so it adapts the template:

- The `Client` IS the engine — a concrete `pub(crate) struct Client` holding the
  `watch` channels (`frame_state`/`blocks`/`variables`/`actions`/`generation`), action
  handlers, and route state. It **calls the repository directly**; there is no
  pure-fn `domain/` layer, because the engine itself is the application/orchestration
  layer (action sequencing, scripting, state application). Forcing thin use-case
  forwarders onto it would be ceremony.
- `data/` splits into `remote.rs` (`FrameRemoteSource`, plain struct — network),
  `source.rs` (`FrameLocalSource` **trait** + SQLite — the feature-gated boundary), and
  `repository.rs` (`FrameRepository`, plain struct — checksum-gated sync, the per-route
  stream cache, global params).
- The reactive `observe_*` stream accessors were **removed** as dead code: a
  `watch::Receiver` can't cross the UniFFI boundary, so the FFI exposes the snapshot
  getters (`native_frame_state`, `blocks_state`, …) instead. The `watch::Sender`
  channels remain internally (the snapshot getters read them via `.borrow()`). Re-add
  stream accessors only when a streaming FFI is actually wired.
- `NativeActionHandler` stays a trait — it's a host-implemented UniFFI callback
  interface (`#[uniffi::export(with_foreign)]`), a real cross-boundary abstraction.
