-- Wrath post-load workarounds that need to wrap functions defined by
-- FrameXML / Blizzard_* addons (i.e. functions that don't exist when
-- compat_bootstrap.lua runs). Loaded from `apply_post_load_workarounds`
-- after addon loading completes.

-- ScrollingEdit cursor-offset guard.
--
-- Wrath FrameXML/UIPanelTemplates.lua's ScrollingEdit_OnUpdate dereferences
-- self.cursorOffset unconditionally. Real WoW lets OnCursorChanged fire
-- before OnUpdate, so cursorOffset is always set; in the simulator the
-- order isn't guaranteed and OnTextChanged can flag handleCursorChange=true
-- before any OnCursorChanged has run, leading to "arithmetic on nil 'cursorOffset'".
--
-- Wrap the function so it early-returns if cursorOffset/cursorHeight aren't
-- initialized yet. Idempotent: only wrap once.
if type(ScrollingEdit_OnUpdate) == "function"
   and rawget(_G, "__wow_sim_wrath_scrolling_edit_wrapped") ~= true then
  local original = ScrollingEdit_OnUpdate
  function ScrollingEdit_OnUpdate(self, elapsed, scrollFrame)
    if self == nil or self.cursorOffset == nil or self.cursorHeight == nil then
      return
    end
    return original(self, elapsed, scrollFrame)
  end
  rawset(_G, "__wow_sim_wrath_scrolling_edit_wrapped", true)
end
