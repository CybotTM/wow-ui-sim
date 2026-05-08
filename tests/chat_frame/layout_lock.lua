local EPS = 0.75

local function approx(actual, expected, eps)
    if type(actual) ~= "number" or type(expected) ~= "number" then
        return false
    end
    return math.abs(actual - expected) <= (eps or EPS)
end

local function rect(frame, tag)
    if type(frame) ~= "table" then
        return nil, tag .. "_missing"
    end
    local l, b, w, h = frame:GetRect()
    if not (l and b and w and h) then
        return nil, tag .. "_missing_rect"
    end
    return { l = l, b = b, w = w, h = h, r = l + w, t = b + h }, nil
end

local function has_point(frame, point, rel, relPoint, x, y, eps)
    for i = 1, frame:GetNumPoints() do
        local p, r, rp, ox, oy = frame:GetPoint(i)
        if p == point and r == rel and rp == relPoint and approx(ox or 0, x, eps) and approx(oy or 0, y, eps) then
            return true
        end
    end
    return false
end

local chat = ChatFrame1
local background = ChatFrame1Background
local editBox = ChatFrame1EditBox
local scrollBar = chat and chat.ScrollBar or nil
local scrollToBottom = chat and chat.ScrollToBottomButton or nil
local resizeButton = ChatFrame1ResizeButton
local buttonFrame = ChatFrame1ButtonFrame
local menuButton = ChatFrameMenuButton
local voiceButton = ChatFrameChannelButton

if not chat then return "chat_missing" end
if not background then return "background_missing" end
if not editBox then return "editbox_missing" end
if not scrollBar then return "scrollbar_missing" end
if not scrollToBottom then return "scroll_to_bottom_missing" end
if not resizeButton then return "resize_button_missing" end
if not buttonFrame then return "button_frame_missing" end
if not menuButton then return "menu_button_missing" end
if not voiceButton then return "voice_button_missing" end

local chatRect, chatErr = rect(chat, "chat")
if not chatRect then return chatErr end
local bgRect, bgErr = rect(background, "background")
if not bgRect then return bgErr end
local editRect, editErr = rect(editBox, "editbox")
if not editRect then return editErr end
local scrollRect, scrollErr = rect(scrollBar, "scrollbar")
if not scrollRect then return scrollErr end
local scrollToBottomRect, scrollToBottomErr = rect(scrollToBottom, "scroll_to_bottom")
if not scrollToBottomRect then return scrollToBottomErr end
local resizeRect, resizeErr = rect(resizeButton, "resize")
if not resizeRect then return resizeErr end
local buttonRect, buttonErr = rect(buttonFrame, "button_frame")
if not buttonRect then return buttonErr end
local menuRect, menuErr = rect(menuButton, "menu_button")
if not menuRect then return menuErr end
local voiceRect, voiceErr = rect(voiceButton, "voice_button")
if not voiceRect then return voiceErr end

if not chat:IsShown() then
    return "chat_hidden"
end
if not background:IsShown() then
    return "background_hidden"
end
if not editBox:IsShown() then
    return "editbox_should_start_shown"
end
if scrollBar:IsShown() then
    return "scrollbar_should_start_hidden"
end
if resizeButton:IsShown() then
    return "resize_button_should_start_hidden"
end
if not menuButton:IsShown() then
    return "menu_button_hidden"
end
if not voiceButton:IsShown() then
    return "voice_button_hidden"
end

if chat:GetNumPoints() ~= 1 then
    return "chat_points=" .. tostring(chat:GetNumPoints())
end
if not has_point(chat, "BOTTOMLEFT", UIParent, "BOTTOMLEFT", 35, 50, 0.1) then
    return "chat_anchor_mismatch"
end
if not approx(chatRect.w, 430, 0.1) or not approx(chatRect.h, 170, 0.1) then
    return "chat_size=" .. tostring(chatRect.w) .. "x" .. tostring(chatRect.h)
end

if background:GetNumPoints() ~= 4 then
    return "background_points=" .. tostring(background:GetNumPoints())
