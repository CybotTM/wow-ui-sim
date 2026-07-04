#!/usr/bin/env python3
"""Generate Rust stub registrations for missing WoW API functions.

Reads:  ~/Repos/wowless/data/products/wow/apis.yaml
Writes: src/lua_api/globals/generated_stubs.rs

Usage: python3 scripts/generate_stubs.py
"""

import re
import sys
from collections import defaultdict
from pathlib import Path

try:
    import yaml
except ImportError:
    print("PyYAML required: pip install pyyaml", file=sys.stderr)
    sys.exit(1)

PROJECT_ROOT = Path(__file__).resolve().parent.parent
APIS_YAML = Path.home() / "Repos/wowless/data/products/wow/apis.yaml"
GLOBALS_DIR = PROJECT_ROOT / "src" / "lua_api" / "globals"
LEGACY_FILE = PROJECT_ROOT / "src" / "lua_api" / "globals_legacy.rs"
OUTPUT_FILE = GLOBALS_DIR / "generated_stubs.rs"


def load_apis():
    """Load and return the apis.yaml data."""
    with open(APIS_YAML) as f:
        return yaml.safe_load(f)


## Globals provided by Elune's C runtime (not visible to Rust source scanner).
ELUNE_PROVIDED = {"securecall", "securecallfunction", "secureexecuterange"}


def extract_existing():
    """Extract global-level names already registered in hand-written Rust code.

    Only matches registrations on the globals table (variables named `globals`
    or `g`), not on C_ namespace sub-tables (named `t`, `c_timer`, etc.).
    """
    names = set(ELUNE_PROVIDED)
    sources = list(GLOBALS_DIR.glob("*.rs")) + [LEGACY_FILE]
    for rs_file in sources:
        if rs_file.name == "generated_stubs.rs":
            continue
        if not rs_file.exists():
            continue
        content = rs_file.read_text()
        for m in re.finditer(r'\b(?:globals|g)\.set\(\s*"([^"]+)"', content):
            names.add(m.group(1))
    return names


def type_category(type_val):
    """Determine the category of a YAML type value."""
    if isinstance(type_val, str):
        return type_val
    if isinstance(type_val, dict):
        for key in ("structure", "enum", "arrayof", "uiobject"):
            if key in type_val:
                return key
    return "unknown"


def value_expr(output):
    """Return a Rust Value expression for a single output's default."""
    if output.get("nilable", False):
        return "Value::Nil"
    cat = type_category(output.get("type", "unknown"))
    mapping = {
        "number": "Value::Integer(0)",
        "string": 'Value::String(lua.create_string("")?)',
        "boolean": "Value::Boolean(false)",
        "table": "Value::Table(lua.create_table()?)",
        "structure": "Value::Table(lua.create_table()?)",
        "arrayof": "Value::Table(lua.create_table()?)",
        "enum": "Value::Integer(0)",
    }
    return mapping.get(cat, "Value::Nil")


def needs_lua(expr):
    """Check if a Value expression requires the lua closure parameter."""
    return "lua." in expr


def closure_str(outputs):
    """Return the Rust closure source for a stub function."""
    if not outputs:
        return "|_, _: MultiValue| Ok(())"

    if len(outputs) == 1:
        expr = value_expr(outputs[0])
        lua_p = "lua" if needs_lua(expr) else "_"
        return f"|{lua_p}, _: MultiValue| Ok({expr})"

    # Multiple return values — use MultiValue
    exprs = [value_expr(o) for o in outputs]
    any_lua = any(needs_lua(e) for e in exprs)
    lua_p = "lua" if any_lua else "_lua"
    items = ", ".join(exprs)
    return f"|{lua_p}, _: MultiValue| Ok(mlua::MultiValue::from_vec(vec![{items}]))"


def ns_fn_name(ns):
    """Convert a namespace like C_AccountInfo to a Rust function name."""
    if ns.startswith("C_"):
        prefix = "c_"
        rest = ns[2:]
    else:
        prefix = ""
        rest = ns
    # CamelCase to snake_case
    snake = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", rest)
    snake = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", snake)
    return f"register_{prefix}{snake.lower()}"


def parse_apis(apis):
    """Split apis.yaml entries into globals and namespace methods."""
    globals_funcs = {}
    c_namespaces = defaultdict(dict)

    for name, spec in apis.items():
        spec = spec or {}
        outputs = spec.get("outputs") or []
        entry = {"outputs": outputs}

        if "." in name:
            ns, method = name.split(".", 1)
            c_namespaces[ns][method] = entry
        else:
            globals_funcs[name] = entry

    return globals_funcs, dict(c_namespaces)


TOOLTIP_DATA_CLOSURE = (
    "|lua, _: MultiValue| {\n"
    "            let t = lua.create_table()?;\n"
    '            t.set("type", 0)?;\n'
    '            t.set("lines", lua.create_table()?)?;\n'
    "            Ok(Value::Table(t))\n"
    "        }"
)


