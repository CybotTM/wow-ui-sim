//! Temporary paging controls page-text workaround.
//!
//! Some loaded paging-control users depend on `UpdateControls` refreshing
//! `PageText`. Keep this wrapper isolated until the simulator models the
//! complete paging-control text update path.

use crate::lua_api::{LoaderEnv, WowLuaEnv};

const PAGING_CONTROLS_PAGE_TEXT_WORKAROUND_LUA: &str = r#"
    if type(PagingControlsMixin) ~= "table"
        or type(PagingControlsMixin.UpdateControls) ~= "function" then
        return
    end

    if rawget(_G, "__wow_paging_controls_update_controls_wrapper") then
        return
    end

    local original_update_controls = PagingControlsMixin.UpdateControls
    PagingControlsMixin.UpdateControls = function(self, ...)
        original_update_controls(self, ...)

        local pageText = self and self.PageText
        if type(pageText) ~= "table" or type(pageText.SetText) ~= "function" then
            return
        end

        local currentPage = tonumber(self.currentPage) or 1
        local maxPages = tonumber(self.maxPages) or 1
        local formatString
        local formatted

        if self.displayMaxPages then
            formatString = self.currentPageWithMaxText or PAGE_NUMBER_WITH_MAX
            formatted = string.format(formatString, currentPage, maxPages)
        else
            formatString = self.currentPageOnlyText or PAGE_NUMBER
            formatted = string.format(formatString, currentPage)
        end

        pageText:SetText(formatted)
    end

    rawset(_G, "__wow_paging_controls_update_controls_wrapper", true)
"#;

pub(crate) fn patch(env: &WowLuaEnv) {
    let _ = env.exec(PAGING_CONTROLS_PAGE_TEXT_WORKAROUND_LUA);
}

pub(crate) fn patch_loader(env: &LoaderEnv<'_>) {
    let _ = env.exec(PAGING_CONTROLS_PAGE_TEXT_WORKAROUND_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_controls_refreshes_page_text_with_max_pages() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            PAGE_NUMBER = "Page %d"
            PAGE_NUMBER_WITH_MAX = "Page %d of %d"
            originalCalls = 0
            PagingControlsMixin = {
                UpdateControls = function(self)
                    originalCalls = originalCalls + 1
                end,
            }
            pageText = {
                SetText = function(self, text)
                    self.text = text
                end,
            }
            controls = {
                currentPage = 3,
                maxPages = 8,
                displayMaxPages = true,
                PageText = pageText,
            }
            "#,
        )
        .expect("paging controls test surface should install");

        patch(&env);

        let (wrapped, original_calls, text): (bool, i64, String) = env
            .eval(
                r#"
                PagingControlsMixin.UpdateControls(controls)
                return __wow_paging_controls_update_controls_wrapper == true,
                    originalCalls,
                    pageText.text
                "#,
            )
            .expect("patched paging controls state should be readable");

        assert!(wrapped);
        assert_eq!(original_calls, 1);
        assert_eq!(text, "Page 3 of 8");
    }

    #[test]
    fn update_controls_uses_page_only_format() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            PAGE_NUMBER = "Page %d"
            PAGE_NUMBER_WITH_MAX = "Page %d of %d"
            PagingControlsMixin = {
                UpdateControls = function() end,
            }
            pageText = {
                SetText = function(self, text)
                    self.text = text
                end,
            }
            controls = {
                currentPage = 4,
                maxPages = 9,
                displayMaxPages = false,
                currentPageOnlyText = "Only %d",
                PageText = pageText,
            }
            "#,
        )
        .expect("paging controls page-only test surface should install");

        patch(&env);

        let text: String = env
            .eval(
                r#"
                PagingControlsMixin.UpdateControls(controls)
                return pageText.text
                "#,
            )
            .expect("patched page-only text should be readable");

        assert_eq!(text, "Only 4");
    }
}
