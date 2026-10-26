# XML binding dispatch

Loaded `Bindings.xml` entries provide named keybinding actions. A key assigned to an action executes its loaded XML body. When no body is loaded for an action, the existing simulator action registry remains available. The loaded body takes precedence over that registry.

A conventional `Bindings.xml` beside an addon TOC is loaded with the addon, including the Wrath `FrameXML` core. Binding chunks retain their XML source path and original line for Lua diagnostics.

The Wrath spellbook, talent, guild, and dungeon finder binding tests exercise the native binding bodies and their panel open/close transitions. `binding_xml_dispatch` exercises a named addon binding on repeated key presses.