def emit_stub_line(target_var, name, outputs, indent="    ", ns=None):
    """Emit the if-nil-then-set lines for one stub."""
    if ns == "C_TooltipInfo":
        cl = TOOLTIP_DATA_CLOSURE
    else:
        cl = closure_str(outputs)
    return (
        f'{indent}if {target_var}.get::<Value>("{name}")?.is_nil() {{\n'
        f'{indent}    {target_var}.set("{name}", lua.create_function({cl})?)?;\n'
        f"{indent}}}"
    )


def generate_rust(globals_funcs, c_namespaces, existing):
    """Generate the complete Rust source file."""
    lines = []
    w = lines.append

    w("// Auto-generated by scripts/generate_stubs.py -- do not edit manually")
    w("// Provides default stub implementations for WoW API functions not yet")
    w("// implemented in hand-written Rust code. Each stub checks is_nil() before")
    w("// setting, so hand-written implementations always take priority.")
    w("")
    w("use mlua::{Lua, MultiValue, Result, Value};")
    w("")

    # --- Main entry point ---
    w("pub fn register_generated_stubs(lua: &Lua) -> Result<()> {")
    w("    let g = lua.globals();")
    w("    register_global_stubs(lua, &g)?;")
    w("    register_c_stubs(lua, &g)?;")
    w("    Ok(())")
    w("}")
    w("")

    # --- Global stubs (split into chunks) ---
    global_names = sorted(n for n in globals_funcs if n not in existing)
    chunk_size = 40
    chunks = [
        global_names[i : i + chunk_size]
        for i in range(0, len(global_names), chunk_size)
    ]

    w("fn register_global_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {")
    for i in range(len(chunks)):
        w(f"    register_global_stubs_{i}(lua, g)?;")
    w("    Ok(())")
    w("}")
    w("")

    for i, chunk in enumerate(chunks):
        w(f"fn register_global_stubs_{i}(lua: &Lua, g: &mlua::Table) -> Result<()> {{")
        for name in chunk:
            w(emit_stub_line("g", name, globals_funcs[name]["outputs"]))
        w("    Ok(())")
        w("}")
        w("")

    # --- C_ namespace stubs ---
    ns_names = sorted(c_namespaces.keys())

    w("fn register_c_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {")
    for ns in ns_names:
        w(f"    {ns_fn_name(ns)}(lua, g)?;")
    w("    Ok(())")
    w("}")
    w("")

    for ns in ns_names:
        methods = c_namespaces[ns]
        fn_name = ns_fn_name(ns)
        method_names = sorted(methods.keys())

        if len(method_names) > 40:
            # Split large namespaces into sub-functions
            sub_chunks = [
                method_names[j : j + 40] for j in range(0, len(method_names), 40)
            ]

            w(f"fn {fn_name}(lua: &Lua, g: &mlua::Table) -> Result<()> {{")
            w(f'    let t: mlua::Table = match g.get::<Value>("{ns}")? {{')
            w("        Value::Table(t) => t,")
            w("        _ => lua.create_table()?,")
            w("    };")
            for j in range(len(sub_chunks)):
                w(f"    {fn_name}_{j}(lua, &t)?;")
            w(f'    g.set("{ns}", t)?;')
            w("    Ok(())")
            w("}")
            w("")

            for j, sub in enumerate(sub_chunks):
                w(f"fn {fn_name}_{j}(lua: &Lua, t: &mlua::Table) -> Result<()> {{")
                for m in sub:
                    w(emit_stub_line("t", m, methods[m]["outputs"], ns=ns))
                w("    Ok(())")
                w("}")
                w("")
        else:
            w(f"fn {fn_name}(lua: &Lua, g: &mlua::Table) -> Result<()> {{")
            w(f'    let t: mlua::Table = match g.get::<Value>("{ns}")? {{')
            w("        Value::Table(t) => t,")
            w("        _ => lua.create_table()?,")
            w("    };")
            for m in method_names:
                w(emit_stub_line("t", m, methods[m]["outputs"], ns=ns))
            w(f'    g.set("{ns}", t)?;')
            w("    Ok(())")
            w("}")
            w("")

    return "\n".join(lines) + "\n"


def main():
    print("Loading apis.yaml...")
    apis = load_apis()
    print(f"  {len(apis)} API entries")

    print("Extracting existing registrations...")
    existing = extract_existing()
    print(f"  {len(existing)} existing names")

    print("Parsing functions...")
    globals_funcs, c_namespaces = parse_apis(apis)
    total_methods = sum(len(m) for m in c_namespaces.values())
    new_globals = sum(1 for n in globals_funcs if n not in existing)
    print(f"  {len(globals_funcs)} globals ({new_globals} new)")
    print(f"  {len(c_namespaces)} namespaces, {total_methods} methods")

    print("Generating Rust code...")
    code = generate_rust(globals_funcs, c_namespaces, existing)

    OUTPUT_FILE.write_text(code)
    line_count = code.count("\n")
    print(f"  Wrote {OUTPUT_FILE}")
    print(f"  {len(code):,} bytes, {line_count:,} lines")


if __name__ == "__main__":
    main()
