# browser-protocol

[![Crates.io](https://img.shields.io/crates/v/browser-protocol.svg)](https://crates.io/crates/browser-protocol)
[![Documentation](https://docs.rs/browser-protocol/badge.svg)](https://docs.rs/browser-protocol)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, fully type-safe Rust client for the **Chrome DevTools Protocol (CDP)**, automatically generated from the official protocol definitions.

## 🚀 Features

- **Full Coverage**: Includes types, commands, and events for all CDP domains.
- **Fluent Builders**: Build commands easily with ergonomic builder APIs.
- **Required-Argument Safety**: Required parameters are enforced directly in the `.builder(...)` constructor, ensuring compile-time protocol compliance.
- **Zero-Allocation**: Leverages `Cow<'a, str>` for string properties to avoid heap allocation overhead.
- **Encapsulated & Safe**: Struct fields are private, exposing read-only access through compact getter methods.
- **Clean Serialization**: Automatically omits optional `None` fields from serialized payloads to reduce network bandwidth.
- **Zero Warnings**: Crate is compiled warning-free with inline Rustdoc comments from official schemas.

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
browser-protocol = { version = "0.1.2", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 🛠 Usage Example

### 1. Constructing commands with parameters (e.g. `Page.navigate`)
Required fields are passed directly to `builder(...)`, while optional fields are chained:

```rust
use browser_protocol::page::{NavigateParams, TransitionType};

fn main() {
    // url is required, so it is passed directly. transitionType is optional.
    let nav = NavigateParams::builder("https://www.rust-lang.org")
        .transitionType(TransitionType::Typed)
        .build();
    
    // Controlled read-only getter access
    println!("Navigating to: {}", nav.url());
    
    // Serialized payload skips unset options (like referrer policy, frameId, etc.)
    println!("Payload: {}", serde_json::to_string(&nav).unwrap());
}
```

### 2. Constructing commands with no parameters (e.g. `Browser.getVersion`)
No builder boilerplate is generated or needed; just use `default()`:

```rust
use browser_protocol::browser::GetVersionParams;

fn main() {
    let params = GetVersionParams::default();
    println!("Payload: {}", serde_json::to_string(&params).unwrap());
}
```

## 🏗 How it was built
This crate is automatically generated using a custom Python script that parses the `browser_protocol.json` and produces idiomatic Rust modules.

## ⚖ License
Distributed under the MIT License. See `LICENSE` for more information.

---
*Disclaimer: This is an automatically generated project. Always check the official CDP documentation for the latest protocol changes.*
