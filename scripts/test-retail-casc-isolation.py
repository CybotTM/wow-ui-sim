#!/usr/bin/env python3
"""Verify the retail Blizzard UI manifest using only the local _retail_ install."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


REPO_ROOT = Path(__file__).resolve().parent.parent
DEFAULT_WOW_ROOT = Path("/syncthing/World of Warcraft")
RETAIL_SYNC_EXCLUDED = {
    "Blizzard_CooldownBroadcaster/Blizzard_CooldownBroadcaster_Bootstrap.lua",
}
RETAIL_CONTENT_REQUIREMENTS = {
    # 12.0.7 split RuneforgeUtil.lua/.xml into separate TOC entries; the XML no
    # longer <Script>-includes the Lua. Guard on a frame that is in the file.
    "Blizzard_FrameXMLUtil/RuneforgeUtil.xml": 'name="RuneforgeCovenantSigilTemplate"',
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Mask non-retail WoW flavor directories with Bubblewrap and sync the "
            "retail Blizzard UI manifest into a fresh cache."
        )
    )
    parser.add_argument(
        "--wow-root",
        type=Path,
        default=Path(os.environ.get("WOW_INSTALL_PATH", DEFAULT_WOW_ROOT)),
        help="World of Warcraft installation root (default: WOW_INSTALL_PATH or %(default)s)",
    )
    parser.add_argument(
        "--cache-dir",
        type=Path,
        help="Use this empty cache directory instead of creating a temporary one",
    )
    parser.add_argument(
        "--skip-build",
        action="store_true",
        help="Use the existing target/debug/wow-cli binary",
    )
    parser.add_argument(
        "--keep-cache",
        action="store_true",
        help="Keep the temporary cache after a successful test (failures are always kept)",
    )
    return parser.parse_args()


def require_program(name: str) -> str:
    path = shutil.which(name)
    if path is None:
        raise SystemExit(f"required program not found: {name}")
    return path


def find_non_retail_flavors(wow_root: Path) -> list[Path]:
    retail = wow_root / "_retail_"
    if not retail.is_dir():
        raise SystemExit(f"retail install not found: {retail}")

    return sorted(
        path
        for path in wow_root.iterdir()
        if path.is_dir() and path.name.startswith("_") and path.name != "_retail_"
    )


def prepare_cache(requested: Path | None) -> tuple[Path, bool]:
    if requested is None:
        return Path(tempfile.mkdtemp(prefix="wow-ui-sim-retail-only-")), True

    cache = requested.resolve()
    cache.mkdir(parents=True, exist_ok=True)
    if any(cache.iterdir()):
        raise SystemExit(f"cache directory must be empty: {cache}")
    return cache, False


def build_wow_cli() -> None:
    subprocess.run(
        ["cargo", "build", "--bin", "wow-cli"],
        cwd=REPO_ROOT,
        check=True,
    )


def bwrap_prefix(bwrap: str, wow_root: Path, cache: Path, masked: list[Path]) -> list[str]:
    command = [
        bwrap,
        "--die-with-parent",
        "--ro-bind",
        "/",
        "/",
        "--dev",
        "/dev",
        "--proc",
        "/proc",
        "--bind",
        str(cache),
        str(cache),
    ]
    for path in masked:
        command.extend(("--tmpfs", str(path)))
    command.extend(
        (
            "--setenv",
            "WOW_INSTALL_PATH",
            str(wow_root),
            "--setenv",
            "XDG_CACHE_HOME",
            str(cache),
            "--setenv",
            "HOME",
            str(cache),
            "--",
        )
    )
    return command


def verify_masks(prefix: list[str], wow_root: Path, masked: list[Path]) -> None:
    probe = (
        "import json, pathlib, sys; "
        "root=pathlib.Path(sys.argv[1]); masked=json.loads(sys.argv[2]); "
        "assert any((root/'_retail_').iterdir()), '_retail_ is empty'; "
        "bad=[p for p in masked if any(pathlib.Path(p).iterdir())]; "
        "assert not bad, f'non-retail masks expose files: {bad}'"
    )
    subprocess.run(
        prefix
        + [
            sys.executable,
            "-c",
            probe,
            str(wow_root),
            json.dumps([str(path) for path in masked]),
        ],
        cwd=REPO_ROOT,
        check=True,
    )


def run_sync(prefix: list[str], cache: Path) -> tuple[int, Path]:
    log_path = cache / "retail-casc-isolation.log"
    wow_cli = REPO_ROOT / "target/debug/wow-cli"
    if not wow_cli.is_file():
        raise SystemExit(f"wow-cli binary not found: {wow_cli}; omit --skip-build")
    with log_path.open("w", encoding="utf-8") as log:
        result = subprocess.run(
            prefix + [str(wow_cli), "casc", "sync-blizzard-ui"],
            cwd=REPO_ROOT,
            stdout=log,
            stderr=subprocess.STDOUT,
            text=True,
            check=False,
        )
    return result.returncode, log_path


def cache_entry_problems(cache: Path) -> list[str]:
    manifest_path = REPO_ROOT / "data/blizzard-ui-files/retail.txt"
    cache_root = cache / "wow-ui-sim/blizzard-ui/retail/AddOns"
    manifest = [
        line.strip()
        for line in manifest_path.read_text(encoding="utf-8").splitlines()
        if line.strip() and line.strip() not in RETAIL_SYNC_EXCLUDED
    ]
    problems = [entry for entry in manifest if not (cache_root / entry).is_file()]
    for entry, required_text in RETAIL_CONTENT_REQUIREMENTS.items():
        path = cache_root / entry
        if path.is_file() and required_text not in path.read_text(
            encoding="utf-8", errors="replace"
        ):
            problems.append(entry)
    return problems


def print_failure(log_path: Path, problems: list[str]) -> None:
    print(f"Retail-only CASC sync failed; log: {log_path}", file=sys.stderr)
    print(f"Missing or unusable cache entries: {len(problems)}", file=sys.stderr)
    for entry in problems:
        print(entry, file=sys.stderr)

    lines = log_path.read_text(encoding="utf-8", errors="replace").splitlines()
    if lines:
        print("\nLog tail:", file=sys.stderr)
        for line in lines[-20:]:
            print(line, file=sys.stderr)


def main() -> int:
    args = parse_args()
    bwrap = require_program("bwrap")
    if not args.skip_build:
        require_program("cargo")
    wow_root = args.wow_root.resolve()
    masked = find_non_retail_flavors(wow_root)
    cache, temporary = prepare_cache(args.cache_dir)

    print(f"Retail install: {wow_root / '_retail_'}")
    print("Masked flavors:")
    for path in masked:
        print(f"  {path.name}")
    print(f"Isolated cache: {cache}")

    if not args.skip_build:
        build_wow_cli()

    prefix = bwrap_prefix(bwrap, wow_root, cache, masked)
    verify_masks(prefix, wow_root, masked)
    return_code, log_path = run_sync(prefix, cache)
    problems = cache_entry_problems(cache)

    if return_code != 0:
        print_failure(log_path, problems)
        return return_code

    if problems:
        print_failure(log_path, problems)
        return 1

    print("Retail-only CASC sync passed.")
    print(f"Log: {log_path}")
    if temporary and not args.keep_cache:
        shutil.rmtree(cache)
        print("Removed temporary cache.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
