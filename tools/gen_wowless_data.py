#!/usr/bin/env python3
"""Generate the WowlessData addon from wowless YAML files.

Usage:
    python3 tools/gen_wowless_data.py

Reads YAML from ~/Repos/wowless/data/ and outputs to ./Interface/AddOns/WowlessData/
"""

import os
import re
import sys
from pathlib import Path

try:
    import yaml
except ImportError:
    print(
        "Error: PyYAML is required. Install with: pip install pyyaml", file=sys.stderr
    )
    sys.exit(1)

WOWLESS_DIR = Path(os.path.expanduser("~/Repos/wowless"))
OUTPUT_DIR = Path("./Interface/AddOns/WowlessData")
PRODUCT = "wow"


class _StringBoolLoader(yaml.SafeLoader):
    """YAML loader that keeps YAML 1.1 boolean strings (Off/On/Yes/No) as strings."""

    pass


# Remove the implicit bool resolvers that convert Off/On/Yes/No to Python bools.
# Keep only true/false (lowercase) as actual booleans, matching YAML 1.2 behavior.
_StringBoolLoader.yaml_implicit_resolvers = {
    k: [(tag, regexp) for tag, regexp in v if tag != "tag:yaml.org,2002:bool"]
    for k, v in yaml.SafeLoader.yaml_implicit_resolvers.copy().items()
}
# Re-add only true/false (YAML 1.2 booleans)
_StringBoolLoader.add_implicit_resolver(
    "tag:yaml.org,2002:bool",
    __import__("re").compile(r"^(?:true|false)$"),
    list("tf"),
)


def read_yaml(path: Path):
    with open(path) as f:
        return yaml.load(f, Loader=_StringBoolLoader)


def perproduct(product: str, filename: str):
    return read_yaml(WOWLESS_DIR / "data" / "products" / product / f"{filename}.yaml")


def read_file(path: Path) -> str:
    with open(path) as f:
        return f.read()


# Lua identifier regex
_IDENT_RE = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")
_LUA_KEYWORDS = {
    "and",
    "break",
    "do",
    "else",
    "elseif",
    "end",
    "false",
    "for",
    "function",
    "if",
    "in",
    "local",
    "nil",
    "not",
    "or",
    "repeat",
    "return",
    "then",
    "true",
    "until",
    "while",
}


def _is_safe_key(key: str) -> bool:
    return bool(_IDENT_RE.match(key)) and key not in _LUA_KEYWORDS


def _lua_str(s: str) -> str:
    s = s.replace("\\", "\\\\")
    s = s.replace("\n", "\\n")
    s = s.replace("\r", "\\r")
    s = s.replace("\t", "\\t")
    s = s.replace('"', '\\"')
    s = s.replace("\0", "\\0")
    return f'"{s}"'


def lua_repr(value, indent: int = 0) -> str:
    """Convert a Python value to a Lua table literal.

    None maps to {} (empty table), not nil. This matches wowless's native
    cyaml parser which converts YAML null values to empty Lua tables.
    Using {} preserves key existence in set-like tables (e.g. capsule.enums).
    """
    if value is None:
        return "{}"
    if value is True:
        return "true"
    if value is False:
        return "false"
    if isinstance(value, int):
        return str(value)
    if isinstance(value, float):
        if value == int(value):
            return str(int(value))
        return str(value)
    if isinstance(value, str):
        return _lua_str(value)
    if isinstance(value, list):
        if not value:
            return "{}"
        inner_indent = indent + 2
        parts = [lua_repr(v, inner_indent) for v in value]
        if (
            all("\n" not in p for p in parts)
            and sum(len(p) for p in parts) + len(parts) * 2 < 80
        ):
            return "{ " + ", ".join(parts) + " }"
        pad = " " * inner_indent
        lines = [f"{pad}{p}," for p in parts]
        close_pad = " " * indent
        return "{\n" + "\n".join(lines) + "\n" + close_pad + "}"
    if isinstance(value, dict):
        if not value:
            return "{}"
        inner_indent = indent + 2
        pad = " " * inner_indent
        close_pad = " " * indent
        lines = []

        # Sort: string keys alphabetically, then integer keys
        def sort_key(k):
            if isinstance(k, int):
                return (1, k, "")
            return (0, 0, str(k))

        for k in sorted(value.keys(), key=sort_key):
            v = value[k]
            v_repr = lua_repr(v, inner_indent)
            if isinstance(k, int):
                lines.append(f"{pad}[{k}] = {v_repr},")
            elif isinstance(k, str) and _is_safe_key(k):
                lines.append(f"{pad}{k} = {v_repr},")
            else:
                lines.append(f"{pad}[{_lua_str(str(k))}] = {v_repr},")
        return "{\n" + "\n".join(lines) + "\n" + close_pad + "}"
    raise TypeError(f"Cannot convert {type(value)} to Lua: {value!r}")


