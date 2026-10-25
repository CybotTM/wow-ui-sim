#!/usr/bin/env python3
"""Launch an admitted Cargo build through a stable per-slot source mount."""
import argparse
import hashlib
import json
import os
import re
from pathlib import Path
import subprocess
import sys
import time


TARGET_ROOT = Path("/home/osso/.cache/wow-ui-sim-targets")
SOURCE_ROOT = Path("/home/osso/.cache/wow-ui-sim-srcmount")
CAPACITY = "/home/osso/.worktrees/logs/capacity.sh"
LIBRARY_SUFFIXES = {".rlib", ".rmeta", ".so", ".a", ".dll", ".dylib"}


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
        "--setenv=CARGO_INCREMENTAL=1", "--setenv=RUSTC_WRAPPER=",
        f"--setenv=CARGO_TARGET_DIR={target}",
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


def full_source_inputs(worktree):
    tracked = {os.fsdecode(p) for p in git(worktree, "ls-files", "-z").split(b"\0") if p}
    result = subprocess.run(
        ["cargo", "metadata", "--no-deps", "--format-version", "1", "--offline"],
        cwd=worktree, capture_output=True, check=False)
    if result.returncode:
        return tracked
    metadata = json.loads(result.stdout)
    roots = set()
    for package in metadata["packages"]:
        roots.add(Path(package["manifest_path"]).parent)
        roots.update(Path(dep["path"]) for dep in package["dependencies"] if dep.get("path"))
    # The root package covers build.rs's build/, installer/, native/intl/ and
    # Cargo.lock, plus test-harness/build.rs's tests reads, without a whitelist.
    inputs = {"build.rs", "Cargo.toml", "Cargo.lock"} & tracked
    inputs.update(name for name in tracked
                  if any((worktree / name).is_relative_to(root) for root in roots))
    return inputs


def read_content_changes(worktree, identity, dirty, previous):
    if previous and identity == previous.get("identity"):
        return set()
    try:
        if not previous:
            raise ValueError("no previous source")
        paths = git(worktree, "diff", "--name-only", "--no-renames", "-z",
                    previous["identity"]["head"], identity["head"], "--")
        changed = {os.fsdecode(p) for p in paths.split(b"\0") if p}
        changed.update(previous["dirty_paths"])
    except (subprocess.CalledProcessError, KeyError, ValueError):
        changed = full_source_inputs(worktree)
    return changed.union(dirty)


def prepare_source(worktree, previous):
    identity, dirty = source_identity(worktree)
    content_changed = read_content_changes(worktree, identity, dirty, previous)
    changed = set(content_changed)
    if (not previous or identity != previous.get("identity")
            or previous.get("result", "success") != "success"):
        if previous and previous.get("result", "success") != "success":
            changed.update(full_source_inputs(worktree))
        changed.update(dirty)
        now = time.time_ns()
        for name in sorted(changed):
            path = worktree / name
            if path.is_file() and not path.is_symlink():
                os.utime(path, ns=(path.stat().st_atime_ns, now))
    return {"identity": identity, "dirty_paths": dirty,
            "changed_paths": sorted(changed),
            "content_changed_paths": sorted(content_changed)}


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(".tmp")
    temporary.write_text(json.dumps(value, indent=2) + "\n")
    temporary.replace(path)


def read_package_roots(worktree):
    result = subprocess.run(
        ["cargo", "metadata", "--no-deps", "--format-version", "1", "--offline"],
        cwd=worktree, capture_output=True, check=False)
    if result.returncode:
        # Match the existing all-tracked-input policy when metadata is unavailable.
        return {None: [str(worktree)]}
    packages = json.loads(result.stdout)["packages"]
    return {package["id"]: [str(Path(package["manifest_path"]).parent),
                           *(dep["path"] for dep in package["dependencies"]
                             if dep.get("path"))]
            for package in packages}


def package_inputs_changed(current, roots):
    worktree = Path(current["worktree"])
    return any((worktree / name).is_relative_to(root)
               for name in current["changed_paths"] for root in roots)


def dep_info_words(value):
    words = re.findall(r"(?:\\.|[^\s\\])+", value)
    return [re.sub(r"\\([ #\\:])", r"\1", word).replace("$$", "$")
            for word in words]


