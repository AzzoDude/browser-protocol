# browser_protocol

[![Crates.io](https://img.shields.io/crates/v/browser_protocol.svg)](https://crates.io/crates/browser_protocol)
[![Documentation](https://docs.rs/browser_protocol/badge.svg)](https://docs.rs/browser_protocol)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, fully type-safe Rust client for the **Chrome DevTools Protocol (CDP)**, automatically generated from the official protocol definitions.

## 🚀 Features

- **Full Coverage**: Includes types, commands, and events for all CDP domains.
- **Type Safety**: Leverage Rust's type system to avoid runtime protocol errors.
- **Async Ready**: Designed to work seamlessly with `tokio` and `serde`.
- **Zero Warnings**: The crate and its documentation are built to be perfectly clean.
- **Documentation**: All protocol descriptions are included as Rustdoc comments.

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
browser_protocol = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 🛠 Usage Example

```rust
use browser_protocol::dom::GetDocumentParams;
use browser_protocol::page::NavigateParams;

fn main() {
    // Example: Create a navigation command
    let nav = NavigateParams {
        url: "https://www.rust-lang.org".to_string(),
        ..Default::default()
    };
    
    println!("Request: {:?}", serde_json::to_string(&nav).unwrap());
}
```

## 🏗 How it was built
This crate is automatically generated using a custom Python script that parses the `browser_protocol.json` and produces idiomatic Rust modules.

## ⚖ License
Distributed under the MIT License. See `LICENSE` for more information.

---
*Disclaimer: This is an automatically generated project. Always check the official CDP documentation for the latest protocol changes.*