end
if not approx(bgRect.w, 447, 0.1) or not approx(bgRect.h, 179, 0.1) then
    return "background_size=" .. tostring(bgRect.w) .. "x" .. tostring(bgRect.h)
end
if not approx(bgRect.l, chatRect.l - 2, 0.1) then
    return "background_left=" .. tostring(bgRect.l) .. " chat_left=" .. tostring(chatRect.l)
end
if not approx(bgRect.r, chatRect.r + 15, 0.1) then
    return "background_right=" .. tostring(bgRect.r) .. " chat_right=" .. tostring(chatRect.r)
end
if not approx(bgRect.b, chatRect.b - 6, 0.1) then
    return "background_bottom=" .. tostring(bgRect.b) .. " chat_bottom=" .. tostring(chatRect.b)
end
if not approx(bgRect.t, chatRect.t + 3, 0.1) then
    return "background_top=" .. tostring(bgRect.t) .. " chat_top=" .. tostring(chatRect.t)
end

if buttonFrame:GetNumPoints() ~= 2 then
    return "button_frame_points=" .. tostring(buttonFrame:GetNumPoints())
end
if not approx(buttonRect.w, 29, 0.1) or not approx(buttonRect.h, 170, 0.1) then
    return "button_frame_size=" .. tostring(buttonRect.w) .. "x" .. tostring(buttonRect.h)
end
if not approx(buttonRect.r, bgRect.l - 3, 0.1) then
    return "button_frame_right=" .. tostring(buttonRect.r) .. " bg_left=" .. tostring(bgRect.l)
end
if not approx(buttonRect.t, bgRect.t - 3, 0.1) then
    return "button_frame_top=" .. tostring(buttonRect.t) .. " bg_top=" .. tostring(bgRect.t)
end
if not approx(buttonRect.b, bgRect.b + 6, 0.1) then
    return "button_frame_bottom=" .. tostring(buttonRect.b) .. " bg_bottom=" .. tostring(bgRect.b)
end

if menuButton:GetNumPoints() ~= 1 then
    return "menu_button_points=" .. tostring(menuButton:GetNumPoints())
end
if not has_point(menuButton, "BOTTOM", buttonFrame, "BOTTOM", 0, 0, 0.1) then
    return "menu_button_anchor_mismatch"
end
if not approx(menuRect.w, 32, 0.1) or not approx(menuRect.h, 32, 0.1) then
    return "menu_button_size=" .. tostring(menuRect.w) .. "x" .. tostring(menuRect.h)
end
if not approx(menuRect.b, buttonRect.b, 0.1) then
    return "menu_button_bottom=" .. tostring(menuRect.b) .. " button_bottom=" .. tostring(buttonRect.b)
end
local menuCenter = menuRect.l + (menuRect.w / 2)
local buttonCenter = buttonRect.l + (buttonRect.w / 2)
if not approx(menuCenter, buttonCenter, 0.1) then
    return "menu_button_center=" .. tostring(menuCenter) .. " button_center=" .. tostring(buttonCenter)
end

if voiceButton:GetNumPoints() ~= 1 then
    return "voice_button_points=" .. tostring(voiceButton:GetNumPoints())
end
if not has_point(voiceButton, "TOP", buttonFrame, "TOP", 0, 0, 0.1) then
    return "voice_button_anchor_mismatch"
end
if not approx(voiceRect.w, 27, 0.1) or not approx(voiceRect.h, 26, 0.1) then
    return "voice_button_size=" .. tostring(voiceRect.w) .. "x" .. tostring(voiceRect.h)
end
if not approx(voiceRect.t, buttonRect.t, 0.1) then
    return "voice_button_top=" .. tostring(voiceRect.t) .. " button_top=" .. tostring(buttonRect.t)
end
local voiceCenter = voiceRect.l + (voiceRect.w / 2)
if not approx(voiceCenter, buttonCenter, 0.1) then
    return "voice_button_center=" .. tostring(voiceCenter) .. " button_center=" .. tostring(buttonCenter)