def parse_dep_info(content, worktree):
    outputs, inputs = set(), set()
    for line in content.replace("\\\n", " ").splitlines():
        target, separator, sources = line.partition(": ")
        if not separator or not sources.strip():
            continue
        outputs.update(os.path.abspath(worktree / word) for word in dep_info_words(target))
        inputs.update(os.path.abspath(worktree / word) for word in dep_info_words(sources))
    if not outputs or not inputs:
        raise ValueError("no dependency rules")
    return {"outputs": sorted(outputs), "inputs": sorted(inputs)}


def read_dep_info_snapshot(slot, worktree):
    files, executables = {}, []
    for directory, children, names in os.walk(slot):
        children[:] = [name for name in children
                       if not name.startswith(".") and name not in {"incremental", "receipts"}]
        for name in names:
            path = Path(directory) / name
            if path.is_file() and (path.stat().st_mode & 0o111 or path.suffix == ".exe"):
                executables.append(str(path))
            if path.suffix != ".d":
                continue
            try:
                files[str(path)] = parse_dep_info(path.read_text(), worktree)
            except (OSError, UnicodeError, ValueError) as error:
                files[str(path)] = {"error": str(error)}
    return {"files": files, "executables": executables}


def target_dep_inputs(record, snapshot, allow_missing=False):
    artifacts = {os.path.abspath(path) for path in
                 [record["executable"], *record.get("filenames", [])]}
    candidates = {str(Path(path).with_suffix(".d")) for path in artifacts}
    matches = {name: info for name, info in snapshot["files"].items()
               if name in candidates or artifacts.intersection(info.get("outputs", []))}
    if not matches and not allow_missing:
        raise ValueError(f"missing dep-info for {record['executable']}")
    inputs = set()
    for name, info in matches.items():
        if "error" in info:
            raise ValueError(f"unreadable dep-info {name}: {info['error']}")
        inputs.update(info["inputs"])
    return inputs


def package_build_inputs(current, record):
    manifest = record.get("manifest_path")
    roots = current["package_roots"].get(record.get("package_id"), [])
    package = Path(current["worktree"])
    if manifest:
        package = Path(manifest).parent
    elif roots:
        package = Path(roots[0])
    inputs = {str(package / name) for name in ("build.rs", "Cargo.toml", "Cargo.lock")}
    inputs.add(str(Path(current["worktree"]) / "Cargo.lock"))
    return inputs


def repo_relative_inputs(current, inputs):
    worktree = Path(current["worktree"])
    mount = SOURCE_ROOT / Path(current["slot"]).name
    utility_mount = SOURCE_ROOT / f"ct-bins-{Path(current['slot']).name}"
    roots = [mount, utility_mount, worktree]
    relative = set()
    for name in inputs:
        path = Path(os.path.abspath(worktree / name))
        for root in roots:
            if path.is_relative_to(root):
                relative.add(str(path.relative_to(root)))
                break
    return relative


def read_target_inputs(current, before, after):
    targets, errors = {}, []
    for record in current["reported_artifacts"]:
        executable = record.get("executable")
        if not executable:
            continue
        try:
            inputs = target_dep_inputs(record, after)
            if executable in before["executables"]:
                inputs.update(target_dep_inputs(
                    record, before, allow_missing=record.get("fresh") is False))
            inputs = repo_relative_inputs(current, inputs)
            if not inputs:
                raise ValueError(f"zero repo-relative inputs for {executable}")
            inputs.update(repo_relative_inputs(current, package_build_inputs(current, record)))
            targets[executable] = sorted(inputs)
        except ValueError as error:
            if current["content_changed_paths"]:
                errors.append(str(error))
    mapped = set().union(*targets.values())
    changed = current["content_changed_paths"]
    if targets and changed and not mapped.intersection(changed):
        errors.append("no reported target maps changed inputs")
    return targets, errors


def find_stale_artifacts(current, previous_artifacts, artifacts):
    stale = []
    changed = set(current["changed_paths"])
    content_changed = set(current["content_changed_paths"])
    for record in current["reported_artifacts"]:
        executable = record.get("executable")
        inputs = current["target_inputs"].get(executable, [])
        if not executable or not changed.intersection(inputs):
            continue
        name = os.path.relpath(executable, current["slot"])
        changed_hash = name in artifacts and previous_artifacts.get(name) != artifacts[name]
        needs_changed_hash = bool(content_changed.intersection(inputs))
        if (record.get("fresh") is not False or name not in artifacts
                or (needs_changed_hash and not changed_hash)):
            stale.append(name)
    return sorted(set(stale))


