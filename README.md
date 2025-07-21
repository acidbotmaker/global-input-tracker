# `global-input-tracker`

> Cross-platform global mouse and keyboard event tracker for Node.js using Rust and napi-rs.

[![npm](https://img.shields.io/npm/v/global-input-tracker)](https://www.npmjs.com/package/global-input-tracker)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## ✨ Features

- 🔁 Real-time global keyboard & mouse event tracking
- ⚡ Built in Rust for speed and low overhead
- 🔌 Native Node.js addon via [`napi-rs`](https://napi.rs/)
- 💻 Supports Windows, macOS, Linux (X11)

---

## 🚀 Installation

```bash
npm install global-input-tracker
```

```bash
yarn add global-input-tracker
```

## Support matrix

### Operating Systems

|                  | node14 | node16 | node18 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✗      | ✗      | ✗      |
| Windows x32      | ✗      | ✗      | ✗      |
| Windows arm64    | ✗      | ✗      | ✗      |
| macOS x64        | ✓      | ✓      | ✓      |
| macOS arm64      | ✗      | ✗      | ✗      |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      |
| Linux arm gnu    | ✗      | ✗      | ✗      |
| Linux arm64 gnu  | ✗      | ✗      | ✗      |
| Linux arm64 musl | ✗      | ✗      | ✗      |
| Android arm64    | ✗      | ✗      | ✗      |
| Android armv7    | ✗      | ✗      | ✗      |
| FreeBSD x64      | ✓      | ✓      | ✓      |

## Ability
### Global Input Tracking
- Monitor all keyboard presses and releases across the system
- Track mouse movements globally, even outside your app window
- Detect mouse button presses and releases

### 🔄 Cross-Platform Support
- Works on macOS, Linux (GNU + MUSL), and FreeBSD
- Node.js runtime support: Node 14+, 16+, 18+

### 🔌 Efficient Native Performance
- Powered by Rust and rdev for low-latency input detection
- Uses napi-rs for safe and efficient FFI to Node.js

### 🧩 Structured Event Data
- Returns rich InputEvent objects including:
  - event_type (e.g., KeyDown, MouseMove)
  - payload (e.g., KeyA, LeftButton)
  - x, y mouse coordinates (when applicable)

### 👂 Background Event Listener
 - Runs in a background thread without blocking the main Node.js event loop
 - Designed for real-time, asynchronous input handling

### 🛠 TypeScript Friendly
  - Auto-generated TypeScript types
  - Easy to use in both JavaScript and TypeScript environments


## 🧪 Run Example
To test the module locally with an example script:

```
node examples/main.mjs
```