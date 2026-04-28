# Windows Port Build

Windows can build and start the simulator once the GUI dependency avoids building the local `iced-dynamic` DLL.

## Content

The first Windows `cargo build --bin wow-sim` attempt reached MSVC linking and failed while producing `iced_dynamic.dll`:

```text
LINK : fatal error LNK1189: library limit of 65535 objects exceeded
```

`iced-dynamic` is only a re-export crate (`pub use iced::*;`) with `crate-type = ["dylib"]`. On Windows, that forces a very large Iced/WGPU DLL link and hits MSVC's object limit. The porting fix is to depend on upstream `iced` directly from the root crate instead of routing the `iced` dependency through the dynamic wrapper. That keeps the code's `iced::...` imports unchanged and avoids the DLL link step.

Verification on Windows:

- `cargo build --bin wow-sim` succeeds.
- `cargo build --bin wow-cli` succeeds.
- `target\debug\wow-sim.exe --no-addons --no-saved-vars` reaches startup, creates a window, and keeps running as an interactive app.
- `target\debug\wow-sim.exe --no-addons --no-saved-vars screenshot -o windows-smoke.webp --width 640 --height 480` exits successfully and writes the screenshot.

Resource discovery is centralized in `src/paths.rs`. Callers use one resolver for:

- WoW install root: `C:\World of Warcraft`
- CASC data root: `C:\World of Warcraft\Data`
- extracted Interface art: `C:\World of Warcraft\_retail_\BlizzardInterfaceArt\Interface`
- AddOns: repo `./Interface/AddOns` first, then installed `_retail_\Interface\AddOns`
- WTF: `C:\World of Warcraft\_retail_\WTF`

The resolver also supports explicit overrides: `WOW_SIM_WOW_PATH`, `WOW_SIM_CASC_PATH`, `WOW_SIM_INTERFACE_PATH`, `WOW_SIM_ADDONS_PATH`, `WOW_SIM_WTF_PATH`, and `WOW_SIM_TEXTURES_PATH`.

Live WTF is treated as a read-only import source. `SavedVariablesManager` skips live WTF import once a simulator-local saved-variable file exists for the addon, and all saves write to simulator storage, not back to the WoW installation. The regression coverage is:

- `test_wtf_source_is_read_only_when_saving`
- `test_local_storage_takes_precedence_over_wtf_import_source`

Known remaining observations from the smoke runs:

- Cargo prints `warn: could not canonicalize path: 'C:\Users\adeia'` in this environment.
- The app still reports the existing `StoreFrame_CheckForFree` Lua startup error during `VARIABLES_LOADED`.
- Vulkan validation/Galaxy overlay layer warnings can appear during screenshot creation, but rendering still completes.
- `target\debug\wow-sim.exe --no-addons lua-errors` now logs `WTF import source (read-only): 50868465#2 @ Burning Blade/Haky`, confirming the Windows WTF path is found without making it a write target.

## Sources

- [Cargo manifest](../../Cargo.toml) - root dependency selection for `iced`
- [iced-dynamic manifest](../../iced-dynamic/Cargo.toml) - dynamic re-export crate that triggered the Windows DLL link
- [path resolver](../../../src/paths.rs) - shared WoW resource discovery for CASC, Interface, AddOns, and WTF
- [saved variables](../../../src/saved_variables.rs) - live WTF import and simulator-local write behavior

## See Also

- [[cli-commands]] - smoke-test commands used for build and screenshot verification
- [[rendering-pipeline]] - screenshot path and GPU renderer context