def gen_toc(product: str) -> str:
    build = perproduct(product, "build")
    tocversion = build["tocversion"]
    # product.lua first, then other files sorted alphabetically
    other_files = sorted(
        [
            "build.lua",
            "config.lua",
            "cvars.lua",
            "events.lua",
            "globalapis.lua",
            "globals.lua",
            "impltests.lua",
            "luaobjects.lua",
            "namespaceapis.lua",
            "uiobjectapis.lua",
        ]
    )
    files = ["product.lua"] + other_files
    lines = [f"## Interface: {tocversion}", ""] + files
    return "\n".join(lines) + "\n"


def gen_product(product: str) -> str:
    return f'_G.WowlessData = {{ product = "{product}" }}\n'


def gen_simple(product: str, filename: str, key: str) -> str:
    data = perproduct(product, filename)
    return f"_G.WowlessData.{key} = {lua_repr(data)}\n"


def gen_events(product: str) -> str:
    events_data = perproduct(product, "events") or {}
    t = {}
    for k, v in events_data.items():
        v = v or {}
        payload = v.get("payload") or []
        entry = {
            "callback": v.get("callback") or False,
            "payload": len(payload),
            "registerable": not v.get("noscript", False),
        }
        if "restricted" in v:
            entry["restricted"] = v["restricted"]
        t[k] = entry

    products_list = read_yaml(WOWLESS_DIR / "data" / "products.yaml")
    for other_product in products_list:
        other_events = perproduct(other_product, "events") or {}
        for k in other_events:
            if k not in t:
                t[k] = {
                    "callback": False,
                    "payload": -1,
                    "registerable": False,
                }

    return f"_G.WowlessData.Events = {lua_repr(t)}\n"


_MISSING = object()


def _tpath(d, *keys):
    """Follow a chain of dict keys; return _MISSING if any key is absent."""
    for k in keys:
        if not isinstance(d, dict):
            return _MISSING
        d = d.get(k, _MISSING)
        if d is _MISSING:
            return _MISSING
    return d


def gen_globalapis(product: str) -> str:
    config = perproduct(product, "config")
    apis = perproduct(product, "apis") or {}
    t = {}
    for name in apis:
        if "." not in name:
            if _tpath(config, "addon", "overwritten_apis", name) is not _MISSING:
                t[name] = {"overwritten": True}
            else:
                t[name] = True
    return f"_G.WowlessData.GlobalApis = {lua_repr(t)}\n"


def gen_impltests(product: str) -> str:
    test = read_yaml(WOWLESS_DIR / "data" / "test.yaml") or {}
    apis = perproduct(product, "apis") or {}
    t = {}
    for api in apis.values():
        if not api:
            continue
        impl = api.get("impl")
        if impl and impl in test and impl not in t:
            lua_path = WOWLESS_DIR / "data" / "test" / f"{impl}.lua"
            t[impl] = read_file(lua_path)
    return f"_G.WowlessData.ImplTests = {lua_repr(t)}\n"


