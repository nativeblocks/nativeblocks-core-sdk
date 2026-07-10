# Exporting the Nativeblocks Engine as a Shared Rust Core

> Goal: **one engine, many UIs.** Extract the DB / network / business-logic core into a single
> **Rust** library that compiles to a native artifact per platform and runs unchanged behind
> Android, iOS, Flutter, and React Native. The **rendering layer stays per-platform** — Jetpack
> Compose on Android, SwiftUI on iOS, Flutter widgets on Flutter, native components on RN — all
> driving the same shared engine.
> Date: 2026-06-20. Baseline: `core` module at SDK `1.8.1`. Supersedes the earlier KMP draft.

---

## Decision

**Build the shared engine in Rust.** Distribute it as an **Android `.aar`** and an **iOS
`.framework`**; Flutter and React Native consume those native artifacts through their native
plugin/module mechanisms. Replace the current Kotlin and Swift engine logic with this core over
time. The UI layers are NOT shared — each platform renders natively against the engine's state.

**Web React is explicitly out of the shared-engine scope** and keeps its own TypeScript engine,
because a browser cannot consume an `.aar`/`.framework`. (A future `wasm32` build of the same
Rust core _could_ reach web, but that is not a goal of this plan.)

### Why Rust (and not KMP / Zig / Go)

The choice was driven by four hard constraints, in priority order:

1. **App size is the #1 priority.** Rust has no runtime and no garbage collector, so it adds the
   least to the binary of any shared-code option. Ranked smallest→largest:
   `Zig ≲ Rust ≪ KMP (+Kotlin/Native runtime on iOS) ≪ Go (+Go runtime & GC on both)`.
   - **Go is disqualified by size** (multi-MB runtime+GC floor on every platform; gomobile is
     also effectively unmaintained) — despite being the most familiar language.
   - **KMP regresses iOS size** by adding the Kotlin/Native runtime where today's native Swift
     engine adds none.
2. **The workload is async + ecosystem-heavy.** The engine needs HTTP/GraphQL, reactive state
   streams, SQLite, and concurrent event-driven state. These are exactly Rust's strengths and
   exactly where **Zig is weakest today** (no `async`/`await` in the language, thin ecosystem,
   manual concurrency). So between the two systems languages, Zig's size edge does not buy back
   its missing async + libraries for _this_ engine. **Rust over Zig.**
3. **Distribution is `.aar` + `.framework`.** This already excludes web from the shared engine,
   which removes the one unique advantage a native core had over KMP (WASM/web reach). Flutter
   (`flutter_rust_bridge`) and RN (`uniffi-react-native`) both wrap native artifacts cleanly.