def finish_build(slot, receipt, current, previous, artifacts, returncode):
    previous_artifacts = (previous or {}).get("artifacts", {})
    stale_paths = find_stale_artifacts(
        current, previous_artifacts, artifacts) if returncode == 0 else []
    roots = [root for roots in current["package_roots"].values() for root in roots]
    missing_evidence = (returncode == 0 and not current["reported_artifacts"]
                        and package_inputs_changed(current, roots))
    dep_info_errors = current.get("dep_info_errors", []) if returncode == 0 else []
    stale = bool(stale_paths) or missing_evidence or bool(dep_info_errors)
    result = "success" if returncode == 0 else "failed"
    if stale:
        result = "stale-artifacts"
    current = dict(current, artifacts=artifacts, returncode=returncode, result=result)
    write_json(receipt, current)
    write_json(slot / ".source-identity.json", current)
    if stale:
        reason = ("no Cargo compiler-artifact records visible for changed package inputs"
                  if missing_evidence else f"missing fresh=false or changed SHA256: {stale_paths}")
        if dep_info_errors:
            reason = "; ".join(dep_info_errors)
        raise ValueError(f"stale artifacts: {reason}; see {receipt}")


def hash_artifacts(slot, paths):
    artifacts = {}
    for path in sorted(paths):
        if path.is_file() and not path.is_symlink():
            with path.open("rb") as stream:
                artifacts[os.path.relpath(path, slot)] = hashlib.file_digest(
                    stream, "sha256").hexdigest()
    return artifacts


def collect_artifacts(slot, started_ns, previous, records):
    # Retain discovery/hash evidence for capturing wrappers. Only current Cargo
    # records select executables for the freshness decision.

    paths = {slot / name for name in (previous or {}).get("artifacts", {})}
    for directory, children, files in os.walk(slot):
        children[:] = [name for name in children
                       if not name.startswith(".") and name not in {"incremental", "receipts"}]
        for name in files:
            path = Path(directory) / name
            if path.is_symlink() or not path.is_file():
                continue
            stat = path.stat()
            artifact = (stat.st_mode & 0o111 or path.suffix in LIBRARY_SUFFIXES
                        or path.suffix == ".exe")
            if artifact and max(stat.st_mtime_ns, stat.st_ctime_ns) >= started_ns:
                paths.add(path)
    paths.update(Path(record["executable"]) for record in records if record.get("executable"))
    return hash_artifacts(slot, paths)


def run_build_command(command):
    records = []
    with subprocess.Popen(command, stdout=subprocess.PIPE) as child:
        for line in child.stdout:
            sys.stdout.buffer.write(line)
            sys.stdout.buffer.flush()
            try:
                record = json.loads(line)
            except (ValueError, UnicodeDecodeError):
                continue
            if isinstance(record, dict) and record.get("reason") == "compiler-artifact":
                records.append(record)
        return child.wait(), records


def execute_build(request_path):
    request = json.loads(request_path.read_text())
    slot = Path(request["slot"])
    check_owner(slot, request["owner"])
    state = slot / ".source-identity.json"
    previous = json.loads(state.read_text()) if state.exists() else None
    worktree = Path(request["worktree"])
    current = prepare_source(worktree, previous)
    current.update(unit=request["unit"], cargo=request["cargo"], worktree=str(worktree),
                   slot=str(slot), package_roots=read_package_roots(worktree))
    # An interrupted/failed build may already have replaced fingerprints. Record
    # its input identity before Cargo starts, not only after successful builds.
    write_json(state, dict(current, artifacts={}, result="building"))
    receipt = Path(request["receipt"])
    write_json(receipt, dict(current, artifacts={}, result="building"))
    before = read_dep_info_snapshot(slot, worktree)
    started_ns = time.time_ns()
    returncode, records = run_build_command(request["cargo"])
    current["reported_artifacts"] = records
    after = read_dep_info_snapshot(slot, worktree)
    current["target_inputs"], current["dep_info_errors"] = read_target_inputs(
        current, before, after)
    artifacts = collect_artifacts(slot, started_ns, previous, records)
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
