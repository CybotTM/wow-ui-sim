//! Pre-created global frames and frame/texture attribute surface checks.

use super::super::*;

#[test]
fn test_uiparent_exists() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return UIParent:GetObjectType()").unwrap();
    assert_eq!(ty, "Frame");
}

#[test]
fn test_create_frame_exposes_core_event_methods() {
    let env = WowLuaEnv::new().unwrap();
    let registry_mt_type: String = {
        let mut lua = env.lua.borrow_mut();
        let state = lua.state_mut();
        match crate::lua_api::methods::registry_get(state, "__rilua_frame_mt") {
            rilua::Val::Table(_) => "table".to_string(),
            other => other.type_name().to_string(),
        }
    };
    let (set_forbidden, set_script, register_event, get_object_type, mt_type, mt_index_type, mt_set_forbidden, mt_get_object_type): (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ) = env
        .eval(
            r#"
            local f = CreateFrame("Frame")
            local mt = getmetatable(f)
            return type(f.SetForbidden), type(f.SetScript), type(f.RegisterEvent), type(f.GetObjectType),
                type(mt), type(mt and mt.__index), type(mt and mt.SetForbidden), type(mt and mt.GetObjectType)
            "#,
        )
        .unwrap();
    assert_eq!(
        (set_forbidden, set_script, register_event, get_object_type),
        (
            "function".to_string(),
            "function".to_string(),
            "function".to_string(),
            "function".to_string(),
        ),
        "frame surface mismatch: registry={registry_mt_type}, mt={mt_type}, mt.__index={mt_index_type}, mt.SetForbidden={mt_set_forbidden}, mt.GetObjectType={mt_get_object_type}",
    );
}

#[test]
fn test_get_frame_metatable_without_instance_returns_shared_metatable() {
    let env = WowLuaEnv::new().unwrap();
    let (mt_ty, mt_index_ty, get_object_type_ty, set_forbidden_ty): (String, String, String, String) = env
        .eval(
            r#"
            local mt = GetFrameMetatable()
            return type(mt), type(mt and mt.__index), type(mt and mt.GetObjectType), type(mt and mt.SetForbidden)
            "#,
        )
        .unwrap();
    assert_eq!(mt_ty, "table");
    assert_eq!(mt_index_ty, "table");
    assert_eq!(get_object_type_ty, "function");
    assert_eq!(set_forbidden_ty, "function");
}

#[test]
fn test_create_texture_exposes_core_visual_methods() {
    let env = WowLuaEnv::new().unwrap();
    let (
        set_texture,
        set_color_texture,
        set_vertex_color,
        set_blend_mode,
        set_tex_coord,
        set_horiz_tile,
        set_vert_tile,
        set_texel_snapping_bias,
        get_texel_snapping_bias,
        set_snap_to_pixel_grid,
        set_desaturated_ty,
        is_desaturated_ty,
        get_desaturation_ty,
        bias,
        is_desaturated,
        desaturation,
    ): (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        f64,
        bool,
        f64,
    ) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame")
            local tex = frame:CreateTexture()
            tex:SetTexture("Interface\\Buttons\\WHITE8X8")
            tex:SetVertexColor(0.1, 0.2, 0.3, 0.4)
            tex:SetBlendMode("ADD")
            tex:SetColorTexture(0.5, 0.6, 0.7, 0.8)
            tex:SetTexCoord(0, 1, 0, 1)
            tex:SetHorizTile(true)
            tex:SetVertTile(true)
            tex:SetTexelSnappingBias(0.25)
            tex:SetSnapToPixelGrid(true)
            tex:SetDesaturated(true)
            return type(tex.SetTexture), type(tex.SetColorTexture), type(tex.SetVertexColor),
                type(tex.SetBlendMode), type(tex.SetTexCoord), type(tex.SetHorizTile),
                type(tex.SetVertTile), type(tex.SetTexelSnappingBias),
                type(tex.GetTexelSnappingBias), type(tex.SetSnapToPixelGrid),
                type(tex.SetDesaturated), type(tex.IsDesaturated), type(tex.GetDesaturation),
                tex:GetTexelSnappingBias(), tex:IsDesaturated(), tex:GetDesaturation()
            "#,
        )
        .unwrap();
    for ty in [
        set_texture,
        set_color_texture,
        set_vertex_color,
        set_blend_mode,
        set_tex_coord,
        set_horiz_tile,
        set_vert_tile,
        set_texel_snapping_bias,
        get_texel_snapping_bias,
        set_snap_to_pixel_grid,
        set_desaturated_ty,
        is_desaturated_ty,
        get_desaturation_ty,
    ] {
        assert_eq!(ty, "function");
    }
    assert_eq!(bias, 0.25);
    assert!(is_desaturated);
    assert_eq!(desaturation, 1.0);
}

#[test]
fn test_get_children_excludes_layer_regions() {
    let env = WowLuaEnv::new().unwrap();
    let (num_children, num_regions, first_child_key, first_region_key): (i64, i64, String, String) =
        env.eval(
            r#"
            local frame = CreateFrame("Frame")
            local child = CreateFrame("Frame", nil, frame)
            child:SetParentKey("childFrame")
            local texture = frame:CreateTexture()
            texture:SetParentKey("regionTexture")
            local fontString = frame:CreateFontString()
            fontString:SetParentKey("regionText")
            return frame:GetNumChildren(), frame:GetNumRegions(),
                ({ frame:GetChildren() })[1]:GetParentKey(),
                ({ frame:GetRegions() })[1]:GetParentKey()
            "#,
        )
        .unwrap();

    assert_eq!(num_children, 1);
    assert_eq!(num_regions, 2);
    assert_eq!(first_child_key, "childFrame");
    assert_eq!(first_region_key, "regionTexture");
}

#[test]
fn test_set_attribute_fires_on_attribute_changed() {
    let env = WowLuaEnv::new().unwrap();
    let (name_ty, seen_name, value_ty, seen_value, stored_ty, stored_value): (
        String,
        String,
        String,
        String,
        String,
        String,
    ) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame")
            local seenName, seenValue

            frame:SetScript("OnAttributeChanged", function(_, name, value)
                seenName = name
                seenValue = value
            end)

            frame:SetAttribute("count", 7)

            return type(seenName), tostring(seenName), type(seenValue), tostring(seenValue),
                type(frame:GetAttribute("count")), tostring(frame:GetAttribute("count"))
            "#,
        )
        .unwrap();

    assert_eq!(name_ty, "string");
    assert_eq!(seen_name, "count");
    assert_eq!(
        value_ty, "number",
        "seen_name={seen_name} seen_value={seen_value} stored_ty={stored_ty} stored_value={stored_value}"
    );
    assert_eq!(seen_value, "7");
    assert_eq!(stored_ty, "number");
    assert_eq!(stored_value, "7");
}

#[test]
fn test_global_unpack_exists() {
    let env = WowLuaEnv::new().unwrap();
    let (global_ty, table_ty, first, second, third): (String, String, i64, i64, i64) = env
        .eval(
            r#"
            local values = {11, 22, 33}
            return type(unpack), type(table.unpack), table.unpack(values)
            "#,
        )
        .unwrap();

    assert_eq!(global_ty, "function");
    assert_eq!(table_ty, "function");
    assert_eq!((first, second, third), (11, 22, 33));
}
