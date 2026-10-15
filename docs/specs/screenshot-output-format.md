# Screenshot output format

`wow-sim screenshot` and live `wow-cli screenshot` select screenshot encoding explicitly. Source lives in `src/screenshot_output.rs` and the CLI/live capture paths below; see the [CLI reference](../wiki/reference/cli-commands.md#screenshot).

## What it must do

- [ ] Default to lossy WebP, quality 15 for standalone capture and 65 for live IPC; force `.webp` regardless of the requested extension, including `-o frame.png`.
- [ ] Accept `--png` in both CLIs, propagate the choice through live IPC, write lossless PNG preserving supplied RGBA bytes, and force `.png` regardless of the requested extension.
- [ ] Preserve the default 1600×1200 viewport and existing source pixels, crop, color handling, and layout; format selection changes encoding only.
- [ ] Report write failures rather than success; success messages identify the actual output path.
- [ ] Leave Windows screenshot IPC unsupported.

## How it works

- [Screenshot commands](../wiki/reference/cli-commands.md#screenshot)
- [Rendering pipeline](../rendering-pipeline.md)

## Implementation inventory

- `src/screenshot_output.rs` — shared encoder and forced extension.
- `src/bin/wow_sim/{main.rs,gui_commands.rs}` — standalone option, capture, crop, and quality 15 save.
- `src/bin/wow_cli/main.rs` — live option and request.
- `src/lua_server.rs`, `src/lua_server_windows.rs` — IPC request transport and unsupported Windows client.
- `src/iced_app/{update_servers.rs,screenshot.rs}` — live dispatch, capture, crop, and quality 65 save.

## Tests asserting this spec

Authored tests; current PNG implementation has not been verified:

- `src/screenshot_output.rs`: `png_preserves_every_rgba_byte_and_forces_extension`, `webp_has_riff_webp_signature_and_forces_extension`, `missing_parent_returns_write_error_for_both_formats`, `directory_destination_returns_write_error_for_both_formats` — encoder fixtures, extensions, and write errors only; not rendered output.
- `src/bin/wow_sim/main.rs`: `screenshot_png_selection_preserves_defaults` — standalone parsing/defaults only.
- `src/lua_server/screenshot_tests.rs`: actual Unix socket framing, public client forwarding, mpsc selection, omitted-field default, and response roundtrip.
- `src/iced_app/update_servers.rs`: `screenshot_commands_save_selected_format_after_crop` — live App command dispatch, GPU capture, crop, output signatures/dimensions, and success paths.
- `src/bin/wow_cli/main.rs`: `screenshot_png_flag_parses`, `screenshot_png_choice_survives_wire_roundtrip`, `screenshot_defaults_are_unchanged` — live parsing, serialization, and default control only; not a running-server capture.

## Known gaps (current cycle)

- [ ] Establish current-source GREEN. Supplied corrected-argv RED at `76cb61c40779226bf1d8a224db7abd65a3e793d9` records two failures (PNG parsing and wire roundtrip) and one passing default control; it does not verify the added PNG implementation.
- [ ] Verify standalone and live capture end to end, including decoded PNG pixels, output paths/messages, and crop/viewport/color/layout preservation. Encoder and parser tests alone cannot establish these boundaries.

## Out of scope

Windows IPC support, renderer/color/layout changes, viewport or crop changes, and WebP quality changes: this contract selects encoding only.
