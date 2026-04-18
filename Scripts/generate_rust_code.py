import subprocess
import shutil
import os
import platform
import sys
import argparse
import json
import re

dependencies_rs: dict[str, str] = {
    "tokio": "full",
    "serde": "derive",
    "serde_json": ""
}

profile_release_content: str = """
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
"""

def to_camel_case(snake_str):
    components = snake_str.replace('-', '_').split('_')
    return "".join(x[:1].upper() + x[1:] for x in components if x)

def format_rustdoc(description, indent_level=0, is_inner=False):
    if not description: return ""
    indent = " " * indent_level
    symbol = "//! " if is_inner else "/// "
    
    # Basic cleaning
    clean_text = description.replace("\\n", "\n").replace("`", "'")
    
    # Wrap URLs in < > if not already wrapped
    url_pattern = r'(?<!<)(https?://[^\s)]+)(?!>)'
    clean_text = re.sub(url_pattern, r'<\1>', clean_text)
    
    # Escape HTML-like tags by escaping < and > that are not part of a URL
    # Move URLs to placeholders, escape, and restore
    urls = []
    def url_repl(match):
        urls.append(match.group(0))
        return f"__URL_PLACEHOLDER_{len(urls)-1}__"
    
    placeholder_text = re.sub(r'<https?://[^>]+>', url_repl, clean_text)
    placeholder_text = placeholder_text.replace("<", r"\<").replace(">", r"\>")
    placeholder_text = placeholder_text.replace("[", r"\[").replace("]", r"\]")
    
    for i, url in enumerate(urls):
        placeholder_text = placeholder_text.replace(f"__URL_PLACEHOLDER_{i}__", url)
    
    clean_text = placeholder_text
    
    lines = clean_text.split("\n")
    doc_lines = []
    for line in lines:
        clean_line = line.strip()
        doc_lines.append(f"{indent}{symbol}{clean_line}" if clean_line else f"{indent}{symbol}")
    return "\n".join(doc_lines) + "\n"

def get_rust_type(prop, current_struct_name=None):
    base_type = "serde_json::Value"
    is_recursive = False

    if "$ref" in prop:
        ref = prop["$ref"]
        if "." in ref:
            domain, t_name = ref.split(".")
            if t_name == "Value": t_name = "ProtocolValue"
            base_type = f"crate::{domain.lower()}::{t_name}"
            # Check for recursion (simple check for the current struct)
            if t_name == current_struct_name: is_recursive = True
        else:
            base_type = ref
            if ref == "Value": base_type = "ProtocolValue"
            if ref == current_struct_name: is_recursive = True

    elif prop.get("type") == "string": base_type = "String"
    elif prop.get("type") == "number": base_type = "f64"
    elif prop.get("type") == "boolean": base_type = "bool"
    elif prop.get("type") == "any": base_type = "serde_json::Value"
    elif prop.get("type") == "array":
        item_type = get_rust_type(prop.get("items", {}))
        base_type = f"Vec<{item_type}>"
    elif prop.get("type") == "integer":
        name = prop.get("name", "").lower()
        if any(k in name for k in ["delta", "offset"]) or name in ["x", "y"]: base_type = "i32"
        elif any(k in name for k in ["id", "count", "index", "size", "length"]): base_type = "u64"
        else: base_type = "i64"
    elif prop.get("type") == "object":
        base_type = "serde_json::Map<String, serde_json::Value>"
        
    # Apply Indirection (Box) to fix E0072
    if is_recursive:
        base_type = f"Box<{base_type}>"

    if prop.get("optional", False):
        return f"Option<{base_type}>"
    return base_type

