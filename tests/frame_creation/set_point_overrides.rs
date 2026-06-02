use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn test_set_point_5arg_with_fixed_override() {
    let env = WowLuaEnv::new().unwrap();
    let (x, y): (f64, f64) = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "TestParent5Arg")
            parent:SetSize(800, 600)
            parent:SetPoint("CENTER")
            local child = CreateFrame("Frame", "TestChild5Arg", parent)
            child:SetSize(100, 50)

            -- Same fixed override
            child.SetPointBase = child.SetPoint
            child.SetPoint = function(self, point, relativeTo, relativePoint, offsetX, offsetY)
                if type(relativeTo) == "number" then
                    offsetX = relativeTo
                    offsetY = relativePoint
                    relativeTo = nil
                    relativePoint = nil
                end
                self:SetPointBase(point, relativeTo, relativePoint, offsetX, offsetY)
            end

            -- 5-arg form: SetPoint("TOPLEFT", parent, "TOPRIGHT", 5, -10)
            child:SetPoint("TOPLEFT", parent, "TOPRIGHT", 5, -10)
            local _, _, _, ox, oy = child:GetPoint(1)
            return ox, oy
        "#,
        )
        .unwrap();
    assert_eq!(
        (x, y),
        (5.0, -10.0),
        "5-arg SetPoint through fixed override preserves offsets"
    );
}
