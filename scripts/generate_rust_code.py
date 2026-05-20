import subprocess
import shutil
import os
import platform
import sys
import argparse
import json
import re

dependencies_rs: dict[str, str] = {
    "serde": "derive",
    "serde_json": ""
}

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(SCRIPT_DIR)

def to_camel_case(snake_str):
    components = snake_str.replace('-', '_').split('_')
    return "".join(x[:1].upper() + x[1:] for x in components if x)

def to_snake_case(name):
    # Treat acronyms like DOM, AX, ID, HTML, etc., as separate words
    s = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', name)
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', s)
    s = s.lower()
    s = s.replace('-', '_')
    s = re.sub(r'_+', '_', s)
    # Rust keywords list
    keywords = {
        "type", "override", "match", "return", "ref", "fn", "impl", "struct", "enum",
        "move", "loop", "const", "self", "as", "where", "for", "in", "unsafe", "pub",
        "use", "mod", "trait", "let", "mut", "static", "dyn", "async", "await", "true",
        "false"
    }
    if s in keywords:
        s = f"{s}_"
    return s

def wrap_bare_urls(text):
    url_pattern = re.compile(r'https?://[^\s"\'<>`]+')
    
    last_idx = 0
    result = []
    
    for match in url_pattern.finditer(text):
        url = match.group(0)
        start = match.start()
        end = match.end()
        
        result.append(text[last_idx:start])
        
        trailing = ""
        while url:
            last_char = url[-1]
            if last_char in ['.', ',', ':', ';', '?', '!', ')', ']']:
                if last_char == ')' and url.count('(') >= url.count(')'):
                    break
                if last_char == ']' and url.count('[') >= url.count(']'):
                    break
                trailing = last_char + trailing
                url = url[:-1]
                end -= 1
            else:
                break
        
        pre_text = text[:start]
        post_text = text[end:]
        
        is_markdown_link = False
        if pre_text.endswith('('):
            if pre_text[:-1].endswith(']'):
                is_markdown_link = True
                
        is_already_wrapped = pre_text.endswith('<') and post_text.startswith('>')
        
        is_quoted = pre_text.endswith('`') and post_text.startswith('`')
            
        if not is_markdown_link and not is_already_wrapped and not is_quoted:
            result.append(f"<{url}>{trailing}")
        else:
            result.append(match.group(0))
            
        last_idx = match.end()
        
    result.append(text[last_idx:])
    return "".join(result)

def escape_html_brackets(text):
    placeholders = []
    def protect(match):
        placeholders.append(match.group(0))
        return f"__LINK_PLACEHOLDER_{len(placeholders)-1}__"
        
    # Protect <http...> and <https...> links
    protected_text = re.sub(r'<https?://[^\s>]+>', protect, text)
    
    # Protect email links
    protected_text = re.sub(r'<[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+>', protect, protected_text)
    
    # Escape any remaining '<' and '>'
    escaped_text = protected_text.replace('<', r'\<').replace('>', r'\>')
    
    # Restore the protected links
    for i, placeholder in enumerate(placeholders):
        escaped_text = escaped_text.replace(f"__LINK_PLACEHOLDER_{i}__", placeholder)
        
    return escaped_text

def escape_markdown_brackets(text):
    placeholders = []
    def protect(match):
        placeholders.append(match.group(0))
        return f"__BRACKET_PLACEHOLDER_{len(placeholders)-1}__"
        
    # Protect markdown links like [text](url) and ![alt](url)
    protected_text = re.sub(r'!?\[[^\]]*\]\([^\)]+\)', protect, text)
    
    # Escape remaining '[' and ']'
    escaped_text = protected_text.replace('[', r'\[').replace(']', r'\]')
    
    # Restore
    for i, placeholder in enumerate(placeholders):
        escaped_text = escaped_text.replace(f"__BRACKET_PLACEHOLDER_{i}__", placeholder)
        
    return escaped_text