def generate_cdp_modules(project_name: str):
    json_path = "browser_protocol.json"
    if not os.path.exists(json_path):
        json_path = os.path.join("..", "browser_protocol.json")
        
    with open(json_path, "r", encoding="utf-8") as f:
        schema = json.load(f)

    project_path = ".."
    src_dir = os.path.join(project_path, "src")
    lib_rs_content = []

    # STUBS with added UniqueDebuggerId
    all_domains = [d.get("domain").lower() for d in schema.get("domains", [])]
    for stub in ["runtime", "debugger", "heap_profiler", "profiler"]:
        if stub not in all_domains:
            stub_dir = os.path.join(src_dir, stub)
            os.makedirs(stub_dir, exist_ok=True)
            with open(os.path.join(stub_dir, "mod.rs"), "w", encoding="utf-8") as f:
                f.write(f"use serde::{{Serialize, Deserialize}};\n")
                f.write("pub type RemoteObjectId = String;\npub type RemoteObject = serde_json::Value;\n")
                f.write("pub type ScriptId = String;\npub type StackTrace = serde_json::Value;\n")
                f.write("pub type UniqueDebuggerId = String;\npub type SearchMatch = serde_json::Value;\n")
                f.write("pub type ExecutionContextId = i64;\npub type Timestamp = f64;\n")
            lib_rs_content.append(f"pub mod {stub};")

    for domain in schema.get("domains", []):
        d_name = domain.get("domain")
        if d_name.lower() in ["webmcp"]: continue
        lib_rs_content.append(f"pub mod {d_name.lower()};")
        domain_dir = os.path.join(src_dir, d_name.lower())
        os.makedirs(domain_dir, exist_ok=True)
        
        mod_code = []
        if "description" in domain: mod_code.append(format_rustdoc(domain['description'], 0, True))
        # Use alias for Value to fix E0255/E0117
        mod_code.extend(["use serde::{Serialize, Deserialize};", "use serde_json::Value as JsonValue;", ""])

        for t in domain.get("types", []):
            mod_code.append(format_rustdoc(t.get("description"), 0))
            t_id = t.get("id")
            # Rename struct Value to ProtocolValue to avoid conflict
            safe_t_id = f"Protocol{t_id}" if t_id == "Value" else t_id

            if "enum" in t:
                mod_code.append("#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]")
                mod_code.append(f"pub enum {safe_t_id} {{")
                for i, e in enumerate(t["enum"]):
                    var = to_camel_case(e)
                    if var == "Self": var = "SelfValue"
                    if i == 0: mod_code.append("    #[default]")
                    mod_code.append(f"    {var},")
                mod_code.append("}\n")
            elif t.get("type") == "object" and "properties" in t:
                mod_code.append("#[derive(Debug, Clone, Serialize, Deserialize, Default)]")
                mod_code.append('#[serde(rename_all = "camelCase")]')
                mod_code.append(f"pub struct {safe_t_id} {{")
                for p in t["properties"]:
                    mod_code.append(format_rustdoc(p.get("description"), 4))
                    p_name = p["name"]
                    r_type = get_rust_type(p, t_id).replace("serde_json::Value", "JsonValue")
                    if "Option<" in r_type: mod_code.append('    #[serde(skip_serializing_if = "Option::is_none")]')
                    if p_name in ["type", "override", "match", "return"]:
                        mod_code.append(f'    #[serde(rename = "{p_name}")]')
                        p_name = f"{p_name}_"
                    mod_code.append(f"    pub {p_name}: {r_type},")
                mod_code.append("}\n")
            else:
                r_type = get_rust_type(t, t_id).replace("serde_json::Value", "JsonValue")
                mod_code.append(f"pub type {safe_t_id} = {r_type};\n")

        for cmd in domain.get("commands", []):
            c_name = to_camel_case(cmd.get("name"))
            for suffix, key in [("Params", "parameters"), ("Returns", "returns")]:
                props = cmd.get(key, [])
                if props:
                    mod_code.append(format_rustdoc(cmd.get("description"), 0))
                    mod_code.append("#[derive(Debug, Clone, Serialize, Deserialize, Default)]")
                    mod_code.append('#[serde(rename_all = "camelCase")]')
                    mod_code.append(f"pub struct {c_name}{suffix} {{")
                    for p in props:
                        mod_code.append(format_rustdoc(p.get("description"), 4))
                        p_name = p["name"]
                        r_type = get_rust_type(p).replace("serde_json::Value", "JsonValue")
                        if "Option<" in r_type: mod_code.append('    #[serde(skip_serializing_if = "Option::is_none")]')
                        if p_name in ["type", "override", "match", "return"]:
                            mod_code.append(f'    #[serde(rename = "{p_name}")]')
                            p_name = f"{p_name}_"
                        mod_code.append(f"    pub {p_name}: {r_type},")
                    mod_code.append("}\n")

        with open(os.path.join(domain_dir, "mod.rs"), "w", encoding="utf-8") as f:
            f.write("\n".join(mod_code))

    with open(os.path.join(src_dir, "lib.rs"), "w", encoding="utf-8") as f:
        f.write("\n".join(lib_rs_content))

def check_cpp_build_tools():
    if platform.system().lower() != "windows": return True
    if shutil.which("cl.exe") or shutil.which("link.exe"): return True
    try:
        res = subprocess.run(["winget", "list", "Microsoft.VisualStudio"], capture_output=True, text=True, shell=True)
        return res.returncode == 0 and "Microsoft.VisualStudio" in res.stdout
    except: return False

def check_cargo_exist(): return shutil.which("cargo") is not None

def add_dependencies(project_name, deps):
    project_path = ".."
    for lib, feature in deps.items():
        cmd = ["cargo", "add", lib]
        if feature: cmd.extend(["--features", feature])
        subprocess.run(cmd, cwd=project_path, capture_output=True)

