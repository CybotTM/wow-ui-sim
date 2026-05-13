use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const GUILD_ROSTER_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local originalRoster = C_GuildInfo.GuildRoster
local originalSelf = _G.self
local rosterCalls = 0

C_GuildInfo.GuildRoster = function()
  rosterCalls = rosterCalls + 1
end

AutoComplete_OnLoad(AutoCompleteBox)
expect(AutoCompleteBox:IsEventRegistered("GUILD_ROSTER_UPDATE"),
       "GUILD_ROSTER_UPDATE must be registered after OnLoad")

_G.self = AutoCompleteBox
AutoComplete_OnEvent(AutoCompleteBox, "GUILD_ROSTER_UPDATE", true)
_G.self = originalSelf

expect(rosterCalls == 1, "GuildRoster must be called when roster requests are allowed")
expect(not AutoCompleteBox:IsEventRegistered("GUILD_ROSTER_UPDATE"),
       "GUILD_ROSTER_UPDATE must unregister after requesting the roster")

AutoComplete_OnLoad(AutoCompleteBox)
A_Admin.SetGameRule("GuildsDisabled", true)

_G.self = AutoCompleteBox
AutoComplete_OnEvent(AutoCompleteBox, "GUILD_ROSTER_UPDATE", true)
_G.self = originalSelf

A_Admin.SetGameRule("GuildsDisabled", nil)
C_GuildInfo.GuildRoster = originalRoster

expect(rosterCalls == 1, "GuildsDisabled must suppress the roster request")
expect(AutoCompleteBox:IsEventRegistered("GUILD_ROSTER_UPDATE"),
       "GuildsDisabled short-circuit must leave GUILD_ROSTER_UPDATE registered")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_guild_roster_update_requests_once_then_unregisters() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete guild roster handling can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(GUILD_ROSTER_PROBE_LUA)
                    .expect("AutoComplete guild roster probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` guild roster mismatches:\n{failures}"
                );
            });
        });
    });
}
