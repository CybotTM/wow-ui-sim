#!/usr/bin/env python3
"""Run the full wow-ui-sim test suite for one commit, in the background, and record results.

Usage:
  full_suite.py run [REF] --target-dir SLOT [--build-jobs N] [--profile PROFILE]
                             Test REF (default origin/master) now; blocks until done.
  full_suite.py submit [REF] --target-dir SLOT [--build-jobs N] [--profile PROFILE]
                             Same, detached via systemd-run --user; prints the unit name.
  full_suite.py status [REF]   Print per-profile results for REF (or the latest run's SHA).

Build jobs default to 1, independent of FULL_SUITE_JOBS (nextest workers, default 16).
SLOT must resolve to one of ~/.cache/wow-ui-sim-targets/slot-{0,1,2,3}.
Runs serialize on a lock. Each run uses a dedicated checkout and an explicitly selected
approved target slot so agents' working copies are never touched. Results:
RESULTS/<sha>-<profile>.json plus <sha>-<profile>.log; master runs also update
RESULTS/master-latest-<profile>.json for same-profile NEW failure comparisons.
Retail (default) retains Cargo default features; other profiles select explicit features.
Integration/lib run for every profile; prefork runs only for Retail/Mists.
"""

import argparse
import fcntl
import json
import os
import re
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path.home() / "Projects/wow"
REPO = ROOT / "wow-ui-sim"
CHECKOUT = ROOT / "full-suite-checkout"
RESULTS = ROOT / "full-suite-results"
LOCK = RESULTS / ".lock"
JOBS = os.environ.get("FULL_SUITE_JOBS", "16")
PROFILES = ("retail", "ptr", "wrath", "mists", "wowforever", "era", "anniversary")
FAILED = re.compile(
    r"^\s*(?:FAIL|SIGSEGV|SIGABRT|TIMEOUT) \[.*?\] (?:\(\s*\d+/\d+\) )?\S+ (\S+)", re.M
)
PREFORK_FAILED = re.compile(r"^test (\S+) \.\.\. FAILED", re.M)


def git(*args, cwd=REPO):
    return subprocess.run(
        ["git", *args], cwd=cwd, check=True, capture_output=True, text=True
    ).stdout.strip()


def resolve(ref):
    git("fetch", "-q", "origin")
    return git("rev-parse", f"{ref}^{{commit}}")


def prepare_checkout(sha):
    if not (CHECKOUT / ".git").exists():
        subprocess.run(
            ["git", "clone", "-q", "--shared", str(REPO), str(CHECKOUT)], check=True
        )
    git("fetch", "-q", str(REPO), sha, cwd=CHECKOUT)
    git("checkout", "-q", "--force", sha, cwd=CHECKOUT)
    git("clean", "-q", "-fdx", "-e", "target", cwd=CHECKOUT)


def run_step(name, cmd, log, env):
    cmd = [*cmd, "--offline", "--locked"]
    start = time.monotonic()
    proc = subprocess.run(cmd, cwd=CHECKOUT, env=env, capture_output=True, text=True)
    output = proc.stdout + proc.stderr
    log.write(f"===== {name}: {' '.join(cmd)} (exit {proc.returncode})\n{output}\n")
    return {
        "exit": proc.returncode,
        "seconds": round(time.monotonic() - start, 1),
        "command": cmd,
        "environment": {
            "WOW_SIM_TIMEOUT_FIXTURE_CAPACITY": env.get("WOW_SIM_TIMEOUT_FIXTURE_CAPACITY")
        },
    }, output


def summarize(output, pattern):
    return sorted(set(pattern.findall(output)))


def validate_target_dir(value):
    target = Path(value)
    if not target.is_absolute():
        raise ValueError("target dir must be absolute")
    target = target.resolve()
    forbidden = {(REPO / "target").resolve(), (CHECKOUT / "target").resolve()}
    slots = {Path.home() / f".cache/wow-ui-sim-targets/slot-{i}" for i in range(4)}
    if target in forbidden or target not in slots:
        raise ValueError(
            "target dir must resolve to an approved cache slot (0..3), not a checkout target"
        )
    return target


def positive_build_jobs(value):
    jobs = int(value)
    if jobs < 1:
        raise argparse.ArgumentTypeError("build jobs must be positive")
    return jobs


def suite_steps(build_jobs, profile):
    features = (
        []
        if profile == "retail"
        else ["--no-default-features", "--features", f"sound,gui,casc,client-{profile}"]
    )
    return [
        (
            "integration",
            [
                "cargo",
                "nextest",
                "run",
                f"-j{build_jobs}",
                "--test",
                "integration",
                "--no-fail-fast",
                "--test-threads",
                JOBS,
                *features,
            ],
            FAILED,
        ),
        (
            "prefork",
            ["cargo", "test", f"-j{build_jobs}", "--test", "prefork_full_ui", *features]
            if profile in ("retail", "mists")
            else None,
            PREFORK_FAILED,
        ),
        (
            "lib",
            [
                "cargo",
                "nextest",
                "run",
                f"-j{build_jobs}",
                "--lib",
                "--no-fail-fast",
                "--test-threads",
                JOBS,
                *features,
            ],
            FAILED,
        ),
    ]