def gen_luaobjects(product: str) -> str:
    raw = perproduct(product, "luaobjects") or {}
    t = {}

    def pop(k):
        if k in t:
            return
        v = raw.get(k) or {}
        methods = {}
        if v.get("inherits"):
            pop(v["inherits"])
            for mk in t[v["inherits"]]:
                methods[mk] = True
        for mk in v.get("methods") or {}:
            methods[mk] = True
        t[k] = methods

    for k in raw:
        pop(k)

    for k, v in raw.items():
        if v and v.get("virtual"):
            del t[k]

    return f"_G.WowlessData.LuaObjects = {lua_repr(t)}\n"


def gen_namespaceapis(product: str) -> str:
    config = perproduct(product, "config")
    apis = perproduct(product, "apis") or {}
    api_namespaces = {}
    for k, api in apis.items():
        dot = k.find(".")
        if dot == -1:
            continue
        api = api or {}
        if api.get("platform") or api.get("secureonly"):
            continue
        ns = k[:dot]
        method = k[dot + 1 :]
        if ns not in api_namespaces:
            api_namespaces[ns] = {}
        api_namespaces[ns][method] = api

    t = {}
    for ns, methods in api_namespaces.items():
        mt = {}
        for mk in methods:
            if (
                _tpath(config, "addon", "overwritten_apis", f"{ns}.{mk}")
                is not _MISSING
            ):
                mt[mk] = {"overwritten": True}
            else:
                mt[mk] = True
        t[ns] = mt

    return f"_G.WowlessData.NamespaceApis = {lua_repr(t)}\n"


def _normalize_uiobject(cfg):
    if cfg is None:
        cfg = {}
    if cfg.get("fieldinitoverrides") is None:
        cfg["fieldinitoverrides"] = {}
    if cfg.get("fields") is None:
        cfg["fields"] = {}
    if cfg.get("methods") is None:
        cfg["methods"] = {}
    if cfg.get("inherits") is None:
        cfg["inherits"] = {}
    # Normalize None values in methods and fields to empty dicts
    for k, v in list(cfg["methods"].items()):
        if v is None:
            cfg["methods"][k] = {}
    for k, v in list(cfg["fields"].items()):
        if v is None:
            cfg["fields"][k] = {}
    return cfg


