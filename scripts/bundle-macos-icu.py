"""Bundle the linked Homebrew ICU closure before ad-hoc signing the app."""

from pathlib import Path
import shutil
import subprocess
import sys


def dependencies(path):
    output = subprocess.check_output(["otool", "-L", str(path)], text=True)
    return [line.strip().split(" (compatibility version", 1)[0]
            for line in output.splitlines()[1:]]


def bundle(executable, icu_lib):
    destination = executable.parent.parent / "Frameworks"
    destination.mkdir(exist_ok=True)
    pending = [executable]
    copied = set()
    while pending:
        binary = pending.pop()
        for dependency in dependencies(binary):
            name = Path(dependency).name
            if not name.startswith("libicu"):
                continue
            source = (icu_lib / name).resolve(strict=True)
            target = destination / name
            if name not in copied:
                shutil.copyfile(source, target)
                target.chmod(0o755)
                subprocess.run(["install_name_tool", "-id",
                                f"@executable_path/../Frameworks/{name}", str(target)], check=True)
                copied.add(name)
                pending.append(target)
            subprocess.run(["install_name_tool", "-change", dependency,
                            f"@executable_path/../Frameworks/{name}", str(binary)], check=True)
    if not copied:
        raise RuntimeError("Retail executable has no ICU dependencies to bundle")
    for binary in [executable, *sorted(destination.glob("*.dylib"))]:
        for dependency in dependencies(binary):
            if dependency.startswith(("/usr/lib/", "/System/Library/")):
                continue
            prefix = "@executable_path/../Frameworks/"
            if dependency.startswith(prefix) and (destination / dependency[len(prefix):]).is_file():
                continue
            raise RuntimeError(f"Unbundled dependency in {binary}: {dependency}")


if __name__ == "__main__":
    bundle(Path(sys.argv[1]).resolve(strict=True), Path(sys.argv[2]).resolve(strict=True))