def format_rustdoc(description, indent_level=0, is_inner=False):
    if not description: return ""
    
    # Wrap bare URLs in angle brackets
    description = wrap_bare_urls(description)
    
    # Escape raw HTML brackets to prevent unclosed HTML tag warnings
    description = escape_html_brackets(description)
    
    # Escape raw square brackets to prevent unresolved link warnings
    description = escape_markdown_brackets(description)
    
    indent = " " * indent_level
    symbol = "//! " if is_inner else "/// "
    clean_text = description.replace("\\n", "\n").replace("`", "'")
    lines = clean_text.split("\n")
    doc_lines = []
    for line in lines:
        clean_line = line.strip()
        doc_lines.append(f"{indent}{symbol}{clean_line}" if clean_line else f"{indent}{symbol}")
    return "\n".join(doc_lines) + "\n"

def check_property_lifetime(prop, current_domain, lifetime_keys):
    if "$ref" in prop:
        ref = prop["$ref"]
        if "." in ref:
            ref_domain, ref_name = ref.split(".")
            if ref_name == "Value": ref_name = "ProtocolValue"
            ref_key = (ref_domain.lower(), ref_name)
        else:
            ref_name = ref
            if ref_name == "Value": ref_name = "ProtocolValue"
            ref_key = (current_domain.lower(), ref_name)
        return ref_key in lifetime_keys
    
    p_type = prop.get("type")
    if p_type == "string":
        return True
    elif p_type == "array":
        return check_property_lifetime(prop.get("items", {}), current_domain, lifetime_keys)
    
    return False

def get_rust_type(prop, current_domain, current_struct_name=None, lifetime_keys=set()):
    base_type = "JsonValue"
    is_recursive = False
    if "$ref" in prop:
        ref = prop["$ref"]
        if "." in ref:
            domain, t_name = ref.split(".")
            if t_name == "Value": t_name = "ProtocolValue"
            base_type = f"crate::{domain.lower()}::{t_name}"
            ref_key = (domain.lower(), t_name)
            if t_name == current_struct_name: is_recursive = True
        else:
            base_type = ref
            if ref == "Value": base_type = "ProtocolValue"
            ref_key = (current_domain.lower(), base_type)
            if base_type == current_struct_name: is_recursive = True
        
        if ref_key in lifetime_keys:
            base_type = f"{base_type}<'a>"
            
    elif prop.get("type") == "string":
        base_type = "Cow<'a, str>"
    elif prop.get("type") == "number":
        base_type = "f64"
    elif prop.get("type") == "boolean":
        base_type = "bool"
    elif prop.get("type") == "any":
        base_type = "JsonValue"
    elif prop.get("type") == "array":
        item_type = get_rust_type(prop.get("items", {}), current_domain, current_struct_name, lifetime_keys)
        base_type = f"Vec<{item_type}>"
    elif prop.get("type") == "integer":
        name = prop.get("name", "").lower()
        if any(k in name for k in ["delta", "offset"]) or name in ["x", "y"]: base_type = "i32"
        elif any(k in name for k in ["id", "count", "index", "size", "length"]): base_type = "u64"
        else: base_type = "i64"
    elif prop.get("type") == "object":
        base_type = "serde_json::Map<String, JsonValue>"
        
    if is_recursive:
        base_type = f"Box<{base_type}>"
    if prop.get("optional", False):
        return f"Option<{base_type}>"
    return base_type

def generate_getter_method(rust_name, r_type, doc_comment):
    comment = doc_comment if doc_comment else ""
    if r_type.startswith("Option<Box<") and r_type.endswith(">>"):
        inner = r_type[11:-2]
        return f"{comment}    pub fn {rust_name}(&self) -> Option<&{inner}> {{ self.{rust_name}.as_deref() }}"
    elif r_type.startswith("Box<") and r_type.endswith(">"):
        inner = r_type[4:-1]
        return f"{comment}    pub fn {rust_name}(&self) -> &{inner} {{ &self.{rust_name} }}"
    elif r_type in ["Option<Cow<'a, str>>", "Option<std::borrow::Cow<'a, str>>"]:
        return f"{comment}    pub fn {rust_name}(&self) -> Option<&str> {{ self.{rust_name}.as_deref() }}"
    elif r_type in ["Cow<'a, str>", "std::borrow::Cow<'a, str>"]:
        return f"{comment}    pub fn {rust_name}(&self) -> &str {{ self.{rust_name}.as_ref() }}"
    elif r_type.startswith("Option<Vec<") and r_type.endswith(">>"):
        inner = r_type[11:-2]
        return f"{comment}    pub fn {rust_name}(&self) -> Option<&[{inner}]> {{ self.{rust_name}.as_deref() }}"
    elif r_type.startswith("Vec<") and r_type.endswith(">"):
        inner = r_type[4:-1]
        return f"{comment}    pub fn {rust_name}(&self) -> &[{inner}] {{ &self.{rust_name} }}"
    elif r_type in ["i64", "u64", "i32", "u32", "f64", "bool", "Option<i64>", "Option<u64>", "Option<i32>", "Option<u32>", "Option<f64>", "Option<bool>"]:
        return f"{comment}    pub fn {rust_name}(&self) -> {r_type} {{ self.{rust_name} }}"
    elif r_type.startswith("Option<") and r_type.endswith(">"):
        inner = r_type[7:-1]
        return f"{comment}    pub fn {rust_name}(&self) -> Option<&{inner}> {{ self.{rust_name}.as_ref() }}"
    else:
        return f"{comment}    pub fn {rust_name}(&self) -> &{r_type} {{ &self.{rust_name} }}"

