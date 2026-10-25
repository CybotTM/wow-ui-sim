#!/usr/bin/env python3
"""Launch an admitted Cargo build through a stable per-slot source mount."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time


TARGET_ROOT = Path("/home/osso/.cache/wow-ui-sim-targets")
SOURCE_ROOT = Path("/home/osso/.cache/wow-ui-sim-srcmount")
CAPACITY = "/home/osso/.worktrees/logs/capacity.sh"


def check_owner(slot, owner):
    if (slot / ".owner").read_text() != owner:
        raise ValueError(f"slot owner mismatch: {slot / '.owner'}")


def check_capacity(capacity_id):
    env = dict(os.environ, CAPACITY_ID=capacity_id)
    env.pop("CAPACITY_PEEK", None)
    result = subprocess.run([CAPACITY], env=env, check=True,
                            capture_output=True, text=True)
    verdict = result.stdout.strip().rsplit("|", 1)[-1].strip()
    if verdict != "ADD" and not verdict.startswith("ADD:"):
        raise ValueError(f"capacity refused: {result.stdout.strip()}")


def build_argv(slot, worktree, unit, cargo):
    target = TARGET_ROOT / f"slot-{slot}"
    # Incremental reuse requires this fixed BindPaths source path per target slot.
    mount = SOURCE_ROOT / f"slot-{slot}"
    return [
        "/usr/bin/systemd-run", "--user", "--no-block", "--remain-after-exit",
        f"--unit={unit}", "--slice=agents.slice",
        "--property=MemoryMax=9G", "--property=MemorySwapMax=0",
        "--property=CPUAccounting=yes",
        "--setenv=CARGO_INCREMENTAL=1", f"--setenv=CARGO_TARGET_DIR={target}",
        f"--property=BindPaths={worktree}:{mount}",
        f"--working-directory={mount}", "--", *cargo,
    ]


def git(worktree, *args):
    return subprocess.check_output(["git", "-C", str(worktree), *args])


def source_identity(worktree):
    head = git(worktree, "rev-parse", "HEAD").decode().strip()
    diff = git(worktree, "diff", "--binary", "HEAD", "--")
    dirty = set(os.fsdecode(p) for p in git(
        worktree, "diff", "--name-only", "-z", "HEAD", "--").split(b"\0") if p)
    untracked = git(worktree, "ls-files", "--others", "--exclude-standard", "-z")
    digest = hashlib.sha256(diff)
    for path in sorted(p for p in untracked.split(b"\0") if p):
        name = os.fsdecode(path)
        dirty.add(name)
        file = worktree / name
        content = os.fsencode(os.readlink(file)) if file.is_symlink() else file.read_bytes()
        digest.update(path + b"\0" + hashlib.sha256(content).digest())
    return {"head": head, "dirty_sha256": digest.hexdigest()}, sorted(dirty)


def crate_input(path):
    return (path.split("/", 1)[0] in {"src", "crates", "tests", "data", "build", "installer"}
            or path in {"build.rs", "Cargo.toml", "Cargo.lock"})


def prepare_source(worktree, previous):
    identity, dirty = source_identity(worktree)
    changed = set()
    if (not previous or identity != previous.get("identity")
            or previous.get("result", "success") != "success"):
        try:
            if not previous or previous.get("result", "success") != "success":
                raise ValueError("no previous source")
            changed.update(os.fsdecode(p) for p in git(
                worktree, "diff", "--name-only", "--no-renames", "-z",
                previous["identity"]["head"], identity["head"], "--").split(b"\0") if p)
            changed.update(previous["dirty_paths"])
        except (subprocess.CalledProcessError, KeyError, ValueError):
            changed.update(os.fsdecode(p) for p in git(
                worktree, "ls-files", "-z").split(b"\0") if p and crate_input(os.fsdecode(p)))
        changed.update(dirty)
        now = time.time_ns()
        for name in sorted(changed):
            path = worktree / name
            if path.is_file() and not path.is_symlink():
                os.utime(path, ns=(path.stat().st_atime_ns, now))
    return {"identity": identity, "dirty_paths": dirty,
            "changed_paths": sorted(changed)}


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(".tmp")
    temporary.write_text(json.dumps(value, indent=2) + "\n")
    temporary.replace(path)


def finish_build(slot, receipt, current, previous, artifacts, returncode):
    rust_changed = any(p.endswith(".rs") for p in current["changed_paths"])
    old_hashes = set((previous or {}).get("artifacts", {}).values())
    stale = bool(returncode == 0 and rust_changed and artifacts and old_hashes
                 and set(artifacts.values()).issubset(old_hashes))
    current = dict(current, artifacts=artifacts, returncode=returncode,
                   result="stale-artifacts" if stale else ("success" if returncode == 0 else "failed"))
    write_json(receipt, current)
    write_json(slot / ".source-identity.json", current)
    if stale:
        raise ValueError(f"stale artifacts: Rust source changed but all binary SHA256 values "
                         f"match the previous build; see {receipt}")


def execute_build(request_path):
    request = json.loads(request_path.read_text())
    slot = Path(request["slot"])
    check_owner(slot, request["owner"])
    state = slot / ".source-identity.json"
    previous = json.loads(state.read_text()) if state.exists() else None
    worktree = Path(request["worktree"])
    current = prepare_source(worktree, previous)
    current.update(unit=request["unit"], cargo=request["cargo"], worktree=str(worktree))
    # An interrupted/failed build may already have replaced fingerprints. Record
    # its input identity before Cargo starts, not only after successful builds.
    write_json(state, dict(current, artifacts={}, result="building"))
    receipt = Path(request["receipt"])
    write_json(receipt, dict(current, artifacts={}, result="building"))
    cargo = request["cargo"]
    formats = [arg.split("=", 1)[1] for arg in cargo if arg.startswith("--message-format=")]
    if "--message-format" in cargo:
        formats.append(cargo[cargo.index("--message-format") + 1])
    if formats and formats not in [["json"], ["json-render-diagnostics"]]:
        raise ValueError("slot-build requires JSON Cargo messages for artifact receipts")
    if not formats:
        cargo = list(cargo)
        position = cargo.index("--") if "--" in cargo else len(cargo)
        cargo.insert(position, "--message-format=json-render-diagnostics")
    artifacts = {}
    with subprocess.Popen(cargo,
                          stdout=subprocess.PIPE, text=True) as process:
        for line in process.stdout:
            print(line, end="", flush=True)
            try:
                message = json.loads(line)
            except json.JSONDecodeError:
                continue
            if not isinstance(message, dict):
                continue
            binary = message.get("executable")
            if message.get("reason") == "compiler-artifact" and binary:
                path = Path(binary)
                with path.open("rb") as stream:
                    artifacts[str(path.relative_to(slot))] = hashlib.file_digest(
                        stream, "sha256").hexdigest()
        returncode = process.wait()
    finish_build(slot, receipt, current, previous, artifacts, returncode)
    return returncode


def print_status(unit):
    properties = ["SubState", "Result", "CPUUsageNSec",
                  "ExecMainStartTimestampMonotonic", "ExecMainExitTimestampMonotonic"]
    argv = ["/usr/bin/systemctl", "--user", "show", unit]
    for name in properties:
        argv.extend(["-p", name])
    result = subprocess.run(argv, check=True, capture_output=True, text=True)
    values = dict(line.split("=", 1) for line in result.stdout.splitlines() if "=" in line)
    for name in properties[:3]:
        print(f"{name}={values[name]}")
    start = int(values[properties[3]])
    end = int(values[properties[4]]) or time.monotonic_ns() // 1000
    wall = f"{(end - start) / 1e6:.6f}" if start else "unavailable"
    print(f"WallTimeSec={wall}")


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--execute", type=Path, help=argparse.SUPPRESS)
    parser.add_argument("--status", metavar="UNIT")
    parser.add_argument("--slot", type=int, choices=range(4))
    parser.add_argument("--worktree", type=Path)
    parser.add_argument("--unit")
    parser.add_argument("--owner")
    parser.add_argument("--capacity-id")
    parser.add_argument("cargo", nargs=argparse.REMAINDER)
    args = parser.parse_args(argv)
    if args.execute:
        return execute_build(args.execute)
    if args.status:
        if any([args.slot is not None, args.worktree, args.unit,
                args.owner, args.capacity_id, args.cargo]):
            parser.error("--status cannot be combined with launch arguments")
        print_status(args.status)
        return
    if args.slot is None or not all([args.worktree, args.unit, args.owner, args.capacity_id]):
        parser.error("launch requires --slot, --worktree, --unit, --owner and --capacity-id")
    cargo = args.cargo[1:] if args.cargo[:1] == ["--"] else args.cargo
    if not cargo:
        parser.error("provide a Cargo command after --")
    if not args.worktree.is_absolute() or not args.worktree.is_dir():
        parser.error("--worktree must be an absolute directory path")
    worktree = args.worktree.resolve()
    target = TARGET_ROOT / f"slot-{args.slot}"
    check_owner(target, args.owner)
    check_capacity(args.capacity_id)
    check_owner(target, args.owner)
    (SOURCE_ROOT / f"slot-{args.slot}").mkdir(parents=True, exist_ok=True)
    if Path(args.unit).name != args.unit or args.unit in {".", ".."}:
        parser.error("--unit must be a unit name, not a path")
    request = target / "receipts" / f"{args.unit}.request.json"
    write_json(request, {"slot": str(target), "worktree": str(worktree),
                         "owner": args.owner, "unit": args.unit, "cargo": cargo,
                         "receipt": str(request.with_name(f"{args.unit}.json"))})
    worker = [sys.executable, str(Path(__file__).resolve()), "--execute", str(request)]
    subprocess.run(build_argv(args.slot, worktree, args.unit, worker), check=True)
    print(args.unit)


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        raise SystemExit(str(error)) from error
