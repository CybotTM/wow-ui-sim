# Enter-to-chat binding

Current Retail/PTR simulator default Enter dispatch must open the unchanged Blizzard chat editor from an unfocused startup state. Historical Blizzard manual §5.2 describes Enter opening the chat prompt; this grounds the usability requirement, not an exhaustive current native keymap. Cached Retail/PTR `Blizzard_DeprecatedChatInfo/Deprecated_ChatFrame.lua` aliases `ChatFrame_OpenChat` to the whole `ChatFrameUtil.OpenChat` consumer. Default binding is simulator policy; original handlers and configured bindings remain authoritative within this model.

## What it must do

- [ ] An unmodified Enter key through ordinary loaded-game dispatch opens/focuses the real chat editor. Subsequent printable input reaches that editor; subsequent Enter uses its original submit handler, clearing text and releasing focus without reopening chat.
- [ ] Explicit Enter binding overrides the default; explicit Enter unbinding suppresses it. Do not alter configured/account bindings to install this default.
- [ ] Preserve focused EditBox special-key dispatch and existing non-Enter bindings. No copied/replaced Blizzard handlers or VNC-specific keyboard path.

## How it works

- [Event system](../event-system.md)
- [Prefork harness](../wiki/systems/prefork-test-harness.md)

## Implementation inventory

- `src/lua_api/globals/keybindings.rs`: existing binding actions/default-key dispatch; Enter uses `OPENCHAT` and the unchanged `ChatFrame_OpenChat` consumer.
- `src/lua_api/key_dispatch.rs`: existing focused EditBox and general key dispatch; unchanged.
- `src/iced_app/view.rs`: existing native/toolkit keyboard subscription; unchanged.

## Tests asserting this spec

- [chat_enter_binding.rs](../../tests/chat_enter_binding.rs): three loaded Game-parent/fork cases. At `7e8c5ffb6`, default activation failed before downstream typing/submission, while configured/cleared preservation controls passed; prefork exit1. GREEN remains pending.
- [chat_frame.rs](../../tests/chat_frame.rs): existing click/type/submit consumer proof; not a default Enter activation test.

## Known gaps (current cycle)

- [ ] Complete focused GREEN after genuine default-activation RED; typing/submission assertions remain unreached in RED.
- [ ] Focused GREEN, native GUI keypress smoke and intended-user keyboard acceptance remain pending.

## Out of scope

Native account-keymap parity, network chat delivery, all-key/gameplay support, vendor changes, original WTF mutation, and treating GetBindingAction's current omission of simulator convenience defaults as proof that dispatch has no default. Earlier-profile/native acceptance remains separate.
