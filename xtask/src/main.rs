use std::env;
use std::process::{Command, ExitCode};

const RELEASE_FEATURES: &str = "sound,gui";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("release") => release(args.next().as_deref(), args.collect()),
        Some(command) => Err(format!("unknown xtask command: {command}\n{USAGE}")),
        None => Err(USAGE.to_string()),
    }
}

fn release(platform: Option<&str>, extra_cargo_args: Vec<String>) -> Result<(), String> {
    let target = match platform {
        Some("windows") | Some("win") => "x86_64-pc-windows-msvc",
        Some("linux") => "x86_64-unknown-linux-gnu",
        Some("current") | None => current_target()?,
        Some(platform) => return Err(format!("unknown release platform: {platform}\n{USAGE}")),
    };

    let mut command = Command::new("cargo");
    command.args([
        "build",
        "--release",
        "--no-default-features",
        "--features",
        RELEASE_FEATURES,
        "--target",
        target,
        "--bin",
        "wow-sim",
        "--bin",
        "wow-cli",
    ]);
    command.args(extra_cargo_args);

    println!("running: {command:?}");
    let status = command
        .status()
        .map_err(|error| format!("failed to start cargo build: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "release build failed for target {target}: {status}"
        ))
    }
}

fn current_target() -> Result<&'static str, String> {
    if cfg!(target_os = "windows") {
        Ok("x86_64-pc-windows-msvc")
    } else if cfg!(target_os = "linux") {
        Ok("x86_64-unknown-linux-gnu")
    } else {
        Err("current platform is not supported for release builds".to_string())
    }
}

const USAGE: &str = "\
usage:
  cargo xtask release [current|windows|linux] [cargo build args...]
  cargo release-current [cargo build args...]
  cargo release-windows [cargo build args...]
  cargo release-linux [cargo build args...]";
