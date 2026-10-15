# Static security review

Reviewed: 2026-10-15. External checkout: `/home/osso/Repos/wowdecomp`, revision `0e4e9fe0a4a9fa08027c5eed4b2c5e873739cfd3`.

## Decision

**No confirmed malicious implant found in the inspected paths. This is not a declaration that the repository is safe to execute.** Continue with source-only comparison; do not configure, compile, install or run it. No external repository code was executed during this audit.

The client contains networking, credential handling, native library loading and an elevated patch executor. These capabilities are consequential even if they implement an authentic game client rather than malware. Static review cannot establish intent or exclude hidden malicious behavior.

## Inspection and inventory

Clone command, executed through an argv-style command helper:

```text
git -c core.hooksPath=/dev/null clone --no-recurse-submodules https://github.com/SatyPardus/wowdecomp /home/osso/Repos/wowdecomp
```

- Git inventory: **3,555 tracked files**, all mode `100644`; no tracked symlinks, executable bits or gitlinks. No `.gitmodules` or GitHub workflow directory. Mode does not determine whether content can execute.
- Hidden tracked files: `.gitattributes`, `.gitignore` and two FreeType `.cvsignore` files. `.gitattributes` only requests automatic text normalization; no repository-supplied filter command.
- An initial-8-KiB NUL-byte inventory identified 41 files: assets, compressed/test data, documentation and **precompiled NVAPI/Xvid `.lib` libraries**. Those libraries are opaque compiled inputs, not source-audited dependencies. No tracked `.exe`/`.dll` was found. This heuristic is not full binary identification.
- Read root README/build definitions and sensitive build, update, login, HTTP, dynamic-loading and crash-log paths. Pattern-scanned the remaining tree for process/network/credential/registry/persistence and build-time execution indicators. Most files were not manually read; large search displays were truncated, followed by targeted inspection of significant hits.

## Sensitive boundaries

Paths below are relative to the pinned external checkout. Line numbers refer to that revision.

| Boundary | Source evidence | Assessment |
|---|---|---|
| Patch archive can write/delete files and launch commands | `WoW/Source/Glue/PatchDownloadGlue.cpp:56-132,334-354`; `WoW/Source/Glue/Win64/PatchDownloadGlue.cpp:11-34` | `prepatch.lst` accepts `extract`, `execute`, `delete`. The caller first authenticates the archive and requires `authResult >= 5`. On Windows version 6+, command launch requests `runas` elevation. High-impact trust boundary, **not a demonstrated signature bypass or malicious payload**. Archive authentication semantics and path confinement remain unproved. |
| Downloaded Scan DLL becomes native code | `WoW/Source/Glue/ScanDLLGlue.cpp:104-128,183-225,254-307`; `WoW/Source/WowSvcs/WowSvcsClient/CheckExecutableSignature.cpp:32-109,135-160`; `Engine/Source/Os/Win64/OsLibrary.cpp:6-19` | Download/replacement followed by signature verification and DLL loading. No bypass established. Network-to-native-code boundary remains execution risk. |
| Optional signature-policy weakening | `CMakeLists.txt:50-55`; `README.md:79-82` | `WOWDECOMP_DISABLE_SIGNATURE_CHECKS` defaults OFF. Enabling it accepts modified/unsigned MPQ/interface content. The inspected Scan DLL checker has its own signature check; do **not** infer this option disables that check. |
| Crash log replacement | `Tools/WowClient/crash_log.cpp:131-152` | Deletes/reopens `WowClient_crash.log` beside the executable. Documented logging, not evidence of stealth persistence; writable-directory races/ACLs untested. |
| Authentication and network traffic | `WoW/Source/WowSvcs/WowSvcsClient/ClientServices.cpp:420-461,826-891,1622-1628`; `Engine/Source/BlizzardCore/Source/System/URL/Win64/URL.cpp:268-376`; `WoW/Source/WowServices/Win64/WowConnection.cpp:659-733` | Credentials/tokens enter login implementations; HTTP(S) and TCP are supported. No credential theft established in inspected paths. Destinations, TLS behavior, storage and protocol security were not exhaustively audited. |

## Build entry points inspected, not executed

Root CMake includes local source directories. Scanned CMake files had no `FetchContent`, `ExternalProject` or `file(DOWNLOAD)` hit. This is bounded search evidence, not a guarantee against indirect downloads.

- `ThirdParty/CMakeLists.txt:143-150`: configure-time `${CMAKE_AR}` creates a local import library from `DivxDecoder.def`.
- `ThirdParty/CMakeLists.txt:159-169`: optionally copies a user-supplied `DivxDecoder.dll`; none was tracked in this checkout.
- `Engine/Source/SoundEngine/CMakeLists.txt:77-95`: NASM compiles local assembly inputs.
- Bundled third-party shell/batch/Python helpers were inventoried but not executed; their upstream identity and modifications were not authenticated.

## Export-name false positive checked during the expanded inventory

`WoW/Source/Ui/ScriptEvents_Scripts.cpp:4364-4371` lists `ReadFile`, `DeleteFile` and `AppendToFile`, but their bodies at `:779-795` explicitly raise `Access Denied`. These names alone are not evidence of arbitrary addon filesystem access or a malicious implant. The broader source/runtime and dependency limits below remain unchanged.

## Remaining limits

No runtime, sandbox execution, antivirus verdict, dependency authenticity verification, complete manual source review, original executable comparison, or exhaustive endpoint/authentication analysis. Git revision pins bytes, not trustworthiness. Repository `OFFSET`/`VERIFY` comments and README completion claims are author assertions, not independent evidence of original-client equivalence.

Continue using this checkout as inert research material. Any decision to run it requires a separate execution/security assessment; this audit does not authorize it.

## Related

- [Audit index and behavioral coverage](README.md)