def is_string_type(t_name, current_domain, string_types):
    clean = t_name
    if clean.startswith("Option<"):
        clean = clean[7:-1]
    if clean.startswith("Box<"):
        clean = clean[4:-1]
    if clean.endswith("<'a>"):
        clean = clean[:-4]
    
    if clean == "Cow<'a, str>" or clean == "std::borrow::Cow<'a, str>":
        return True
        
    if clean.startswith("crate::"):
        parts = clean.split("::")
        if len(parts) == 4:
            key = (parts[2].lower(), parts[3])
            return key in string_types
    else:
        key = (current_domain.lower(), clean)
        return key in string_types
    return False

def generate_struct_with_builder(struct_name, props, current_domain, lifetime_keys, string_types):
    if not props:
        return f"""#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct {struct_name} {{}}
"""

    has_lifetime = (current_domain.lower(), struct_name) in lifetime_keys
    lifetime_suffix = "<'a>" if has_lifetime else ""
    impl_lifetime = "<'a>" if has_lifetime else ""
    
    fields_def = []
    builder_fields_def = []
    builder_args = []
    builder_inits = []
    setter_methods = []
    build_assignments = []
    getter_methods = []
    
    for p in props:
        p_name = p["name"]
        rust_name = to_snake_case(p_name)
        r_type = get_rust_type(p, current_domain, struct_name, lifetime_keys)
        is_opt = p.get("optional", False)
        
        doc = format_rustdoc(p.get("description"), 4)
        
        serde_attrs = []
        if is_opt:
            serde_attrs.append('skip_serializing_if = "Option::is_none"')
        if p_name != rust_name:
            serde_attrs.append(f'rename = "{p_name}"')
        
        serde_line = ""
        if serde_attrs:
            serde_line = f"    #[serde({', '.join(serde_attrs)})]\n"
        
        fields_def.append(f"{doc}{serde_line}    {rust_name}: {r_type},")
        getter_methods.append(generate_getter_method(rust_name, r_type, doc))
        
        if is_opt:
            b_type = r_type
            builder_fields_def.append(f"    {rust_name}: {b_type},")
            builder_inits.append(f"            {rust_name}: None,")
            
            inner_type = r_type[7:-1]
            is_string = is_string_type(inner_type, current_domain, string_types)
            if is_string:
                arg_type = f"impl Into<{inner_type}>"
                setter_val = f"{rust_name}.into()"
            else:
                arg_type = inner_type
                setter_val = rust_name
                
            setter_doc = format_rustdoc(p.get("description"), 4)
            setter_methods.append(f"{setter_doc}    pub fn {rust_name}(mut self, {rust_name}: {arg_type}) -> Self {{ self.{rust_name} = Some({setter_val}); self }}")
            build_assignments.append(f"            {rust_name}: self.{rust_name},")
        else:
            b_type = r_type
            builder_fields_def.append(f"    {rust_name}: {b_type},")
            
            is_string = is_string_type(r_type, current_domain, string_types)
            if is_string:
                arg_type = f"impl Into<{r_type}>"
                init_val = f"{rust_name}.into()"
            else:
                arg_type = r_type
                init_val = rust_name
                
            builder_args.append(f"{rust_name}: {arg_type}")
            builder_inits.append(f"            {rust_name}: {init_val},")
            build_assignments.append(f"            {rust_name}: self.{rust_name},")
            
    body = []
    
    body.append("#[derive(Debug, Clone, Serialize, Deserialize, Default)]")
    body.append('#[serde(rename_all = "camelCase")]')
    body.append(f"pub struct {struct_name}{lifetime_suffix} {{")
    body.append("\n".join(fields_def))
    body.append("}\n")
    
    impl_body = []
    builder_args_str = ", ".join(builder_args)
    builder_inits_str = "\n".join(builder_inits)
    
    # Generate builder documentation listing parameters
    builder_doc_lines = ["    /// Creates a builder for this type with the required parameters:"]
    for p in props:
        if not p.get("optional", False):
            desc = p.get("description", "").replace("\n", " ")
            desc = wrap_bare_urls(desc)
            desc = escape_html_brackets(desc)
            desc = escape_markdown_brackets(desc)
            builder_doc_lines.append(f"    /// * `{to_snake_case(p['name'])}`: {desc}")
    builder_doc_str = "\n".join(builder_doc_lines) + "\n" if len(builder_doc_lines) > 1 else "    /// Creates a builder for this type.\n"
    
    impl_body.append(builder_doc_str + f"    pub fn builder({builder_args_str}) -> {struct_name}Builder{lifetime_suffix} {{")
    impl_body.append(f"        {struct_name}Builder {{")
    impl_body.append(builder_inits_str)
    impl_body.append("        }")
    impl_body.append("    }")
    
    for g in getter_methods:
        impl_body.append(g)
        
    body.append(f"impl{impl_lifetime} {struct_name}{lifetime_suffix} {{")
    body.append("\n".join(impl_body))
    body.append("}\n")
    
    builder_derive_default = "#[derive(Default)]" if not builder_args else ""
    body.append(builder_derive_default)
    body.append(f"pub struct {struct_name}Builder{lifetime_suffix} {{")
    body.append("\n".join(builder_fields_def))
    body.append("}\n")
    
    builder_impl_body = []
    for s in setter_methods:
        builder_impl_body.append(s)
        
    build_assign_str = "\n".join(build_assignments)
    builder_impl_body.append(f"""    pub fn build(self) -> {struct_name}{lifetime_suffix} {{
        {struct_name} {{
{build_assign_str}
        }}
    }}""")
    
    body.append(f"impl{impl_lifetime} {struct_name}Builder{lifetime_suffix} {{")
    body.append("\n".join(builder_impl_body))
    body.append("}\n")
    
    return "\n".join(body)

