use crate::loader::LoadResult;
use crate::lua_api::LoaderEnv;

pub(super) fn patch_map_canvas_scroll_container(env: &LoaderEnv<'_>, result: &mut LoadResult) {
    if let Err(e) = env.exec(MAP_CANVAS_SCROLL_CONTAINER_PATCH) {
        super::push_patch_warning(
            result,
            "Blizzard_MapCanvas",
            "patch map canvas scroll container",
            &e,
        );
    }
}

const MAP_CANVAS_SCROLL_CONTAINER_PATCH: &str = r#"
    local function __wow_find_first_scroll_frame_child(parent)
        if type(parent) ~= "table" or type(parent.GetNumChildren) ~= "function" or type(parent.GetChildren) ~= "function" then
            return nil
        end

        local count = parent:GetNumChildren()
        for index = 1, count do
            local child = select(index, parent:GetChildren())
            if type(child) == "table" then
                local isScrollFrame =
                    (type(child.IsObjectType) == "function" and child:IsObjectType("ScrollFrame")) or
                    (type(child.GetObjectType) == "function" and child:GetObjectType() == "ScrollFrame")
                if isScrollFrame then
                    return child
                end
            end
        end

        return nil
    end

    local function __wow_ensure_map_canvas_scroll_container(frame)
        if type(frame) ~= "table" then
            return nil
        end

        local existing = rawget(frame, "ScrollContainer")
        if existing ~= nil then
            return existing
        end

        local scroll = __wow_find_first_scroll_frame_child(frame)
        if scroll ~= nil then
            rawset(frame, "ScrollContainer", scroll)
        end
        return scroll
    end

    local function __wow_try_init_map_canvas(frame)
        if type(frame) ~= "table" then
            return
        end

        __wow_ensure_map_canvas_scroll_container(frame)
        if rawget(frame, "__wow_map_canvas_onload_ran") then
            return
        end

        local scroll = rawget(frame, "ScrollContainer")
        if scroll == nil then
            return
        end

        rawset(frame, "__wow_map_canvas_onload_ran", true)
        local originalOnLoad = rawget(_G, "__wow_map_canvas_original_onload")
        if type(originalOnLoad) == "function" then
            originalOnLoad(frame)
        end
    end

    if type(MapCanvasMixin) == "table" and not rawget(_G, "__wow_map_canvas_scroll_container_patched") then
        if rawget(_G, "__wow_map_canvas_original_onload") == nil and type(MapCanvasMixin.OnLoad) == "function" then
            _G.__wow_map_canvas_original_onload = MapCanvasMixin.OnLoad
            MapCanvasMixin.OnLoad = function(self, ...)
                if rawget(self, "__wow_map_canvas_onload_ran") then
                    return
                end
                __wow_try_init_map_canvas(self)
            end
        end

        if type(MapCanvasMixin.SetMapID) == "function" then
            local originalSetMapID = MapCanvasMixin.SetMapID
            MapCanvasMixin.SetMapID = function(self, ...)
                __wow_try_init_map_canvas(self)
                return originalSetMapID(self, ...)
            end
        end

        if type(MapCanvasMixin.GetCanvas) == "function" then
            local originalGetCanvas = MapCanvasMixin.GetCanvas
            MapCanvasMixin.GetCanvas = function(self, ...)
                __wow_try_init_map_canvas(self)
                return originalGetCanvas(self, ...)
            end
        end

        if type(MapCanvasMixin.GetCanvasContainer) == "function" then
            local originalGetCanvasContainer = MapCanvasMixin.GetCanvasContainer
            MapCanvasMixin.GetCanvasContainer = function(self, ...)
                __wow_try_init_map_canvas(self)
                return originalGetCanvasContainer(self, ...)
            end
        end

        if type(MapCanvasMixin.OnFrameSizeChanged) == "function" then
            local originalOnFrameSizeChanged = MapCanvasMixin.OnFrameSizeChanged
            MapCanvasMixin.OnFrameSizeChanged = function(self, ...)
                __wow_try_init_map_canvas(self)
                return originalOnFrameSizeChanged(self, ...)
            end
        end

        __wow_map_canvas_scroll_container_patched = true
    end
"#;
