import subprocess
import shutil
import os
import platform
import sys
import argparse
import json
import re
import urllib.request

dependencies_rs: dict[str, str] = {
    "tokio": "full",
    "serde": "derive",
    "serde_json": ""
}

def to_camel_case(snake_str):
    components = snake_str.replace('-', '_').split('_')
    return "".join(x[:1].upper() + x[1:] for x in components if x)

def format_rustdoc(description, indent_level=0, is_inner=False):
    if not description: return ""
    indent = " " * indent_level
    symbol = "//! " if is_inner else "/// "
    clean_text = description.replace("\\n", "\n").replace("`", "'")
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
    elif prop.get("type") == "object": base_type = "serde_json::Map<String, serde_json::Value>"
    if is_recursive: base_type = f"Box<{base_type}>"
    if prop.get("optional", False): return f"Option<{base_type}>"
    return base_type

def generate_cdp_modules(project_name: str):
    json_path = "browser_protocol.json"
    parent_json = os.path.join("..", "browser_protocol.json")
    if os.path.exists(parent_json): json_path = parent_json
    
    with open(json_path, "r", encoding="utf-8") as f:
        schema = json.load(f)

    project_path = ".."
    src_dir = os.path.join(project_path, "src")
    lib_rs_content = [
        "#![allow(non_snake_case)]", "#![allow(unused_imports)]", "#![allow(dead_code)]", "",
        "use serde::{Serialize, Deserialize};", "use serde_json::Value as JsonValue;", "",
        "/// Trait for CDP commands that associate parameters with a method name and response type.",
        "pub trait CdpCommand: Serialize {", "    const METHOD: &'static str;", "    type Response: for<'de> Deserialize<'de>;", "}", "",
        "/// A generic CDP command envelope.",
        "#[derive(Serialize)]", "pub struct Command<'a, T: CdpCommand> {", "    pub id: u64,", "    pub method: &'static str,", "    pub params: &'a T,", "}", "",
        "impl<'a, T: CdpCommand> Command<'a, T> {", "    pub fn new(id: u64, params: &'a T) -> Self {", "        Self { id, method: T::METHOD, params }", "    }", "}", "",
        "/// A generic CDP response envelope.",
        "#[derive(Deserialize, Debug)]", "pub struct Response<T> {", "    pub id: u64,", "    pub result: T,", "}", "",
        "/// An empty response for commands that don't return anything.",
        "#[derive(Deserialize, Debug, Clone, Default)]", "pub struct EmptyReturns {}", ""
    ]

    all_domains = [d.get("domain").lower() for d in schema.get("domains", [])]
    for stub in ["runtime", "debugger", "heapprofiler", "profiler"]:
        if stub not in all_domains:
            stub_dir = os.path.join(src_dir, stub)
            os.makedirs(stub_dir, exist_ok=True)
            with open(os.path.join(stub_dir, "mod.rs"), "w", encoding="utf-8") as f:
                f.write("use serde::{Serialize, Deserialize};\n")
                f.write("pub type RemoteObjectId = String;\npub type RemoteObject = serde_json::Value;\n")
                f.write("pub type ScriptId = String;\npub type StackTrace = serde_json::Value;\n")
                f.write("pub type UniqueDebuggerId = String;\npub type SearchMatch = serde_json::Value;\n")
                f.write("pub type ExecutionContextId = i64;\npub type Timestamp = f64;\n")
            lib_rs_content.append(f'#[cfg(feature = "{stub}")]')
            lib_rs_content.append(f"pub mod {stub};")

    for domain in schema.get("domains", []):
        d_name = domain.get("domain")
        if d_name.lower() in ["webmcp"]: continue
        lib_rs_content.append(f'#[cfg(feature = "{d_name.lower()}")]')
        lib_rs_content.append(f"pub mod {d_name.lower()};")
        domain_dir = os.path.join(src_dir, d_name.lower())
        os.makedirs(domain_dir, exist_ok=True)
        
        mod_body = []
        if "description" in domain: mod_body.append(format_rustdoc(domain['description'], 0, True))

        for t in domain.get("types", []):
            mod_body.append(format_rustdoc(t.get("description"), 0))
            t_id = t.get("id")
            safe_t_id = f"Protocol{t_id}" if t_id == "Value" else t_id
            if "enum" in t:
                mod_body.append("#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]")
                mod_body.append(f"pub enum {safe_t_id} {{")
                for i, e in enumerate(t["enum"]):
                    var = to_camel_case(e)
                    if var == "Self": var = "SelfValue"
                    if i == 0: mod_body.append("    #[default]")
                    mod_body.append(f"    {var},")
                mod_body.append("}\n")
            elif t.get("type") == "object" and "properties" in t:
                mod_body.append("#[derive(Debug, Clone, Serialize, Deserialize, Default)]")
                mod_body.append('#[serde(rename_all = "camelCase")]')
                mod_body.append(f"pub struct {safe_t_id} {{")
                for p in t["properties"]:
                    mod_body.append(format_rustdoc(p.get("description"), 4))
                    p_name = p["name"]
                    r_type = get_rust_type(p, t_id).replace("serde_json::Value", "JsonValue")
                    if "Option<" in r_type: mod_body.append('    #[serde(skip_serializing_if = "Option::is_none")]')
                    if p_name in ["type", "override", "match", "return"]:
                        mod_body.append(f'    #[serde(rename = "{p_name}")]')
                        p_name = f"{p_name}_"
                    mod_body.append(f"    pub {p_name}: {r_type},")
                mod_body.append("}\n")
            else:
                r_type = get_rust_type(t, t_id).replace("serde_json::Value", "JsonValue")
                mod_body.append(f"pub type {safe_t_id} = {r_type};\n")

        for cmd in domain.get("commands", []):
            raw_c_name = cmd.get("name")
            c_name = to_camel_case(raw_c_name)
            for suffix, key in [("Params", "parameters"), ("Returns", "returns")]:
                props = cmd.get(key, [])
                if props:
                    mod_body.append(format_rustdoc(cmd.get("description"), 0))
                    mod_body.append("#[derive(Debug, Clone, Serialize, Deserialize, Default)]")
                    mod_body.append('#[serde(rename_all = "camelCase")]')
                    mod_body.append(f"pub struct {c_name}{suffix} {{")
                    for p in props:
                        mod_body.append(format_rustdoc(p.get("description"), 4))
                        p_name = p["name"]
                        r_type = get_rust_type(p).replace("serde_json::Value", "JsonValue")
                        if "Option<" in r_type: mod_body.append('    #[serde(skip_serializing_if = "Option::is_none")]')
                        if p_name in ["type", "override", "match", "return"]:
                            mod_body.append(f'    #[serde(rename = "{p_name}")]')
                            p_name = f"{p_name}_"
                        mod_body.append(f"    pub {p_name}: {r_type},")
                    mod_body.append("}\n")

            if not cmd.get("parameters"):
                mod_body.append(f"#[derive(Debug, Clone, Serialize, Deserialize, Default)]\npub struct {c_name}Params {{}}\n")
            
            mod_body.append(f"impl {c_name}Params {{ pub const METHOD: &'static str = \"{d_name}.{raw_c_name}\"; }}\n")
            mod_body.append(f"impl crate::CdpCommand for {c_name}Params {{")
            mod_body.append(f"    const METHOD: &'static str = \"{d_name}.{raw_c_name}\";")
            if cmd.get("returns"): mod_body.append(f"    type Response = {c_name}Returns;")
            else: mod_body.append("    type Response = crate::EmptyReturns;")
            mod_body.append("}\n")

        mod_code = ["use serde::{Serialize, Deserialize};", "use serde_json::Value as JsonValue;", "", "\n".join(mod_body)]
        with open(os.path.join(domain_dir, "mod.rs"), "w", encoding="utf-8") as f: f.write("\n".join(mod_code))

    with open(os.path.join(src_dir, "lib.rs"), "w", encoding="utf-8") as f:
        f.write("\n".join(lib_rs_content))

