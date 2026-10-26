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


def prepare_source(worktree, previous, attempted=None):
    identity, dirty = source_identity(worktree)
    content_changed = read_content_changes(worktree, identity, dirty, previous)
    changed = set(content_changed)
    last_attempt = attempted if attempted is not None else previous
    retry = bool(last_attempt and last_attempt.get("result", "success") != "success")
    if not previous or identity != previous.get("identity") or retry:
        if retry:
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
        outputs.update(os.path.realpath(worktree / word, strict=False)
                       for word in dep_info_words(target))
        # Keep symlink-sensitive /.. traversal until classification and existence checks.
        inputs.update(str(worktree / word) for word in dep_info_words(sources))
    if not outputs or not inputs:
        raise ValueError("no dependency rules")
    return {"outputs": sorted(outputs), "inputs": sorted(inputs)}


def read_dep_info_snapshot(slot, worktree):
    files, executables, identities = {}, [], {}
    for directory, children, names in os.walk(slot):
        children[:] = [name for name in children
                       if not name.startswith(".") and name not in {"incremental", "receipts"}]
        for name in names:
            path = Path(directory) / name
            if path.is_file() and (path.stat().st_mode & 0o111 or path.suffix == ".exe"):
                executables.append(str(path))
                stat = path.stat()
                identities.setdefault((stat.st_dev, stat.st_ino), []).append(str(path))
            if path.suffix != ".d":
                continue
            try:
                files[str(path)] = parse_dep_info(path.read_text(), worktree)
            except (OSError, UnicodeError, ValueError) as error:
                files[str(path)] = {"error": str(error)}
    aliases = {path: paths for paths in identities.values() for path in paths}
    return {"files": files, "executables": executables, "aliases": aliases}


def target_dep_inputs(record, snapshot, allow_missing=False):
    artifacts = {os.path.realpath(path, strict=False) for path in
                 [record.get("executable"), *record.get("filenames", [])] if path}
    # Cargo may report only the uplifted bin, while rustc writes deps/<name>-<hash>.d.
    # Snapshot hard-link identities bind that bin to the correct metadata hash.
    aliases = snapshot.get("aliases", {})
    artifacts.update(alias for path in list(artifacts) for alias in aliases.get(path, []))
    candidates = {str(Path(path).with_suffix(".d")) for path in artifacts}
    matches = {name: info for name, info in snapshot["files"].items()
               if name in candidates or artifacts.intersection(info.get("outputs", []))}
    rustc_matches = {name: info for name, info in matches.items()
                     if Path(name).parent.name == "deps"}
    if rustc_matches:
        matches = rustc_matches
    if not matches and not allow_missing:
        raise ValueError(f"missing dep-info for {artifact_path(record)}")
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
    roots = [Path(os.path.realpath(root, strict=False))
             for root in (mount, utility_mount, worktree)]
    relative = set()
    for name in inputs:
        path = Path(os.path.realpath(worktree / name, strict=False))
        for root in roots:
            if path.is_relative_to(root):
                relative.add(str(path.relative_to(root)))
                break
    return relative


def generated_input_paths(current, inputs):
    slot = Path(os.path.realpath(current["slot"], strict=False))
    generated = set()
    for name in inputs:
        path = Path(os.path.realpath(Path(current["worktree"]) / name, strict=False))
        if path.is_relative_to(slot) and not repo_relative_inputs(current, [str(path)]):
            generated.add(str(path))
    return generated


def artifact_path(record):
    if record.get("executable"):
        return record["executable"]
    filenames = record.get("filenames", [])
    return filenames[0] if filenames else record.get("target", {}).get("name", "unknown")


def is_repo_artifact(current, record):
    if record.get("package_id", "").startswith(("registry+", "git+")):
        return False
    paths = [path for path in [record.get("manifest_path"),
                              record.get("target", {}).get("src_path")] if path]
    # Records without ownership metadata cannot prove that they are external.
    return not paths or bool(repo_relative_inputs(current, paths))


def read_record_inputs(current, record, before, after):
    compiler_inputs = target_dep_inputs(record, after)
    inputs = repo_relative_inputs(current, compiler_inputs)
    if not inputs:
        raise ValueError(f"zero repo-relative inputs for {artifact_path(record)}")
    previous = target_dep_inputs(record, before, allow_missing=record.get("fresh") is False)
    inputs.update(repo_relative_inputs(current, previous))
    inputs.update(generated_input_paths(current, previous | compiler_inputs))
    inputs.update(repo_relative_inputs(current, package_build_inputs(current, record)))
    return sorted(inputs)


def read_target_inputs(current, before, after):
    targets, errors = {}, []
    for record in current["reported_artifacts"]:
        if not is_repo_artifact(current, record):
            continue
        try:
            targets[artifact_path(record)] = read_record_inputs(current, record, before, after)
        except ValueError as error:
            errors.append(str(error))
    return targets, errors


def read_input_hashes(worktree, names):
    hashes = {}
    for name in sorted(names):
        path = worktree / name
        try:
            with path.open("rb") as stream:
                hashes[name] = hashlib.file_digest(stream, "sha256").hexdigest()
        except FileNotFoundError:
            hashes[name] = None
    return hashes


def snapshot_source_hashes(current, snapshot):
    worktree = Path(current["worktree"])
    names = {os.fsdecode(p) for p in git(
        worktree, "ls-files", "--cached", "--others", "--exclude-standard", "-z").split(b"\0") if p}
    paths = {name for info in snapshot["files"].values() for name in info.get("inputs", [])}
    roots = {root for roots in current["package_roots"].values() for root in roots}
    roots.add(str(worktree))
    paths.update(str(Path(root) / name) for root in roots
                 for name in ("build.rs", "Cargo.toml", "Cargo.lock"))
    names.update(repo_relative_inputs(current, paths))
    return read_input_hashes(worktree, names)