def update_cargo_metadata(project_name):
    project_path = ".."
    path = os.path.join(project_path, "Cargo.toml")
    with open(path, "r", encoding="utf-8") as f: lines = f.readlines()
    
    new_lines = []
    for line in lines:
        new_lines.append(line)
        if line.strip() == "[package]":
            new_lines.append(f'authors = ["AzzoDude"]\n')
            new_lines.append(f'description = "Generated Rust types and commands for the Chrome DevTools Protocol ({project_name})"\n')
            new_lines.append(f'license = "MIT"\n')
            new_lines.append(f'repository = "https://github.com/AzzoDude/{project_name}"\n')
            new_lines.append(f'readme = "README.md"\n')
            new_lines.append(f'keywords = ["cdp", "browser", "automation", "protocol"]\n')
            new_lines.append(f'categories = ["development-tools", "web-programming"]\n')
            
    # Add profile release optimizations (existing logic)
    if not any("[profile.release]" in l for l in lines):
        new_lines.append('\n[profile.release]\nopt-level = 3\nlto = "fat"\ncodegen-units = 1\npanic = "abort"\nstrip = true\n')

    with open(path, "w", encoding="utf-8") as f: f.writelines(new_lines)

def generate_readme(project_name):
    project_path = ".."
    path = os.path.join(project_path, "README.md")
    content = f"""# {project_name}

[![Crates.io](https://img.shields.io/crates/v/{project_name}.svg)](https://crates.io/crates/{project_name})
[![Documentation](https://docs.rs/{project_name}/badge.svg)](https://docs.rs/{project_name})
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
{project_name} = "0.1.0"
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
```

## 🛠 Usage Example

```rust
use {project_name}::dom::GetDocumentParams;
use {project_name}::page::NavigateParams;

fn main() {{
    // Example: Create a navigation command
    let nav = NavigateParams {{
        url: "https://www.rust-lang.org".to_string(),
        ..Default::default()
    }};
    
    println!("Request: {{:?}}", serde_json::to_string(&nav).unwrap());
}}
```

## 🏗 How it was built
This crate is automatically generated using a custom Python script that parses the `browser_protocol.json` and produces idiomatic Rust modules.

## ⚖ License
Distributed under the MIT License. See `LICENSE` for more information.

---
*Disclaimer: This is an automatically generated project. Always check the official CDP documentation for the latest protocol changes.*
"""
    with open(path, "w", encoding="utf-8") as f: f.write(content)

def generate_gitignore(project_name):
    project_path = ".."
    path = os.path.join(project_path, ".gitignore")
    # Standard Rust + VS Code + Visual Studio gitignore
    content = """# Created by https://www.toptal.com/developers/gitignore/api/rust,visualstudio,visualstudiocode

### Rust ###
debug/
target/
Cargo.lock
**/*.rs.bk
*.pdb

### VisualStudioCode ###
.vscode/*
!.vscode/settings.json
!.vscode/tasks.json
!.vscode/launch.json
!.vscode/extensions.json
!.vscode/*.code-snippets
.history/
*.vsix

### VisualStudio ###
.vs/
[Dd]ebug/
[Rr]elease/
x64/
x86/
[Ww][Ii][Nn]32/
[Aa][Rr][Mm]/
[Aa][Rr][Mm]64/
*.obj
*.log
*.tlog
*.vspscc
*.vssscc
.builds
"""
    with open(path, "w", encoding="utf-8") as f: f.write(content)

def build_project(project_name, release):
    project_path = ".."
    cmd = ["cargo", "build"]
    if release: cmd.append("--release")
    subprocess.run(cmd, cwd=project_path)

def init_rust_lib(project_name, is_release, source_only):
    try:
        project_path = ".."
        if not os.path.exists(project_path): os.makedirs(project_path)
        
        # Only init if Cargo.toml doesn't exist
        if not os.path.exists(os.path.join(project_path, "Cargo.toml")):
            subprocess.run(["cargo", "init", "--lib", "--name", project_name], cwd=project_path, check=True, capture_output=True)
            add_dependencies(project_name, dependencies_rs)
            
        update_cargo_metadata(project_name)
        generate_readme(project_name)
        generate_gitignore(project_name)
        print(f"Project '{project_name}' updated in {project_path} with publishing metadata and .gitignore.")
        generate_cdp_modules(project_name)
        if not source_only: build_project(project_name, is_release)
        return True
    except Exception as e:
        print(f"Error: {e}")
        return False

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--name", type=str, required=True)
    parser.add_argument("--release", action="store_true")
    parser.add_argument("--source", action="store_true")
    args = parser.parse_args()
    if not args.source and not check_cpp_build_tools(): sys.exit("Error: MSVC not found.")
    if not check_cargo_exist(): sys.exit("Error: Cargo not installed.")
    init_rust_lib(args.name, args.release, args.source)
