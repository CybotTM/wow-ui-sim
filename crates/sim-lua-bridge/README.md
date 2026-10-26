# sim-lua-bridge

Feature-neutral rilua extraction/injection traits, frame identity, table helpers,
and registration macros. The root re-exports the module and all exported macros.
Macro `$crate` paths bind here; helper exports must stay public across that boundary.
`val_to_i32` is always compiled, rather than epoch-gating this package.