4. **FFI is handled by generators, not hand-written.** The team has limited FFI experience; the
   binding generators (UniFFI, flutter_rust_bridge) write the glue. See
   [The FFI boundary](#the-ffi-boundary).

> **Open dependency on one measurement:** if a same-shape KMP build's iOS `.framework` size
> turns out to be within budget, KMP would be the lower-skill-barrier option. This plan assumes
> the size measurement favors Rust; run that spike before committing irrevocably (see
> [Phase 0](#phase-0--de-risk-before-committing)).

---

## Why the codebase is ready for this

A scan of all 84 files in `core` shows the business logic is cleanly separated from the UI. The
domain models, use cases, and repository _interfaces_ have **zero platform imports** — they are
the behavioral spec the Rust core must reproduce. Platform coupling is concentrated in ~14 files.

| Layer                                                                                | Files | Today                                   | In the Rust plan                                                                      |
| ------------------------------------------------------------------------------------ | ----- | --------------------------------------- | ------------------------------------------------------------------------------------- |
| `frame/domain/model/*`                                                               | 8     | plain data classes, no platform imports | → Rust `structs` + `serde` (the wire contract)                                        |
| `frame/domain/useCase/*`                                                             | 4     | pure Kotlin + coroutines                | → Rust modules / async fns                                                            |
| `frame/domain/repository/*`                                                          | 5     | interfaces                              | → Rust `trait`s                                                                       |
| `network/*`                                                                          | logic | Apollo + OkHttp + Ktor                  | → `reqwest`/`ureq` + `rustls` + `graphql-client`                                      |
| `cache/ICacheProvider`                                                               | 1     | interface                               | → Rust `trait`, host-provided or in-core                                              |
| `cache/LocalCacheProvider`                                                           | 1     | `SharedPreferences`                     | → in-core file/kv store, or host callback                                             |
| `frame/data/source/local/*`                                                          | 10    | Room (SQLite)                           | → `rusqlite` (bundled SQLite)                                                         |
| `di/*`                                                                               | 6     | Koin                                    | → plain Rust construction / a small registry                                          |
| `NativeFrameViewModel`                                                               | 1     | `androidx.lifecycle.ViewModel`          | → **actor + `watch` channel** (see [State](#4-frame-state-the-viewmodel-replacement)) |
| `NativeScriptAction`                                                                 | 1     | Rhino (JVM)                             | → **QuickJS** embedded in Rust (`rquickjs`)                                           |
| UI: `NativeFrame`, `NativeblocksFrame`, `BlockContext`, providers, `RootBlock`, util | ~12   | Compose                                 | ⛔ **stays per-platform** (not ported)                                                |

---

## Target architecture

```
                       ┌──────────────────────────────────────────────┐
                       │            engine  (Rust crate)               │
                       │  models (serde), use cases, repositories,     │
                       │  network (reqwest+rustls+graphql-client),     │
                       │  SQLite cache (rusqlite), variable/JSONPath    │
                       │  resolution, action sequencing,               │
                       │  QuickJS scripting (rquickjs),                 │
                       │  FrameEngine = actor + watch channel          │
                       │                                               │
                       │  host callbacks (traits): logging, secure     │
                       │  storage, navigation/contractor events        │
                       └───┬──────────────┬──────────────┬─────────────┘
              UniFFI →     │ .aar (JNI)   │ .framework   │ flutter_rust_bridge
                           ▼              ▼              ▼          ▼ uniffi-react-native
                 ┌──────────────────┐ ┌────────────┐ ┌──────────┐ ┌──────────────┐
                 │  Android native  │ │ iOS native │ │ Flutter  │ │ React Native │
                 │  Compose blocks  │ │SwiftUI blks│ │ widgets  │ │ native comps │
                 └──────────────────┘ └────────────┘ └──────────┘ └──────────────┘

   web React  ──►  keeps its own TypeScript engine (out of shared scope)
```

- **`engine`** (Rust) is the single source of truth for all non-UI logic.
- **UI layers** register block/action _renderers_ and subscribe to the engine's state stream.
  They are not shared; each platform renders natively. This is the explicit requirement.
- The engine never renders. It outputs **state**; the UI turns state into widgets.

---

## The headless boundary (engine ⇄ UI contract)

This is the contract that makes "one engine, many UIs" work, and it is **language-agnostic** —
the same contract held for the KMP draft. The engine outputs _state_ (a serializable tree) and
accepts _events_.

**Engine → UI: an observable frame state.** Today's `NativeFrameViewModel` already exposes
exactly the right streams; the Rust core reproduces them as `watch` channels:

```
nativeFrameState      : loading / success / error
blocksState           : Map<String, NativeBlockModel>     (the block tree)
variablesState        : Map<String, NativeVariableModel>  (resolved variables)
actionState           : Map<String, List<NativeActionModel>>
frameUpdateGeneration : Int                               (onAppear/onDisappear gen)
```

All payload types are plain data classes with no platform imports today → direct `serde` structs
in Rust. The UI walks `blocksState` and renders; that walk (`FrameBlockBuilder` in
`NativeFrame.kt`) is the part each platform reimplements natively.

**UI → Engine: events.** A tiny inbound surface, already present on the view model:

```
setup_frame(route, route_arguments)
handle_action(index, action, event_type)
handle_variable(variable, need_to_log)
localize(key) -> String?
```

**Boundary rule — keep it coarse.** Emit **whole frame-state snapshots** (diffed), not chatty
per-variable getters. Every cross-FFI call has a cost and every payload is serialized, so a
per-variable read pattern would die by a thousand crossings. Coarse snapshots also dovetail with
the planned per-variable signals work on the UI side (see `variable-engine-signals` notes).

**What stays UI-side (not in the engine):**

- `BlockContext` / `ActionProps` — they carry `@Composable` callbacks; each UI builds its own
  props type from the plain models.
- Block providers (`NativeBlockProvider`) — they hold `@Composable` lambdas; registration is
  per-platform.
- **Contractor / UI actions** (`INativeActionContractor`, navigation, dialogs) — the engine
  emits these as **outbound events** ("navigate to X", "show toast Y") the UI interprets.
- **Logic actions** (`NativeScriptAction`, network calls) — these **move into the engine**.

---

## The four pillars (chosen Rust crates)

### 1. HTTP calling

- Client: **`reqwest`** (async, high-level) or **`ureq`** (blocking, tiny — favors app size).
- TLS: **`rustls`** (pure-Rust) — avoids OpenSSL cross-compilation pain on mobile.
- GraphQL: **`graphql-client`** or **`cynic`** — the current SDK is Apollo/GraphQL, so the
  request shape ports directly.
- JSON: **`serde` / `serde_json`**. JSONPath (today's `NativeJsonPath`/`NativeJsonPointer`):
  **`serde_json_path`** or `jsonpath-rust`.

### 2. Flow / data streaming

- **`tokio::sync::watch`** is a near-exact `StateFlow` replacement (holds latest value, notifies
  on change) — this is how the engine publishes each of the five state streams above.
- `futures::Stream` + `async-stream` for derived streams; `tokio::sync::{mpsc, broadcast}` for
  events/fan-out.
- Bridging: **flutter_rust_bridge maps a Rust `Stream` directly to a Dart `Stream`**; UniFFI
  callback interfaces push updates to Kotlin `Flow` / Swift `AsyncSequence`.

### 3. SQLite caching

- **`rusqlite` with the `bundled` feature** — statically compiles SQLite from source: identical
  version on every platform, no system dependency. The 4 Room DAOs (`FrameDao`,
  `FrameProductionDao`, `LocalizationDao`, `LocalizationProductionDao` — ~5 queries each) become
  small Rust query functions.
- **App-size lever:** to shave the bundled amalgamation, link the _system_ SQLite instead (present
  on both iOS and Android) via `rusqlite` without `bundled`. Decide by measurement.
- Immutable snapshots (today's `kotlinx.collections.immutable`): the **`im`** crate.

### 4. Frame state (the ViewModel replacement)

The ViewModel is gone, so the engine owns state + concurrency. Idiomatic Rust replacement:

- **Actor pattern:** one async task _owns_ the state struct (the blocks/variables/actions maps),
  receives events over an `mpsc` channel, mutates, and publishes snapshots via a `watch` channel.
  The `watch` sender **is** the `StateFlow`. This is a clean, race-free `NativeFrameViewModel`.
- The **borrow checker makes concurrent event handling safe by construction** — important once
  network results, script callbacks, and user actions can arrive at the engine simultaneously
  without a ViewModel's lifecycle scoping.
- Use **`im`** persistent maps to mirror the current `persistentMapOf` cheap-snapshot usage.

---

## Scripting: QuickJS embedded in Rust

Decision: **QuickJS replaces Rhino** (smaller, modern, embeddable C engine; chosen earlier in
this thread). Because QuickJS is C and the engine is Rust, it embeds _natively_ via **`rquickjs`**
— no FFI between the engine and its JS runtime.

The `NativeScriptAction` model must be preserved: scripts call **synchronous host functions**
(`getVariable`, `changeVariable`, `updateBlockProperties`). `rquickjs` supports injecting Rust
closures as JS functions, so that synchronous contract maps over directly. The existing
`NativeblocksScriptTest` cases become the conformance tests for the JS layer. See
[`js-engine-comparison.md`](./js-engine-comparison.md) for the constraints this model imposes.

This also shapes the **hot-update strategy** (below): stable logic lives in native Rust; any
behavior that must update over-the-wire without an app-store release lives in the QuickJS layer.

---

## The FFI boundary

FFI = Foreign Function Interface: how Swift/Kotlin/Dart call the compiled Rust library across a
C-style boundary. **You do not hand-write it — generators do:**

| Target                 | Generator                                  | Notes                                                |
| ---------------------- | ------------------------------------------ | ---------------------------------------------------- |
| Kotlin (Android)       | **UniFFI**                                 | produces the `.aar`-consumable bindings + JNI glue   |
| Swift (iOS)            | **UniFFI**                                 | produces `.framework`-consumable Swift bindings      |
| Flutter                | **flutter_rust_bridge**                    | best-in-class; async + native `Stream`→Dart `Stream` |
| React Native           | **uniffi-react-native** (Callstack) or JSI | wraps the same core                                  |
| Web (future, optional) | `wasm-bindgen`                             | not in scope now                                     |

UniFFI also supports **callback interfaces** (a.k.a. foreign traits): the mechanism for
"the outer layer provides components and the core calls them" (next section).

**The hard part to prototype first:** all four pillars are async + streaming, so the boundary is
async + streaming FFI — the gnarliest kind. The generators handle it, but validate it on a real
slice before betting the SDK on it (see [Phase 0](#phase-0--de-risk-before-committing)).

---

## Host-provided components (callback traits)

The core stays portable by keeping platform I/O in the host and calling _up_ through traits. The
core _owns_ the deterministic, portable work (SQLite cache, file cache, checksums, variable
resolution, action sequencing — all pure or local-I/O). It _delegates_ truly platform-specific
concerns via UniFFI callback interfaces:

```rust
// commonized as Rust traits, implemented per platform and passed in at init
trait Logger            { fn log(&self, level: u8, event: String, message: String, params: Map); }
trait SecureStore       { fn get(&self, key: String) -> Option<String>; fn set(&self, k: String, v: String); }
trait HostEvents        { fn emit(&self, event: HostEvent); }   // navigation, dialogs, contractors
// install-id, locale, etc. — small, platform-specific reads
```

Caching/checksum/file work that today lives in `LocalCacheProvider` + Room can live **entirely
in the Rust core** on native targets (rusqlite + `std::fs` + `sha2`/`blake3`); only a future
WASM/web build would have to delegate storage to the host.

---

## App-size strategy (since it is the #1 constraint)

- Build with `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`, `strip = true`.
- Prefer **`ureq` over `reqwest`** if the async client's dependency tree proves heavy.
- Use **`rustls`** (no OpenSSL) and consider **linking system SQLite** instead of bundling.
- Keep the dependency tree lean — every crate is binary weight; audit with `cargo bloat`.
- Measure the per-platform artifact (`.aar`, `.framework`) early and treat size as a tracked
  budget, not an afterthought.

---

## Repo / crate layout

```
engine/                              (Rust workspace — the shared core)
  Cargo.toml                         opt-level=z, lto, strip
  crates/
    nb-models/      serde structs    ← ports frame/domain/model/* (the wire contract)
    nb-core/        use cases, repositories (traits), variable/JSONPath resolution,
                    FrameEngine (actor + watch), action sequencing
    nb-net/         reqwest/ureq + rustls + graphql-client
    nb-cache/       rusqlite (bundled or system) + file/kv cache + sha2 checksums
    nb-script/      rquickjs + the getVariable/changeVariable/updateBlockProperties host fns
    nb-ffi/         UniFFI interface (.udl / proc-macro) — the public boundary
  bindings/
    android/        cargo-ndk build → .aar (+ UniFFI Kotlin)
    ios/            cargo build → .xcframework (+ UniFFI Swift)
    flutter/        flutter_rust_bridge codegen
    react-native/   uniffi-react-native

ui-compose/   (Android) Compose renderer — consumes the .aar; keeps NativeFrame, BlockContext, providers
ui-swiftui/   (iOS)     SwiftUI renderer — consumes the .framework
(flutter/RN/web UIs live in their own repos)
```

`NativeblocksManager` splits: the **engine half** (`syncFrame`, `getExperiment`,
`setGlobalParameters`, `setLocalization`, `getScaffold`, init) → Rust core; the **UI registration
half** (`provideBlock`, `provideFallbackBlock`, `provideActionContractor`, `provideTypeConverter`)
→ each platform's UI layer.

---

## Phased migration plan

### Phase 0 — De-risk before committing

- **Measure iOS size** of a same-shape KMP `.framework` vs. a Rust `.xcframework` with your real
  dependency set (rustls, rusqlite, rquickjs). This confirms Rust's size advantage is real for
  _your_ build, and is the last checkpoint before ruling out KMP.
- **Prototype the hard boundary:** one async streaming call Rust→Kotlin/Swift/Dart via UniFFI +
  flutter_rust_bridge, proving the `watch`→`Flow`/`Stream` bridge end-to-end.
- Write the **engine spec + cross-platform conformance test suite** (golden tests for sync
  algorithm, checksum caching, variable resolution, JSONPath, script evaluation, action
  sequencing). This is the safety net that lets you swap Kotlin/Swift engines for Rust with a
  parity guarantee — valuable even if you stop here.

### Phase 1 — Strangler-fig the first slice

- Port **one pure slice — checksum + cache** — into `nb-cache` + `nb-models`. Ship it behind the
  conformance suite, consumed by the existing Android SDK via the `.aar`. Prove the full
  build/CI/binding pipeline on every target before porting more.

### Phase 2 — Port the engine core

- Move network (`nb-net`), repositories/use cases, variable/JSONPath resolution, and the
  `FrameEngine` (actor + watch) into the Rust core. Swap Apollo→`graphql-client`, Room→`rusqlite`.
- Embed QuickJS (`nb-script`); promote `NativeblocksScriptTest` into the conformance suite.
- Android and iOS now consume the Rust core; their UI layers are unchanged.

### Phase 3 — Flutter + React Native

- `flutter_rust_bridge` + a Dart `FrameEngine` proxy and widget-tree renderer; port a few
  reference blocks to validate the contract end-to-end. Same for RN via `uniffi-react-native`.

### Phase 4 — Retire the duplicated engines; optional hot-update

- Decommission the Kotlin and Swift engine logic once parity holds.
- Evaluate a **QuickJS-based hot-update layer** for over-the-wire logic updates (the native core
  itself can't be hot-shipped; dynamic behavior lives in JS, pushed like frame definitions).

Each phase ships independently and keeps existing platforms working throughout.

---

## Risks & open questions

- **Async/streaming FFI is the hardest part.** Generators handle it, but it must be the first
  thing prototyped (Phase 0), not discovered late.
- **GraphQL on Rust + mobile cross-compile.** Validate `graphql-client`/`cynic` codegen and
  `rustls` + `cargo-ndk` / `xcframework` packaging early.
- **Rust learning curve.** The team knows Go, not Rust. The borrow checker + async are the ramp;
  budget for it. The generators reduce, but do not remove, the FFI learning.
- **Script parity (QuickJS vs Rhino/JSC).** Number coercion, JSON, regex differences. Mitigation:
  the shared conformance golden tests; keep scripts to the documented small surface.
- **App-size regressions creep in via dependencies.** Track artifact size as a budget; audit with
  `cargo bloat`; prefer lean crates.
- **Two engines remain by design** (Rust native core + web TS engine). Keep them in sync via the
  same conformance spec, or accept web lagging on non-UI features.
- **Decommissioning working code.** Retiring the solid Kotlin and near-mature Swift engines is
  real sunk-cost destruction; only do it once the conformance suite proves parity (Phase 4).

---

## Sources / references (local)

- `core/src/main/java/io/nativeblocks/core/frame/presenter/NativeFrameViewModel.kt` — the
  headless engine logic (StateFlows + action sequencing) the Rust `FrameEngine` reproduces.
- `core/src/main/java/io/nativeblocks/core/frame/presenter/NativeFrame.kt` — the per-platform
  rendering walk each UI reimplements.
- `core/src/main/java/io/nativeblocks/core/api/provider/{block,action}/*` — the UI registration
  contract that stays per-platform.
- `core/src/main/java/io/nativeblocks/core/api/provider/action/defaults/NativeScriptAction.kt`
  and [`js-engine-comparison.md`](./js-engine-comparison.md) — scripting constraints (→ QuickJS).
- `core/src/main/java/io/nativeblocks/core/{cache,network,frame/data}/*` — the logic the Rust
  crates replace (rusqlite, reqwest/rustls/graphql-client).
  </content>
