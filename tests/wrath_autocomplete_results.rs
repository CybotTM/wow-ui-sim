//! Build 12340 contract relayed in `autocomplete-results-12340.md` (2026-10-21).
//! Static reconstruction, not native execution. These tests also reach the
//! original FrameXML consumer; the modern table-returning API remains separate.
#![cfg(feature = "client-wrath")]

use crate::common::wrath_autocomplete::assert_native;

#[test]
fn native_text_change_callback_queries_only_user_input() {
    assert_native(
        r#"
F09EditBox:SetText('F09A')
AutoCompleteEditBox_OnTextChanged(F09EditBox, false)
assert(not AutoCompleteBox:IsShown())
AutoCompleteEditBox_OnTextChanged(F09EditBox, true)
assert(AutoCompleteBox:IsShown())
assert(AutoCompleteButton1:GetText() == 'F09Alpha')
"#,
    );
}

#[test]
fn native_character_callback_completes_suffix_and_preserves_cursor() {
    assert_native(
        r#"
F09EditBox.addHighlightedText = true
F09EditBox:SetText('F09A')
F09EditBox:SetCursorPosition(4)
AutoCompleteEditBox_OnChar(F09EditBox)
assert(F09EditBox:GetText() == 'F09Alpha')
assert(F09EditBox:GetCursorPosition() == 4)
assert(AutoCompleteBox:IsShown())
assert(AutoComplete_GetNumResults(AutoCompleteBox) == 1)
"#,
    );
}

#[test]
fn native_query_returns_strings_to_original_framexml() {
    assert_native(
        r#"
local results = {GetAutoCompleteResults('F09A', AUTOCOMPLETE_FLAG_ALL, 0, 6, 4)}
assert(#results == 1 and results[1] == 'F09Alpha')
AutoComplete_Update(F09EditBox, 'F09A', 4)
assert(AutoCompleteButton1:GetText() == 'F09Alpha')
assert(AutoComplete_GetNumResults(AutoCompleteBox) == 1)
"#,
    );
}

#[test]
fn native_no_match_returns_zero_values_and_hides_original_box() {
    assert_native(
        r#"
assert(select('#', GetAutoCompleteResults('NoF09Match', AUTOCOMPLETE_FLAG_ALL, 0, 6)) == 0)
AutoComplete_UpdateResults(AutoCompleteBox, 'F09Alpha')
AutoComplete_Update(F09EditBox, 'NoF09Match', 10)
assert(not AutoCompleteBox:IsShown())
assert(AutoComplete_GetNumResults(AutoCompleteBox) == 0)
"#,
    );
}

#[test]
fn native_positive_limit_caps_results_and_drives_continuation() {
    assert_native(
        r#"
assert(select('#', GetAutoCompleteResults('F09', AUTOCOMPLETE_FLAG_ALL, 0, 2)) == 2)
AutoComplete_Update(F09EditBox, 'F09', 3)
assert(AutoComplete_GetNumResults(AutoCompleteBox) == 4)
assert(AutoCompleteButton5:GetText() == CONTINUED)
assert(not AutoCompleteButton5:IsEnabled())
"#,
    );
}

#[test]
fn native_nonpositive_limits_leave_results_uncapped() {
    assert_native(
        r#"
for _, limit in ipairs({0, -1}) do
 local count = select('#', GetAutoCompleteResults('F09', AUTOCOMPLETE_FLAG_ALL, 0, limit))
 assert(count == 6, 'nonpositive limit must not cap results')
end
AutoComplete_Update(F09EditBox, 'F09', 3)
assert(AutoCompleteButton5:GetText() == CONTINUED)
"#,
    );
}
