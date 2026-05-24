//! Temporary unit threat defaults.
//!
//! Threat state is not modeled yet beyond the Rust-owned `UnitThreatSituation`
//! nil default. Keep the remaining inert threat helpers explicit here until a
//! real threat table exists.

const UNIT_THREAT_DEFAULTS_LUA: &str = r#"
if UnitDetailedThreatSituation == nil then
  function UnitDetailedThreatSituation(_unit, _other_unit)
    return false, 0, 0, 0, 0
  end
end

if UnitThreatPercentageOfLead == nil then
  function UnitThreatPercentageOfLead(_unit, _other_unit)
    return 0
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(UNIT_THREAT_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_unit_threat_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                local isTanking, status, scaledPercent, rawPercent, threatValue =
                    UnitDetailedThreatSituation("player", "target")
                if isTanking ~= false or status ~= 0 or scaledPercent ~= 0 or rawPercent ~= 0 or threatValue ~= 0 then
                    return "detailed"
                end
                if UnitThreatPercentageOfLead("player", "target") ~= 0 then return "lead" end
                return "ok"
                "#,
            )
            .expect("unit threat defaults probe should run");

        assert_eq!(result, "ok");
    }
}