def generate_cdp_modules(project_name: str):
    json_path = os.path.join(PROJECT_ROOT, "js_protocol.json")
    if not os.path.exists(json_path):
        json_path = os.path.join(PROJECT_ROOT, "browser_protocol.json")
    
    with open(json_path, "r", encoding="utf-8") as f:
        schema = json.load(f)

    project_path = PROJECT_ROOT
    src_dir = os.path.join(project_path, "src")
    lib_rs_content = [
        "#![allow(non_snake_case)]", "#![allow(unused_imports)]", "#![allow(dead_code)]", "",
        "use serde::{Serialize, Deserialize};", "use serde_json::Value as JsonValue;", "",
        "/// Trait for CDP commands that associate parameters with a method name and response type.",
        "pub trait CdpCommand<'a>: Serialize {", "    const METHOD: &'static str;", "    type Response: Deserialize<'a>;", "}", "",
        "/// A generic CDP command envelope.",
        "#[derive(Serialize)]", "pub struct Command<'a, T: CdpCommand<'a>> {", "    pub id: u64,", "    pub method: &'static str,", "    pub params: &'a T,", "}", "",
        "impl<'a, T: CdpCommand<'a>> Command<'a, T> {", "    pub fn new(id: u64, params: &'a T) -> Self {", "        Self { id, method: T::METHOD, params }", "    }", "}", "",
        "/// A generic CDP response envelope.",
        "#[derive(Deserialize, Debug)]", "pub struct Response<T> {", "    pub id: u64,", "    pub result: T,", "}", "",
        "/// An empty response for commands that don't return anything.",
        "#[derive(Deserialize, Debug, Clone, Default)]", "pub struct EmptyReturns {}", ""
    ]

    all_domains = [d.get("domain").lower() for d in schema.get("domains", [])]
    
    # ----------------------------------------------------
    # Lifetime Propagation Analysis Pass
    # ----------------------------------------------------
    all_types = {}
    
    # Pre-populate stub types if they are missing
    for stub in ["runtime", "debugger", "heapprofiler", "profiler"]:
        all_types[(stub, "RemoteObjectId")] = {"kind": "type", "def": {"type": "string"}}
        all_types[(stub, "RemoteObject")] = {"kind": "type", "def": {"type": "any"}}
        all_types[(stub, "ScriptId")] = {"kind": "type", "def": {"type": "string"}}
        all_types[(stub, "StackTrace")] = {"kind": "type", "def": {"type": "any"}}
        all_types[(stub, "UniqueDebuggerId")] = {"kind": "type", "def": {"type": "string"}}
        all_types[(stub, "SearchMatch")] = {"kind": "type", "def": {"type": "any"}}
        all_types[(stub, "ExecutionContextId")] = {"kind": "type", "def": {"type": "integer"}}
        all_types[(stub, "Timestamp")] = {"kind": "type", "def": {"type": "number"}}

    # Populate from schema
    for domain in schema.get("domains", []):
        d_name = domain.get("domain").lower()
        for t in domain.get("types", []):
            t_id = t.get("id")
            safe_t_id = f"Protocol{t_id}" if t_id == "Value" else t_id
            all_types[(d_name, safe_t_id)] = {
                "kind": "type",
                "def": t
            }
        for cmd in domain.get("commands", []):
            c_name = to_camel_case(cmd.get("name"))
            if cmd.get("parameters"):
                all_types[(d_name, f"{c_name}Params")] = {
                    "kind": "params",
                    "props": cmd.get("parameters")
                }
            if cmd.get("returns"):
                all_types[(d_name, f"{c_name}Returns")] = {
                    "kind": "returns",
                    "props": cmd.get("returns")
                }

    # Run fixed-point iteration for lifetimes
    lifetime_keys = set()
    changed = True
    while changed:
        changed = False
        for key, info in all_types.items():
            if key in lifetime_keys:
                continue
            
            has_lifetime = False
            domain_name = key[0]
            
            if info["kind"] == "type":
                t = info["def"]
                if "enum" in t:
                    has_lifetime = False
                elif t.get("type") == "object" and "properties" in t:
                    has_lifetime = any(check_property_lifetime(p, domain_name, lifetime_keys) for p in t["properties"])
                else:
                    has_lifetime = check_property_lifetime(t, domain_name, lifetime_keys)
            else:
                has_lifetime = any(check_property_lifetime(p, domain_name, lifetime_keys) for p in info["props"])
            
            if has_lifetime:
                lifetime_keys.add(key)
                changed = True

    # ----------------------------------------------------
    # String Type Alias Resolution Pass
    # ----------------------------------------------------
    string_types = set()
    for stub in ["runtime", "debugger", "heapprofiler", "profiler"]:
        string_types.add((stub, "RemoteObjectId"))
        string_types.add((stub, "ScriptId"))
        string_types.add((stub, "UniqueDebuggerId"))

    changed = True
    while changed:
        changed = False
        for domain in schema.get("domains", []):
            d_name = domain.get("domain").lower()
            for t in domain.get("types", []):
                t_id = t.get("id")
                safe_t_id = f"Protocol{t_id}" if t_id == "Value" else t_id
                key = (d_name, safe_t_id)
                if key in string_types:
                    continue
                
                is_str = False
                if t.get("type") == "string":
                    is_str = True
                elif "$ref" in t:
                    ref = t["$ref"]
                    if "." in ref:
                        ref_domain, ref_name = ref.split(".")
                        if ref_name == "Value": ref_name = "ProtocolValue"
                        ref_key = (ref_domain.lower(), ref_name)
                    else:
                        ref_name = ref
                        if ref_name == "Value": ref_name = "ProtocolValue"
                        ref_key = (d_name, ref_name)
                    is_str = ref_key in string_types
                
                if is_str:
                    string_types.add(key)
                    changed = True
    # ----------------------------------------------------

    # Write stub mods
    for stub in ["runtime", "debugger", "heapprofiler", "profiler"]:
        if stub not in all_domains:
            stub_dir = os.path.join(src_dir, stub)
            os.makedirs(stub_dir, exist_ok=True)
            with open(os.path.join(stub_dir, "mod.rs"), "w", encoding="utf-8") as f:
                f.write("use serde::{Serialize, Deserialize};\n")
                f.write("pub type RemoteObjectId<'a> = std::borrow::Cow<'a, str>;\npub type RemoteObject = serde_json::Value;\n")
                f.write("pub type ScriptId<'a> = std::borrow::Cow<'a, str>;\npub type StackTrace = serde_json::Value;\n")
                f.write("pub type UniqueDebuggerId<'a> = std::borrow::Cow<'a, str>;\npub type SearchMatch = serde_json::Value;\n")
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
                    mod_body.append(f'    #[serde(rename = "{e}")]')
                    mod_body.append(f"    {var},")
                mod_body.append("}\n")
            elif t.get("type") == "object" and "properties" in t:
                mod_body.append(generate_struct_with_builder(safe_t_id, t["properties"], d_name, lifetime_keys, string_types))
            else:
                r_type = get_rust_type(t, d_name, safe_t_id, lifetime_keys)
                has_lifetime = (d_name.lower(), safe_t_id) in lifetime_keys
                lifetime_suffix = "<'a>" if has_lifetime else ""
                mod_body.append(f"pub type {safe_t_id}{lifetime_suffix} = {r_type};\n")

        for cmd in domain.get("commands", []):
            raw_c_name = cmd.get("name")
            c_name = to_camel_case(raw_c_name)
            for suffix, key in [("Params", "parameters"), ("Returns", "returns")]:
                props = cmd.get(key, [])
                if props:
                    mod_body.append(format_rustdoc(cmd.get("description"), 0))
                    mod_body.append(generate_struct_with_builder(f"{c_name}{suffix}", props, d_name, lifetime_keys, string_types))

            if not cmd.get("parameters"):
                mod_body.append(generate_struct_with_builder(f"{c_name}Params", [], d_name, lifetime_keys, string_types))
            
            # CdpCommand impl
            has_lifetime_params = (d_name.lower(), f"{c_name}Params") in lifetime_keys
            lifetime_suffix_params = "<'a>" if has_lifetime_params else ""
            
            has_lifetime_returns = (d_name.lower(), f"{c_name}Returns") in lifetime_keys
            lifetime_suffix_returns = "<'a>" if has_lifetime_returns else ""
            
            # The trait CdpCommand<'a> always has a lifetime parameter, so impl must define it
            mod_body.append(f"impl{lifetime_suffix_params} {c_name}Params{lifetime_suffix_params} {{ pub const METHOD: &'static str = \"{d_name}.{raw_c_name}\"; }}\n")
            mod_body.append(f"impl<'a> crate::CdpCommand<'a> for {c_name}Params{lifetime_suffix_params} {{")
            mod_body.append(f"    const METHOD: &'static str = \"{d_name}.{raw_c_name}\";")
            if cmd.get("returns"):
                mod_body.append(f"    type Response = {c_name}Returns{lifetime_suffix_returns};")
            else:
                mod_body.append("    type Response = crate::EmptyReturns;")
            mod_body.append("}\n")

        # Handle mod headers (with module description //! before any imports)
        mod_header = []
        if "description" in domain:
            mod_header.append(format_rustdoc(domain['description'], 0, True))
            mod_header.append("")

        mod_code = mod_header + [
            "use serde::{Serialize, Deserialize};",
            "use serde_json::Value as JsonValue;",
            "use std::borrow::Cow;",
            "",
            "\n".join(mod_body)
        ]
        with open(os.path.join(domain_dir, "mod.rs"), "w", encoding="utf-8") as f:
            f.write("\n".join(mod_code))

    with open(os.path.join(src_dir, "lib.rs"), "w", encoding="utf-8") as f:
        f.write("\n".join(lib_rs_content))

def update_cargo_metadata(project_name):
    project_path = PROJECT_ROOT
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
        "version": '"0.1.3"'
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

    # Feature generation logic (specifically for js_protocol.json or browser_protocol.json)
    json_path = os.path.join(PROJECT_ROOT, "js_protocol.json")
    if not os.path.exists(json_path):
        json_path = os.path.join(PROJECT_ROOT, "browser_protocol.json")
    
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