def gen_uiobjectapis(product: str) -> str:
    uiobjects_raw = perproduct(product, "uiobjects") or {}
    allscripts = read_yaml(WOWLESS_DIR / "data" / "scripttypes.yaml") or {}

    # Normalize all objects
    uiobjects = {name: _normalize_uiobject(cfg) for name, cfg in uiobjects_raw.items()}

    # Build isa table for each object
    for name, cfg in uiobjects.items():
        cfg["isa"] = {}
        for name2, cfg2 in uiobjects.items():
            cfg["isa"][name2] = False
            if cfg2.get("objectType"):
                cfg["isa"][cfg2["objectType"]] = False
        if not cfg.get("virtual") or cfg.get("objectType"):
            cfg["isa"][cfg.get("objectType") or name] = True

    # Resolve inheritance recursively
    fixed = set()

    def fixup(name):
        if name in fixed:
            return
        fixed.add(name)
        cfg = uiobjects[name]
        for inhname in cfg["inherits"]:
            if inhname not in uiobjects:
                continue
            fixup(inhname)
            inh = uiobjects[inhname]
            for n, f in inh["fieldinitoverrides"].items():
                cfg["fieldinitoverrides"][n] = f
            for n, f in inh["fields"].items():
                cfg["fields"][n] = f
            for n, m in inh["methods"].items():
                if n not in cfg["methods"]:
                    cfg["methods"][n] = m
            if inh.get("scripts"):
                if cfg.get("scripts") is None:
                    cfg["scripts"] = {}
                for n in inh["scripts"]:
                    cfg["scripts"].setdefault(n, {})
            for ik, iv in inh["isa"].items():
                if iv:
                    cfg["isa"][ik] = True

    for name in uiobjects:
        fixup(name)

    t = {}
    for name, v in uiobjects.items():
        # Build fields table
        ft = {}
        for fk, fv in v["fields"].items():
            fv = fv or {}
            init = fv.get("init")
            override = v["fieldinitoverrides"].get(fk)
            if override is not None:
                init = override
            ft[fk] = {
                "dynamicinit": fv.get("dynamicinit"),
                "getters": [],
                "init": init,
            }

        # Build methods table and collect getters
        mt = {}
        for mk, mv in v["methods"].items():
            mt[mk] = True
            mv = mv or {}
            impl = mv.get("impl") or {}
            if isinstance(impl, dict):
                for idx, getter_entry in enumerate(impl.get("getter") or [], start=1):
                    field_name = getter_entry["name"]
                    if field_name in ft:
                        ft[field_name]["getters"].append({"index": idx, "method": mk})

        # Remove dynamicinit fields
        for fk in [k for k, fv in ft.items() if fv.get("dynamicinit")]:
            del ft[fk]

        # Remove parent field
        ft.pop("parent", None)

        # Singleton: empty fields
        if v.get("singleton"):
            ft = {}
        elif name == "EditBox" and "shown" in ft:
            ft["shown"]["init"] = False

        # Clean up dynamicinit from field entries (don't include None in output)
        for fv in ft.values():
            fv.pop("dynamicinit", None)
            if not fv.get("getters"):
                fv.pop("getters", None)

        # Build scripts table (only if HasScript in methods)
        st = {}
        if mt.get("HasScript"):
            scripts = v.get("scripts") or {}
            for scripttype in allscripts:
                st[scripttype] = scripttype in scripts

        entry = {
            "fields": ft,
            "isa": v["isa"],
            "methods": mt,
            "objtype": v.get("objectType") or name,
            "scripts": st,
        }
        if v.get("singleton"):
            entry["singleton"] = True
        if v.get("virtual"):
            entry["virtual"] = True

        t[name] = entry

    # Add unsupported types from other products
    products_list = read_yaml(WOWLESS_DIR / "data" / "products.yaml")
    for other_product in products_list:
        other_uiobjects = perproduct(other_product, "uiobjects") or {}
        for k in other_uiobjects:
            if k not in t:
                t[k] = {"unsupported": True}

    return f"_G.WowlessData.UIObjectApis = {lua_repr(t)}\n"


def main():
    output_dir = OUTPUT_DIR
    output_dir.mkdir(parents=True, exist_ok=True)

    product = PRODUCT

    files = {
        "WowlessData.toc": lambda: gen_toc(product),
        "product.lua": lambda: gen_product(product),
        "build.lua": lambda: gen_simple(product, "build", "Build"),
        "config.lua": lambda: gen_simple(product, "config", "Config"),
        "cvars.lua": lambda: gen_simple(product, "cvars", "CVars"),
        "events.lua": lambda: gen_events(product),
        "globalapis.lua": lambda: gen_globalapis(product),
        "globals.lua": lambda: gen_simple(product, "globals", "Globals"),
        "impltests.lua": lambda: gen_impltests(product),
        "luaobjects.lua": lambda: gen_luaobjects(product),
        "namespaceapis.lua": lambda: gen_namespaceapis(product),
        "uiobjectapis.lua": lambda: gen_uiobjectapis(product),
    }

    print(f"Reading data from {WOWLESS_DIR}", file=sys.stderr)
    for filename, gen_fn in files.items():
        out_path = output_dir / filename
        content = gen_fn()
        with open(out_path, "w") as f:
            f.write(content)
        size = len(content)
        print(f"  Wrote {filename} ({size:,} bytes)", file=sys.stderr)

    print(f"\nGenerated {len(files)} files in {output_dir}", file=sys.stderr)


if __name__ == "__main__":
    main()
