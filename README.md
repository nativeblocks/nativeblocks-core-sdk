# Nativeblocks

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
![Android](https://img.shields.io/badge/android-0.2.0-green.svg)
![iOS](https://img.shields.io/badge/iOS-0.2.0-orange.svg)
[![Slack](https://img.shields.io/badge/slack-join-4A154B.svg?logo=slack)](https://join.slack.com/t/nativeblockscommunity/shared_invite/zt-3swrrw99s-zoHYrgE8Lfa07oiiGm49Sw)

Over-the-air updates for Android and iOS. Build your app from your own native components, then
drive its screens and logic remotely without shipping an app update.

Nativeblocks runs **frames**: screens made of your Jetpack Compose and SwiftUI components
(**blocks**), wired to your own logic (**actions**). You change a frame remotely and the app picks
it up; the components stay native and stay yours.

Docs and guides: **[nativeblocks.io](https://nativeblocks.io)**

## How It Works

```
┌────────────────────────┐   ┌────────────────────────┐
│  Android (Compose)     │   │  iOS (SwiftUI)         │
│  runtime · foundation  │   │  runtime · foundation  │
│  compiler · devkit     │   │  compiler · devkit     │
└───────────┬────────────┘   └───────────┬────────────┘
            │          UniFFI            │
            └─────────────┬──────────────┘
              ┌───────────┴───────────┐
              │  Rust core (runtime)  │
              └───────────────────────┘
```

One Rust core handles fetching, caching, and running frame logic, so all platforms behave the
same. Each platform wraps it in native libraries:

| Library      | What it does                                                           |
|--------------|------------------------------------------------------------------------|
| `runtime`    | Loads frames and renders them. The entry point: `NativeblocksManager`. |
| `foundation` | Ready-made basic blocks and types.                                     |
| `compiler`   | Turns your annotated components and actions into Nativeblocks blocks.  |
| `devkit`     | Live preview and debugging tools. Use it in debug builds only.         |

## Community

Questions and discussion happen on
[Slack](https://join.slack.com/t/nativeblockscommunity/shared_invite/zt-3swrrw99s-zoHYrgE8Lfa07oiiGm49Sw).

## Contributing

Bug reports with a minimal reproduction are welcome. Read [CONTRIBUTING.md](CONTRIBUTING.md)
before opening a pull request.