def certify_record(current, record, previous, source_before, artifacts, snapshot):
    path = artifact_path(record)
    compiler_inputs = target_dep_inputs(record, snapshot)
    required = repo_relative_inputs(current, compiler_inputs)
    if not required:
        raise ValueError(f"missing compiler inputs for {path}")
    generated = generated_input_paths(current, compiler_inputs)
    required.update(generated)
    for name in compiler_inputs:
        if not (repo_relative_inputs(current, [name])
                or generated_input_paths(current, [name])):
            continue
        if not os.path.exists(Path(current["worktree"]) / name):
            raise ValueError(f"missing compiler input content for {path}: {name}")
    inputs = required | repo_relative_inputs(current, package_build_inputs(current, record))
    hashes = read_input_hashes(Path(current["worktree"]), inputs)
    if any(hashes.get(name) is None for name in required):
        raise ValueError(f"missing compiler input content for {path}")
    # Slot-local generated files may be produced by this build; source inputs must predate it.
    if any(name not in source_before or source_before[name] != digest
           for name, digest in hashes.items() if name not in generated):
        raise ValueError(f"compiler inputs changed during build for {path}")
    outputs = set(record.get("filenames", []))
    if record.get("executable"):
        outputs.add(record["executable"])
    if not outputs:
        raise ValueError(f"missing compiler outputs for {path}")
    certificates = {}
    for output in sorted(outputs):
        name = os.path.relpath(output, current["slot"])
        if name not in artifacts:
            raise ValueError(f"missing output hash for {output}")
        certificate = {"inputs": hashes, "sha256": artifacts[name]}
        if record.get("fresh") is not False and previous.get(name) != certificate:
            raise ValueError(f"uncertified fresh artifact {output}; rebuild required")
        certificates[name] = certificate
    return certificates


def certify_artifacts(current, previous, source_before, artifacts, snapshot):
    certificates = dict((previous or {}).get("artifact_certificates", {}))
    errors = []
    for record in current["reported_artifacts"]:
        if not is_repo_artifact(current, record):
            continue
        try:
            certificates.update(certify_record(
                current, record, certificates, source_before, artifacts, snapshot))
        except (OSError, ValueError) as error:
            errors.append(str(error))
    return certificates, errors


def find_stale_artifacts(current, previous_artifacts, artifacts):
    stale = []
    changed = set(current["changed_paths"])
    content_changed = set(current["content_changed_paths"])
    for record in current["reported_artifacts"]:
        path = artifact_path(record)
        inputs = current["target_inputs"].get(path, [])
        if not changed.intersection(inputs):
            continue
        name = os.path.relpath(path, current["slot"])
        if record.get("fresh") is not False:
            stale.append(name)
            continue
        if not record.get("executable"):
            continue
        changed_hash = name in artifacts and previous_artifacts.get(name) != artifacts[name]
        needs_changed_hash = bool(content_changed.intersection(inputs))
        if name not in artifacts or (needs_changed_hash and not changed_hash):
            stale.append(name)
    return sorted(set(stale))


def unread_package_paths(current):
    inputs = {name for names in current["target_inputs"].values() for name in names}
    roots = {root for paths in current["package_roots"].values() for root in paths}
    roots = repo_relative_inputs(current, roots)
    return sorted(name for name in set(current["content_changed_paths"]) - inputs
                  if any(Path(name).is_relative_to(root) for root in roots))


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
    current = dict(current, artifacts=artifacts, returncode=returncode, result=result,
                   unread_package_paths=unread_package_paths(current))
    write_json(receipt, current)
    write_json(slot / ".source-identity.json", current)
    if result == "success":
        write_json(slot / ".verified-source-identity.json", current)
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
    # records select artifacts for the freshness decision.

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
    paths.update(Path(name) for record in records
                 for name in [record.get("executable"), *record.get("filenames", [])] if name)
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
    verified = slot / ".verified-source-identity.json"
    previous = json.loads(verified.read_text()) if verified.exists() else None
    attempted = json.loads(state.read_text()) if state.exists() else None
    worktree = Path(request["worktree"])
    current = prepare_source(worktree, previous, attempted)
    current.update(unit=request["unit"], cargo=request["cargo"], worktree=str(worktree),
                   slot=str(slot), package_roots=read_package_roots(worktree))
    # Attempts can replace Cargo fingerprints, but cannot certify source content.
    # Only finish_build advances the separate verified baseline after acceptance.
    write_json(state, dict(current, artifacts={}, result="building"))
    receipt = Path(request["receipt"])
    write_json(receipt, dict(current, artifacts={}, result="building"))
    before = read_dep_info_snapshot(slot, worktree)
    source_before = snapshot_source_hashes(current, before)
    started_ns = time.time_ns()
    returncode, records = run_build_command(request["cargo"])
    current["reported_artifacts"] = records
    after = read_dep_info_snapshot(slot, worktree)
    current["target_inputs"], current["dep_info_errors"] = read_target_inputs(
        current, before, after)
    artifacts = collect_artifacts(slot, started_ns, previous, records)
    current["artifact_certificates"], certificate_errors = certify_artifacts(
        current, previous, source_before, artifacts, after)
    current["dep_info_errors"].extend(certificate_errors)
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