end

if resizeButton:GetNumPoints() ~= 1 then
    return "resize_points=" .. tostring(resizeButton:GetNumPoints())
end
if not approx(resizeRect.w, 16, 0.1) or not approx(resizeRect.h, 16, 0.1) then
    return "resize_size=" .. tostring(resizeRect.w) .. "x" .. tostring(resizeRect.h)
end
if not has_point(resizeButton, "BOTTOMRIGHT", chat, "BOTTOMRIGHT", 0, 0, 0.1) then
    return "resize_anchor_mismatch"
end

if scrollToBottom:GetNumPoints() ~= 1 then
    return "scroll_to_bottom_points=" .. tostring(scrollToBottom:GetNumPoints())
end
if not has_point(scrollToBottom, "BOTTOMRIGHT", resizeButton, "TOPRIGHT", -2, -2, 0.1) then
    return "scroll_to_bottom_anchor_mismatch"
end
if not approx(scrollToBottomRect.w, 17, 0.1) or not approx(scrollToBottomRect.h, 15, 0.1) then
    return "scroll_to_bottom_size=" .. tostring(scrollToBottomRect.w) .. "x" .. tostring(scrollToBottomRect.h)
end
if not approx(scrollToBottomRect.r, resizeRect.r - 2, 0.1) then
    return "scroll_to_bottom_right=" .. tostring(scrollToBottomRect.r) .. " resize_right=" .. tostring(resizeRect.r)
end
if not approx(scrollToBottomRect.b, resizeRect.t - 2, 0.1) then
    return "scroll_to_bottom_bottom=" .. tostring(scrollToBottomRect.b) .. " resize_top=" .. tostring(resizeRect.t)
end
if not approx(scrollToBottom:GetAlpha(), 0, 0.01) then
    return "scroll_to_bottom_alpha=" .. tostring(scrollToBottom:GetAlpha())
end

if scrollBar:GetNumPoints() ~= 2 then
    return "scrollbar_points=" .. tostring(scrollBar:GetNumPoints())
end
if not has_point(scrollBar, "TOPLEFT", chat, "TOPRIGHT", 0, 0, 0.1) then
    return "scrollbar_top_anchor_mismatch"
end
if not has_point(scrollBar, "BOTTOMLEFT", scrollToBottom, "TOPLEFT", 0, 2, 0.1) then
    return "scrollbar_bottom_anchor_mismatch"
end
if not approx(scrollRect.w, 23, 0.1) then
    return "scrollbar_width=" .. tostring(scrollRect.w)
end
if not approx(scrollRect.h, 139, 0.1) then
    return "scrollbar_height=" .. tostring(scrollRect.h)
end
if not approx(scrollBar:GetAlpha(), 0, 0.01) then
    return "scrollbar_alpha=" .. tostring(scrollBar:GetAlpha())
end

if editBox:GetNumPoints() ~= 2 then
    return "editbox_points=" .. tostring(editBox:GetNumPoints())
end
if not has_point(editBox, "TOPLEFT", chat, "BOTTOMLEFT", -5, -2, 0.1) then
    return "editbox_top_anchor_mismatch"
end
if not has_point(editBox, "RIGHT", scrollBar, "RIGHT", 8, 0, 0.1) then
    return "editbox_right_anchor_mismatch"
end
if not approx(editRect.w, 466, 0.1) or not approx(editRect.h, 32, 0.1) then
    return "editbox_size=" .. tostring(editRect.w) .. "x" .. tostring(editRect.h)
end
if not approx(editRect.l, chatRect.l - 5, 0.1) then
    return "editbox_left=" .. tostring(editRect.l) .. " chat_left=" .. tostring(chatRect.l)
end
if not approx(editRect.r, scrollRect.r + 8, 0.1) then
    return "editbox_right=" .. tostring(editRect.r) .. " scrollbar_right=" .. tostring(scrollRect.r)
end

return "ok"
