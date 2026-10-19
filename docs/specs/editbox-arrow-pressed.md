# EditBox arrow scripts

EditBox accepts `OnArrowPressed(self, key)` and dispatches it through the focused keyboard input path in `src/lua_api/key_dispatch.rs`. Script support lives in `src/lua_api/frame/methods/text_attribute_event/events.rs`; existing XML parsing and template binding already represent this handler. See [keyboard dispatch](../event-system.md).

## What it must do

- [ ] EditBox supports `SetScript`, deferred `HookScript`, `GetScript`, `HasScript` and clearing `OnArrowPressed`; ordinary Frame does not acquire this handler.
- [ ] Focused EditBox dispatches exactly `self` and an uppercase `LEFT`, `RIGHT`, `UP` or `DOWN` direction. Home/End do not emit arrow callbacks. Left/Right retain cursor movement and arrow input does not change text.
- [ ] Generic EditBox alt-arrow mode defaults to false. False accepts plain arrows. True requires Alt for EditBox arrow handling; plain arrows neither invoke the handler nor move its cursor and remain available to bindings.
- [ ] Alt is recognized from modifier state or an `ALT-` key prefix. Script arguments exclude modifier prefixes. An unfocused EditBox receives no arrow callback.
- [ ] The original Blizzard `AutoCompleteEditBoxTemplate` XML binds its original `AutoCompleteEditBox_OnArrowPressed` function, and keyboard Up/Down changes the original autocomplete selection, including wraparound.

The false default is supported by wowless's EditBox `altArrowKeyMode.init` and Blizzard's `UI.xsd` `ignoreArrows` default. Blizzard's chat EditBox template explicitly requests `ignoreArrows="true"`; that is not the generic EditBox default. Blizzard AutoComplete temporarily calls `SetAltArrowKeyMode(false)` when showing suggestions and restores the previous mode when hiding them.

## How it works

- [Event and keyboard dispatch](../event-system.md)
- [XML/template loading](../xml-template-system.md)

## Implementation inventory

- `src/lua_api/frame/methods/text_attribute_event/events.rs` — EditBox-only script support.
- `src/lua_api/key_dispatch.rs` — mode-aware routing, direction argument and existing cursor editing.
- `src/lua_api/frame/methods/widgets/editbox.rs` — existing alt-arrow mode setter/getter, unchanged.
- `src/widget/frame_defaults.rs` — existing false default, unchanged.
- `src/xml/types_support.rs`, `src/loader/helpers.rs`, `src/lua_api/globals/create_frame/template_chain.rs` — existing XML script parsing/binding, unchanged.

## Tests asserting this spec

`tests/editbox_arrow_pressed.rs` covers script APIs and the deferred timer hook pattern, four directions, cursor/text preservation, alt mode and binding routing, focus exclusion, and original cached Blizzard XML/Lua consumer behavior. `tests/key_dispatch.rs` provides existing keyboard-dispatch preservation coverage.

## Known gaps (current cycle)

- [ ] Native-client parity and the full Ellesmere options acceptance rerun are not established by these bounded tests.

## Out of scope

IME composition handlers, XML `ignoreArrows` attribute modeling, Ctrl word navigation, Shift selection, multiline vertical cursor layout and chat-history navigation are separate compatibility work. No Blizzard or third-party addon behavior is modified.
