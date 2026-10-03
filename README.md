# browser-protocol

[![Crates.io](https://img.shields.io/crates/v/browser-protocol.svg)](https://crates.io/crates/browser-protocol)
[![Documentation](https://docs.rs/browser-protocol/badge.svg)](https://docs.rs/browser-protocol)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, zero-allocation, fully compile-safe Rust representation of the **Chrome DevTools Protocol (CDP)**, generated directly from the official protocol definitions.

---

## Design Goals and Features

Most auto-generated CDP crates expose raw, unidiomatic APIs with substantial runtime allocation overhead. This library is designed from the ground up to solve those problems.

### 1. Idiomatic Rust naming

All generated fields, getters, and builder setters are translated from the protocol's raw `camelCase` to standard Rust `snake_case` (for example, `transitionType` becomes `transition_type` and `backendDOMNodeId` becomes `backend_dom_node_id`). `#[serde(rename = "...")]` attributes ensure the serialized JSON matches the exact wire format Chrome expects.

### 2. Zero-copy string handling

String properties use `Cow<'a, str>` instead of allocating a `String`. Builder arguments use `impl Into<...>`, so you can pass static string literals (`&str`) or owned strings without unnecessary heap allocations.

### 3. Compile-time argument safety

The builder pattern separates required from optional parameters. Required parameters are passed directly to `builder(...)`, so protocol compliance is checked at compile time:

```rust
// `url` is required (passed to builder); `transition_type` is optional (chained).
let nav = NavigateParams::builder("https://www.rust-lang.org")
    .transition_type(TransitionType::Typed)
    .build();
```

### 4. Proc-macro powered, minimal boilerplate

Getters, builders, command glue, and event glue are all synthesized by derives from `browser-protocol-macros`, so the generated source is limited to plain struct definitions. This removes roughly 60% of the generated source compared to hand-written builders and getters, and allows the same build to expose over 200 typed events. Runtime dependencies remain limited to `serde` and `serde_json`.

### 5. No async runtime lock-in

The crate includes no WebSocket client and does not require a specific async runtime. It is compatible with any runtime or network stack.

---

## Installation

```toml
[dependencies]
browser-protocol = { version = "0.1.5", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## Usage

### 1. Constructing a request with optional parameters

```rust
use browser_protocol::page::{NavigateParams, TransitionType};

fn main() {
    // Build the command parameters.
    let nav = NavigateParams::builder("https://www.rust-lang.org")
        .transition_type(TransitionType::Typed)
        .build();

    // Read-only getters.
    println!("Navigating to: {}", nav.url());

    // Serialize to the wire protocol payload (unset Option fields are skipped).
    let payload = serde_json::to_string(&nav).unwrap();
    println!("Payload: {}", payload);
    // Output: {"url":"https://www.rust-lang.org","transitionType":"typed"}
}
```

### 2. Handling command request and response types

Every parameter struct implements `crate::CdpCommand<'a>`, which binds it to its command method and its corresponding response type:

```rust
use browser_protocol::accessibility::GetPartialAXTreeParams;
use browser_protocol::dom::NodeId;
use browser_protocol::CdpCommand;

fn get_accessibility_tree() {
    let params = GetPartialAXTreeParams::builder()
        .node_id(NodeId::from(42))
        .fetch_relatives(true)
        .build();

    // The trait binds this command to its method name and response type.
    assert_eq!(GetPartialAXTreeParams::METHOD, "Accessibility.getPartialAXTree");

    // In your network client:
    // let response_json = websocket.send_command(GetPartialAXTreeParams::METHOD, &params).await;
    // let response: GetPartialAXTreeReturns = serde_json::from_str(&response_json).unwrap();
}
```

### 3. A generic client helper

Because every command implements `CdpCommand`, a single pair of helpers covers all of them:

```rust
use serde::{de::DeserializeOwned, Serialize};
use browser_protocol::{CdpCommand, Command, Response};

fn encode<'a, P: CdpCommand<'a> + Serialize>(id: u64, params: &'a P) -> String {
    serde_json::to_string(&Command::new(id, params)).unwrap()
}

fn decode<'a, P>(json: &'a str) -> Response<P::Response>
where
    P: CdpCommand<'a>,
    P::Response: DeserializeOwned,
{
    serde_json::from_str(json).unwrap()
}
```

---

## The Derive Macros

Three derives from `browser-protocol-macros` keep every generated type down to a plain struct declaration.

### `CdpBuilder` - builders and getters

Every generated type derives `CdpBuilder`, which emits:

* A `builder(...)` constructor where every non-`Option` field is a required argument (typed as `impl Into<FieldType>`).
* Chainable setters for every `Option` field, wrapping the value in `Some`.
* A `build()` method that moves the accumulated fields into the struct.
* Read-only getters whose return type is chosen from the field type:

  | Field type | Getter return |
  | --- | --- |
  | `Cow<'a, str>` / `Option<Cow<'a, str>>` | `&str` / `Option<&str>` |
  | `Vec<T>` / `Option<Vec<T>>` | `&[T]` / `Option<&[T]>` |
  | `Box<T>` / `Option<Box<T>>` | `&T` / `Option<&T>` |
  | numeric / `bool` primitives | the value itself (they are `Copy`) |
  | any other type `T` | `&T` / `Option<&T>` |

### `CdpCommand` - command glue

A command's parameter struct carries its method name and response type instead of a hand-written `impl`:

```rust
#[derive(CdpBuilder, CdpCommand)]
#[cdp(method = "Page.navigate", response = "NavigateReturns<'a>")]
pub struct NavigateParams<'a> { /* fields */ }
```

This generates `NavigateParams::METHOD` and the `CdpCommand<'a>` implementation. If `response` is omitted, the reply type defaults to `crate::EmptyReturns`.

### `CdpEvent` - typed events

Every event in the schema becomes a typed struct with the same builder and getters:

```rust
#[derive(CdpBuilder, CdpEvent)]
#[cdp(method = "Page.frameNavigated")]
pub struct FrameNavigated<'a> { /* fields */ }

assert_eq!(FrameNavigated::METHOD, "Page.frameNavigated");
```

The `CdpEvent` trait exposes `METHOD`, so events can be handled generically rather than by matching raw JSON.

### Error-aware replies

`Response<T>` decodes `{"id", "result"}`. For the failure path, `CdpReply<T>` decodes either shape:

```rust
match serde_json::from_str::<CdpReply<CaptureScreenshotReturns>>(raw)? {
    CdpReply::Ok(reply) => save(reply.result.data()),
    CdpReply::Err(err) => eprintln!("CDP {}: {}", err.error.code, err.error.message),
}
```

---

## Code Generation

The code is generated by a single Python script that performs schema analysis:

1. **Fixed-point lifetime propagation**: The generator iteratively analyzes types, command parameters/returns, and events to detect circular references, nesting, and dependency hierarchies. It determines which types require a lifetime parameter (`<'a>`) and wraps recursive structures in `Box` to prevent infinite-size compile errors.
2. **HTML and Markdown escaping**: Schema documentation contains raw Markdown and HTML brackets. The generator escapes them into valid rustdoc, keeping compilation warning-free.
3. **Per-domain feature flags**: Every CDP domain is a Rust feature. You can limit compile times to the domains you need:

   ```toml
   # Compile only the page and dom domains.
   browser-protocol = { version = "0.1.5", default-features = false, features = ["page", "dom"] }
   ```

### Keeping the protocol up to date

```bash
# Exit code 0 = up to date, 1 = a newer protocol is available.
python scripts/generate_rust_code.py --check

# Download the latest protocol and regenerate everything.
python scripts/generate_rust_code.py --download
```

### Regenerating the code

`--download` is optional; if a local `browser_protocol.json` is present it is used as-is.

```bash
python scripts/generate_rust_code.py --version 0.1.5
```

---

## Development

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Lint policy is defined in the workspace `Cargo.toml` (`[workspace.lints]`) and `clippy.toml`. Clippy warnings are treated as errors in CI. A small set of lints that are not meaningful for generated code (`empty_line_after_doc_comments`, `doc_lazy_continuation`, and `vec_box`) is allowed explicitly; everything else is held to the default Clippy standard.

---

## Repository Layout

```
browser-protocol/          # The crate: generated modules + CdpCommand / CdpEvent traits
  src/<domain>/mod.rs      # One module per CDP domain
  scripts/
    generate_rust_code.py  # Schema analysis and code generation
  macros/                  # browser-protocol-macros: CdpBuilder / CdpCommand / CdpEvent derives
```

> **Publishing note:** `browser-protocol` depends on `browser-protocol-macros`, so the macros crate must be published to crates.io first.

---

## License

Distributed under the MIT License. See `LICENSE` for more information.
