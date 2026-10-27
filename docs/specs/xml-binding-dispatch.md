# XML binding dispatch

Loaded `Bindings.xml` entries provide named keybinding actions. A key assigned to an action executes its loaded XML body. When no body is loaded for an action, the existing simulator action registry remains available. The loaded body takes precedence over that registry.

A conventional `Bindings.xml` beside an addon TOC is loaded with the addon, including the Wrath `FrameXML` core. Binding chunks retain their XML source path and original line for Lua diagnostics.

A loaded body runs on key-down with global `keystate` set to `"down"`. If its `runOnUp` attribute is true, the same body runs on the matching key-up with `keystate` set to `"up"`; otherwise it does not run on key-up. The simulator leaves `keystate` at its last dispatched value; native lifetime outside the callback is unverified. Wrath `FrameXML/Bindings.xml:124–130` supplies the attribute and the action-button down/up branches. The pinned Wrath `FrameXML/UI.xsd` does not declare `runOnUp`; it is not evidence for that attribute.

The Wrath spellbook, guild, and dungeon finder tests exercise native binding bodies and panel open/close transitions. The Wrath action-button test checks physical down/up against the native binding body and button state. `binding_xml_dispatch` exercises a named addon binding, repeated presses, and both values of `runOnUp`.