def run(ref, build_jobs, target_dir, profile="retail"):
    target_dir = validate_target_dir(target_dir)
    RESULTS.mkdir(parents=True, exist_ok=True)
    with open(LOCK, "w") as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        sha = resolve(ref)
        prepare_checkout(sha)
        env = dict(
            os.environ,
            CARGO_BUILD_JOBS=str(build_jobs),
            CARGO_TARGET_DIR=str(target_dir),
            CARGO_TERM_COLOR="never",
        )
        result = {
            "ref": ref,
            "sha": sha,
            "profile": profile,
            "started": time.strftime("%Y-%m-%dT%H:%M:%S%z"),
            "steps": {},
            "failures": {},
        }
        with open(RESULTS / f"{sha}-{profile}.log", "w") as log:
            for name, cmd, pattern in suite_steps(build_jobs, profile):
                if cmd is None:
                    reason = "prefork_full_ui supports only Retail/Mists"
                    result["steps"][name] = {
                        "status": "not_applicable",
                        "reason": reason,
                    }
                    log.write(f"===== {name}: N/A ({reason})\n")
                    continue
                result["steps"][name], output = run_step(name, cmd, log, env)
                result["failures"][name] = summarize(output, pattern)
        result["finished"] = time.strftime("%Y-%m-%dT%H:%M:%S%z")
        result["new_failures"] = new_failures(result)
        (RESULTS / f"{sha}-{profile}.json").write_text(json.dumps(result, indent=2) + "\n")
        if ref in ("origin/master", "master"):
            (RESULTS / f"master-latest-{profile}.json").write_text(
                json.dumps(result, indent=2) + "\n"
            )
        print(
            json.dumps(
                {k: result[k] for k in ("sha", "profile", "steps", "new_failures")}, indent=2
            )
        )


def new_failures(result):
    base_path = RESULTS / f"master-latest-{result['profile']}.json"
    if not base_path.exists():
        return None
    base = json.loads(base_path.read_text())
    return {
        name: sorted(set(fails) - set(base["failures"].get(name, [])))
        for name, fails in result["failures"].items()
    }


def submit(ref, build_jobs, target_dir, profile="retail"):
    target_dir = validate_target_dir(target_dir)
    unit = f"full-suite-{profile}-{int(time.time())}"
    subprocess.run(
        [
            "systemd-run",
            "--user",
            f"--unit={unit}",
            "--collect",
            "--slice=agents.slice",
            "-p",
            "CPUQuota=1200%",
            "-p",
            "MemoryHigh=16G",
            "-p",
            "MemoryMax=16G",
            f"--setenv=FULL_SUITE_JOBS={JOBS}",
            sys.executable,
            os.path.abspath(__file__),
            "run",
            ref,
            "--profile",
            profile,
            "--build-jobs",
            str(build_jobs),
            "--target-dir",
            str(target_dir),
        ],
        check=True,
    )
    print(unit)


def status(ref):
    if ref:
        sha = resolve(ref)
    else:
        runs = [
            p
            for p in RESULTS.glob("*.json")
            if not p.name.startswith("master-latest")
        ]
        if not runs:
            print("no result yet")
            return
        latest = max(runs, key=lambda p: p.stat().st_mtime)
        sha = json.loads(latest.read_text())["sha"]
    profiles = {}
    for profile in PROFILES:
        path = RESULTS / f"{sha}-{profile}.json"
        profiles[profile] = json.loads(path.read_text()) if path.exists() else None
    result = {"sha": sha, "profiles": profiles}
    legacy = RESULTS / f"{sha}.json"
    if legacy.exists():
        result["legacy_retail"] = json.loads(legacy.read_text())
    if not any(profiles.values()) and not legacy.exists():
        print("no result yet")
        return
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    actions = parser.add_subparsers(dest="action", required=True)
    for action in ("run", "submit"):
        command = actions.add_parser(action)
        command.add_argument("ref", nargs="?", default="origin/master")
        command.add_argument("--profile", choices=PROFILES, default="retail")
        command.add_argument("--build-jobs", type=positive_build_jobs, default=1)
        command.add_argument("--target-dir", required=True)
    status_parser = actions.add_parser("status")
    status_parser.add_argument("ref", nargs="?")
    args = parser.parse_args(sys.argv[1:] or ["status"])
    if args.action == "status":
        status(args.ref)
    else:
        try:
            if args.action == "run":
                run(args.ref, args.build_jobs, args.target_dir, args.profile)
            else:
                submit(args.ref, args.build_jobs, args.target_dir, args.profile)
        except ValueError as error:
            parser.error(str(error))
