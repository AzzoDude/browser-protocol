# browser-protocol

[![Crates.io](https://img.shields.io/crates/v/browser-protocol.svg)](https://crates.io/crates/browser-protocol)
[![Documentation](https://docs.rs/browser-protocol/badge.svg)](https://docs.rs/browser-protocol)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, zero-allocation, fully compile-safe Rust representation of the **Chrome DevTools Protocol (CDP)**, generated directly from the official protocol definitions.

---

## 🚀 Key Design Goals & Features

Most auto-generated CDP crates output raw, unidiomatic APIs with substantial runtime allocation overhead. This library is designed from the ground up to solve these issues:

### 1. Idiomatic Rust Naming Conventions
* All generated struct fields, getters, and builder setter methods are translated from the protocol's raw `camelCase` to standard Rust `snake_case` (e.g., `transitionType` becomes `transition_type`, and `backendDOMNodeId` becomes `backend_dom_node_id`).
* Standard `#[serde(rename = "...")]` attributes ensure the serialized JSON wire protocol matches the exact formats required by Chrome.

### 2. Zero-Copy String Management
* Utilizes `Cow<'a, str>` instead of allocating heap memory (`String`) for string properties.
* String arguments in builders use `impl Into<...>`, allowing you to pass static string literals (`&str`) or owned strings without unnecessary heap allocations.

### 3. Compile-Time Argument Safety
* The builder pattern differentiates between **required** and **optional** parameters. Required parameters are passed directly as arguments to the `builder(...)` function, guaranteeing protocol compliance at compile time:
  ```rust
  // `url` is required (passed to builder), `transition_type` is optional (chained)
  let nav = NavigateParams::builder("https://www.rust-lang.org")
      .transition_type(TransitionType::Typed)
      .build();
  ```

### 4. Proc-Macro Powered, Minimal Boilerplate
* All getters, builders, command glue, and event glue are synthesized by derives from `browser-protocol-macros`, so the generated source contains only plain struct definitions.
* This removes roughly **60% of the generated source** compared to hand-emitting builders/getters, and lets the same build also expose **210 typed events**.
* Runtime dependencies stay tiny: `serde` and `serde_json`.

### 5. No Async Runtime Lock-in
* Does not include a WebSocket client or force a specific async runtime (like `tokio`). This keeps the package lightweight and compatible with any async runtime or network stack.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
browser-protocol = { version = "0.1.5", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## 🛠 Usage Examples

### 1. Constructing a Request with Optional Parameters
```rust
use browser_protocol::page::{NavigateParams, TransitionType};

fn main() {
    // 1. Build the command parameters
    let nav = NavigateParams::builder("https://www.rust-lang.org")
        .transition_type(TransitionType::Typed)
        .build();

    // 2. Read-only getters
    println!("Navigating to: {}", nav.url()); // prints "https://www.rust-lang.org"

    // 3. Serialize to wire protocol payload (skips unset Option fields)
    let payload = serde_json::to_string(&nav).unwrap();
    println!("Payload: {}", payload);
    // Output: {"url":"https://www.rust-lang.org","transitionType":"typed"}
}
```

### 2. Handling Command Request/Response Types
Every parameter struct implements `crate::CdpCommand<'a>` which binds it to its command method and its corresponding response (`Returns`) type:

```rust
use browser_protocol::accessibility::{GetPartialAXTreeParams, GetPartialAXTreeReturns};
use browser_protocol::dom::NodeId;
use browser_protocol::CdpCommand;

fn get_accessibility_tree() {
    let params = GetPartialAXTreeParams::builder()
        .node_id(NodeId::from(42))
        .fetch_relatives(true)
        .build();

    // The trait binds this command to its method name and response type:
    assert_eq!(GetPartialAXTreeParams::METHOD, "Accessibility.getPartialAXTree");

    // In your network client:
    // let response_json = websocket.send_command(GetPartialAXTreeParams::METHOD, &params).await;
    // let response: GetPartialAXTreeReturns = serde_json::from_str(&response_json).unwrap();
}
```

---

## 🧬 The Derive Macros

Three derives from `browser-protocol-macros` keep every generated type down to a plain struct declaration.

### `CdpBuilder` — builders and getters

Every generated type is annotated with `#[derive(CdpBuilder)]`. The macro inspects the fields and emits:

* A `builder(...)` constructor where every non-`Option` field is a **required argument** (typed as `impl Into<FieldType>`).
* Chainable setters for every `Option` field, wrapping the value in `Some`.
* A `build()` method that moves the accumulated fields into the final struct.
* Read-only getters whose return type is chosen from the field type:
  | Field type | Getter return |
  | --- | --- |
  | `Cow<'a, str>` / `Option<Cow<'a, str>>` | `&str` / `Option<&str>` |
  | `Vec<T>` / `Option<Vec<T>>` | `&[T]` / `Option<&[T]>` |
  | `Box<T>` / `Option<Box<T>>` | `&T` / `Option<&T>` |
  | numeric / `bool` primitives | the value itself (they are `Copy`) |
  | any other type `T` | `&T` / `Option<&T>` |

### `CdpCommand` — command glue

A command's parameter struct carries the method name instead of a hand-written impl:

```rust
#[derive(CdpBuilder, CdpCommand)]
#[cdp(method = "Page.navigate", response = "NavigateReturns<'a>")]
pub struct NavigateParams<'a> { /* fields */ }
```

This generates `NavigateParams::METHOD` and the `impl CdpCommand<'a>` (with `type Response`). Omit `response` and the reply type defaults to `crate::EmptyReturns`.

### `CdpEvent` — typed events

Every event in the schema becomes a typed struct with the same builder + getters:

```rust
#[derive(CdpBuilder, CdpEvent)]
#[cdp(method = "Page.frameNavigated")]
pub struct FrameNavigated<'a> { /* fields */ }

assert_eq!(FrameNavigated::METHOD, "Page.frameNavigated");
```

The `CdpEvent` trait exposes `METHOD`, so events can be handled generically instead of by matching raw JSON.

### Error-aware replies

`Response<T>` still decodes `{"id", "result"}`. For the failure path, `CdpReply<T>` decodes either shape:

```rust
match serde_json::from_str::<CdpReply<CaptureScreenshotReturns>>(raw)? {
    CdpReply::Ok(reply) => save(reply.result.data()),
    CdpReply::Err(err) => eprintln!("CDP {}: {}", err.error.code, err.error.message),
}
```

---

## 🏗 Code Generation Mechanics

The code is generated by a single Python script that performs advanced schema analysis:

1. **Fixed-Point Lifetime Propagation Pass**: The generator performs iterative analysis over the CDP types, command params/returns, and events to detect circular type references, nesting, and dependency hierarchies. It automatically determines which types must have a lifetime parameter (`<'a>`) and wraps recursive structures inside `Box` to prevent infinite-size compilation errors.
2. **HTML/Markdown Escaping**: Schema documentation from the Chrome DevTools Protocol contains raw markdown and HTML brackets. The generator cleans and escapes these brackets into valid Rustdoc format, keeping compilation entirely warning-free.
4. **Domain-Specific Feature Flags**: Every CDP domain is represented by a Rust feature flag. You can optimize compile times by only compiling the domains your project needs:
   ```toml
   # Compile only the page and dom domains
   browser-protocol = { version = "0.1.5", default-features = false, features = ["page", "dom"] }
   ```

### Keeping the Protocol Up to Date

Check whether upstream Chrome DevTools has published a newer protocol, then pull it down:

```bash
# Exit code 0 = up to date, 1 = newer protocol available
python scripts/generate_rust_code.py --check

# Download the latest protocol and regenerate everything
python scripts/generate_rust_code.py --download
```

### Regenerating the Code

`--download` is optional; if a local `browser_protocol.json` is already present it will be used as-is. To regenerate modules from the current protocol file:

```bash
python scripts/generate_rust_code.py --version 0.1.5
```

---

## 📁 Repository Layout

```
browser-protocol/          # The crate: generated modules + CdpCommand trait
  src/<domain>/mod.rs      # One module per CDP domain
  scripts/
    generate_rust_code.py  # Schema analysis + code generation
  macros/                  # browser-protocol-macros: CdpBuilder/CdpCommand/CdpEvent derives
```

> **Publishing note:** `browser-protocol` depends on `browser-protocol-macros`, so the macros crate must be published to crates.io first.

---

## ⚖ License

Distributed under the MIT License. See `LICENSE` for more information.