def update_cargo_metadata(project_name):
    project_path = ".."
    path = os.path.join(project_path, "Cargo.toml")
    with open(path, "r", encoding="utf-8") as f:
        content = f.read()
    
    metadata = {
        "authors": '["AzzoDude"]',
        "description": f'"Generated Rust types and commands for the Chrome DevTools Protocol ({project_name})"',
        "license": '"MIT"',
        "repository": f'"https://github.com/AzzoDude/{project_name}"',
        "readme": '"README.md"',
        "keywords": '["cdp", "browser", "automation", "protocol"]',
        "categories": '["development-tools", "web-programming"]',
        "version": '"0.1.2"'
    }

    lines = content.splitlines()
    new_lines = []
    in_package = False
    added_metadata = set()
    
    for line in lines:
        if line.strip() == "[package]":
            in_package = True
            new_lines.append(line)
            continue
        if in_package:
            if line.startswith("[") or line.strip() == "":
                for key, value in metadata.items():
                    if key not in added_metadata: new_lines.append(f"{key} = {value}")
                in_package = False
            else:
                key_part = line.split("=")[0].strip()
                if key_part in metadata:
                    new_lines.append(f"{key_part} = {metadata[key_part]}")
                    added_metadata.add(key_part)
                    continue
        new_lines.append(line)

    if in_package:
        for key, value in metadata.items():
            if key not in added_metadata: new_lines.append(f"{key} = {value}")

    # Feature generation logic
    json_path = os.path.join("..", "browser_protocol.json")
    if os.path.exists(json_path):
        with open(json_path, "r", encoding="utf-8") as f: schema = json.load(f)
        domains = [d.get("domain").lower() for d in schema.get("domains", [])]
        stubs = ["runtime", "debugger", "heapprofiler", "profiler"]
        all_features = sorted(list(set(domains + stubs)))
        
        processed_lines = []
        skip = False
        for l in new_lines:
            if l.strip() == "[features]": skip = True
            elif skip and l.startswith("["): skip = False
            if not skip: processed_lines.append(l)
        new_lines = processed_lines

        new_lines.append("\n[features]")
        new_lines.append('default = ["full"]')
        full_deps = ", ".join([f'"{f}"' for f in all_features])
        new_lines.append(f'full = [{full_deps}]')
        for f in all_features: new_lines.append(f'{f} = []')

    with open(path, "w", encoding="utf-8") as f: f.write("\n".join(new_lines) + "\n")

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--name", type=str, required=True)
    args = parser.parse_args()
    update_cargo_metadata(args.name)
    generate_cdp_modules(args.name)
