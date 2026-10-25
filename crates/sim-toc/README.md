# sim-toc

TOC parsing, metadata, and ordered file entries. No client features or resource discovery.
The root supplies `Selection` with ACTIVE-derived rules and host path/security callbacks.
`wow_ui_sim::toc::TocFile` aliases `TocFile<ActiveSelection>`; generic method code can still
be monomorphized in the root. Compile CPU savings require the slot measurement.
The unchanged selected-profile unit suite is included by the root facade from
`src/selected_profile_tests.rs`; it requires the real selected profile and ScreenKind.
