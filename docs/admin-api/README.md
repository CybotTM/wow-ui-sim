# Admin API (A_Admin)

The `A_Admin` namespace provides administrative functions for controlling simulator state from Lua. These functions are unique to the simulator and do not exist in the real WoW client.

**Implementation:** `src/lua_api/globals/admin_api.rs`, `admin_api_world.rs`, `admin_combat.rs`, `admin_encounter.rs`

## Sections

- [Player Identity](player-identity.md) — SetPlayerName, SetPlayerClass, SetPlayerRace, SetPlayerLevel, SetPlayerSex
- [Combat](combat.md) — SetInCombat, SetCasting, StopCasting, SetGCD, SetSpellCooldown
- [Health & Power](health-power.md) — SetPlayerHealth, SetPlayerPower, SetTargetHealth
- [Targeting](targeting.md) — SetTarget, ClearTarget, SetFocus, ClearFocus
- [Party](party.md) — SetPartySize, SetPartyMember, SetPartyMemberHealth, KillPartyMember, ResPartyMember, SetRotDamage
- [Movement](movement.md) — SetMoving, SetMounted, SetFlying, SetFalling, SetSwimming
- [Talents](talents.md) — SetSpec, SetTalentRank, SetTalentSelection, ResetTalents
- [Buffs](buffs.md) — AddBuff, RemoveBuff, ClearBuffs
- [Zone](zone.md) — SetZone, SetSubZone, SetInstanceInfo, SetInInstance
- [Economy](economy.md) — SetMoney, SetItemLevel, AddBagItem, RemoveBagItem
- [Collections](collections.md) — AddTransmog, RemoveTransmog, CollectMount, UncollectMount, CollectPet, UncollectPet, CollectToy, UncollectToy, CollectCampsite, UncollectCampsite, SetAchievementEarned, EarnAchievement
- [PvP](pvp.md) — SetPvPEnabled, SetHonorLevel
- [Guild](guild.md) — SetGuildInfo, ClearGuild, JoinGuild, LeaveGuild
- [Mail](mail.md) — AddMail, ClearInbox, SetInboxCount
- [Premade Groups](premade-groups.md) — AddPremadeListing, ClearPremadeListings, UpdatePremadeListing
- [Events](events.md) — FireEvent
- [Examples](examples.md) — Usage examples (raid healing, combat testing, transmog)
