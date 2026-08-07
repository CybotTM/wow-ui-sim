# Patch 12.1 Broader Behavior Inventory
Non-FrameXML behavioral fidelity register. Family names group rows; status and exception approval remain item-specific.

## Content
- **Source:** `data/patch-api/sources/12.1-behaviors.json`
- **Source SHA-256:** `51ff0c88bd0064c9e8752be60a5d906f89c721acac143b905fdb0013574e6e93`
- **Target:** PTR build `12.1.0`
- **Rows:** 53 changed behavioral boundaries — 0 implemented, 0 best-effort, 0 exception-requested, 53 untriaged
- **Candidate split:** 30 safe best-effort, 20 unsafe, 3 impossible

| Symbol | Machine Status | Candidate | Family | Direction | Contract |
|---|---|---|---|---|---|
| `Patch12_1.UnitAura.AddonSecretError` | untriaged | unsafe | UnitAura secrecy | changed | Addon-tainted UnitAura and aura access raise the retail secret-value error shape. |
| `Patch12_1.UnitAura.BlizzardSecretAccess` | untriaged | unsafe | UnitAura secrecy | changed | Blizzard/internal callers receive the permitted secret-aura behavior distinct from addon-tainted callers. |
| `Patch12_1.UnitAura.SecretAuraData` | untriaged | unsafe | UnitAura secrecy | changed | Fully secret AuraData fields remain inaccessible to addons while preserving the retail object shape. |
| `Patch12_1.UnitAura.SecretEventPayload` | untriaged | unsafe | UnitAura secrecy | changed | Secret UNIT_AURA payload values preserve retail secrecy and tuple shape. |
| `Patch12_1.PrivateScriptObjects.PrivateIdentity` | untriaged | unsafe | Private Script Objects | changed | Private or forbidden objects have identity distinct from their public frame view. |
| `Patch12_1.PrivateScriptObjects.InaccessiblePublicKeys` | untriaged | unsafe | Private Script Objects | changed | Private keys remain inaccessible through the public object. |
| `Patch12_1.PrivateScriptObjects.ChildVisibility` | untriaged | unsafe | Private Script Objects | changed | Public traversal cannot expose forbidden or private children. |
| `Patch12_1.PrivateScriptObjects.HookBoundary` | untriaged | unsafe | Private Script Objects | changed | Hooks cannot cross private or forbidden partitions except through permitted delegates. |
| `Patch12_1.PrivateScriptObjects.ScriptStorage` | untriaged | unsafe | Private Script Objects | changed | Script handlers stored in private partitions are not publicly readable or writable. |
| `Patch12_1.PrivateScriptObjects.SecureDelegateEnforcement` | untriaged | unsafe | Private Script Objects | changed | Public delegates invoke permitted private behavior without exposing private receiver state. |
| `Patch12_1.ForbiddenAspects.UntrustedScriptExecution` | untriaged | unsafe | Forbidden Aspects | changed | Operations requiring trusted script execution reject insecure callers. |
| `Patch12_1.ForbiddenAspects.UntrustedLayoutScriptExecution` | untriaged | unsafe | Forbidden Aspects | changed | Layout-script operations reject insecure callers lacking the required aspect. |
| `Patch12_1.ForbiddenAspects.EventRegistrations` | untriaged | unsafe | Forbidden Aspects | changed | Event registration operations enforce the EventRegistrations aspect restriction. |
| `Patch12_1.ForbiddenAspects.AlwaysPropagateInput` | untriaged | unsafe | Forbidden Aspects | changed | Input propagation changes enforce the AlwaysPropagateInput aspect restriction. |
| `Patch12_1.ForbiddenAspects.ScriptedInput` | untriaged | unsafe | Forbidden Aspects | changed | Scripted-input operations enforce the ScriptedInput aspect restriction. |
| `Patch12_1.ForbiddenAspects.QueryFocus` | untriaged | unsafe | Forbidden Aspects | changed | Focus-query operations enforce the QueryFocus aspect restriction. |
| `Patch12_1.AuraContainer.CreationTypes` | untriaged | best-effort | AuraContainer | changed | AuraContainer, AuraButton, and ManagedAuraContainer can be created with compatible object types. |
| `Patch12_1.AuraContainer.Assignment` | untriaged | best-effort | AuraContainer | changed | Aura data can be assigned to container and button objects with compatible ownership. |
| `Patch12_1.AuraContainer.Filtering` | untriaged | best-effort | AuraContainer | changed | Helpful, harmful, and player filters select a compatible aura subset. |
| `Patch12_1.AuraContainer.Sorting` | untriaged | best-effort | AuraContainer | changed | Aura entries use a documented compatible ordering. |
| `Patch12_1.AuraContainer.PartitionPlacement` | untriaged | best-effort | AuraContainer | changed | Public and private aura data lands in the compatible object partition. |
| `Patch12_1.AuraContainer.ManagedButtonLifecycle` | untriaged | best-effort | AuraContainer | changed | Managed containers create, reuse, and release aura buttons compatibly. |
| `Patch12_1.AuraContainer.TooltipBinding` | untriaged | best-effort | AuraContainer | changed | Aura buttons bind compatible aura tooltip data. |
| `Patch12_1.AuraContainer.SecretVisibility` | untriaged | unsafe | AuraContainer | changed | Secret aura values remain hidden while container and button structure stays usable. |
| `Patch12_1.RadialProgress.Constructor` | untriaged | impossible | RadialProgress | changed | A standalone RadialProgress script object can be constructed through an audited retail API. |
| `Patch12_1.RadialProgress.MethodDispatch` | untriaged | impossible | RadialProgress | changed | Standalone RadialProgress methods dispatch through the expected receiver and metatable. |
| `Patch12_1.RadialProgress.StateBehavior` | untriaged | impossible | RadialProgress | changed | A standalone RadialProgress object stores and updates radial progress state. |
| `Patch12_1.DurationTextBinding.Lifetime` | untriaged | best-effort | DurationTextBinding | changed | DurationTextBinding lifetime and ownership follow Blizzard object semantics. |
| `Patch12_1.DurationTextBinding.MetatableIdentity` | untriaged | best-effort | DurationTextBinding | changed | The returned binding has compatible table or userdata identity and method lookup behavior. |
| `Patch12_1.DurationTextBinding.Formatter` | untriaged | best-effort | DurationTextBinding | changed | Duration formatting and interpolation use the documented compatible contract. |
| `Patch12_1.DurationTextBinding.ColorCurve` | untriaged | best-effort | DurationTextBinding | changed | Color-curve methods preserve compatible binding state. |
| `Patch12_1.DurationTextBinding.FontStringUpdate` | untriaged | best-effort | DurationTextBinding | changed | The binding updates a FontString through a documented compatible lifetime and update contract. |
| `Patch12_1.Service.Discord.OAuthState` | untriaged | best-effort | Service payloads | changed | Discord authorization and refresh state transitions expose compatible result payloads. |
| `Patch12_1.Service.Discord.GuildState` | untriaged | best-effort | Service payloads | changed | Discord guild link, unlink, and setting operations expose compatible state payloads. |
| `Patch12_1.Service.Discord.ServerChannelPayload` | untriaged | best-effort | Service payloads | changed | Discord server and channel lists, names, counts, and linkable-channel payloads are compatible. |
| `Patch12_1.Service.Housing.OwnedHouseState` | untriaged | best-effort | Service payloads | changed | Owned-house and plot state plus ResetHouse behavior follow the local compatibility model. |
| `Patch12_1.Service.Housing.BlueprintPayload` | untriaged | best-effort | Service payloads | changed | Housing blueprint export, import, and share-code payloads follow the local compatibility model. |
| `Patch12_1.Service.Housing.AvailabilityCodes` | untriaged | best-effort | Service payloads | changed | Housing availability and result codes plus import validation follow the local compatibility model. |
| `Patch12_1.Service.Housing.EditorDecorLayoutPayload` | untriaged | best-effort | Service payloads | changed | Housing editor, decor, room, budget, and floorplan payloads follow the local compatibility model. |
| `Patch12_1.Service.BattleNet.FriendInvitePayload` | untriaged | best-effort | Service payloads | changed | Verified Battle.net friend invite creation, deduplication, and info fields follow the local compatibility model. |
| `Patch12_1.Service.BattleNet.TitleFriendPayload` | untriaged | best-effort | Service payloads | changed | Battle.net title-friend custom names, tags, feature flags, and appear-offline state follow the local compatibility model. |
| `Patch12_1.Service.BattleNet.TitleFriendUnitInvite` | untriaged | best-effort | Service payloads | changed | Battle.net title-friend unit invite eligibility uses a documented deterministic compatibility result. |
| `Patch12_1.Service.EncounterJournal.DifficultyPayload` | untriaged | best-effort | Service payloads | changed | Encounter Journal base and valid difficulty IDs follow generated instance-data guesses. |
| `Patch12_1.Service.Cooldown.Payloads` | untriaged | best-effort | Service payloads | changed | Cooldown query structures, secret fields, and update payloads follow a documented compatibility contract. |
| `Patch12_1.Service.Pet.Payloads` | untriaged | best-effort | Service payloads | changed | Pet-related structures and state payloads follow a documented compatibility contract. |
| `Patch12_1.Service.LFG.Payloads` | untriaged | best-effort | Service payloads | changed | LFG service-result structures follow a documented compatibility contract. |
| `Patch12_1.Service.PlayerChoice.Payloads` | untriaged | best-effort | Service payloads | changed | Player-choice structures, options, and state payloads follow a documented compatibility contract. |
| `Patch12_1.Service.TieredAura.Payloads` | untriaged | best-effort | Service payloads | changed | Tiered-aura structures and tier fields follow a documented compatibility contract. |
| `Patch12_1.Service.PrivateAura.Payloads` | untriaged | unsafe | Service payloads | changed | Private-aura payloads preserve inaccessible and secret structural boundaries. |
| `Patch12_1.StrictRemoval.PreStartupVisibility` | untriaged | unsafe | Strict removal timing | changed | Removed APIs are absent from addon-facing globals before Blizzard startup completes. |
| `Patch12_1.StrictRemoval.BlizzardLoadCompatibility` | untriaged | best-effort | Strict removal timing | changed | Pinned Blizzard UI loads while required removed symbols remain temporarily available. |
| `Patch12_1.StrictRemoval.PostStartupHiding` | untriaged | best-effort | Strict removal timing | changed | Removed symbols are hidden from addon-facing checks after startup. |
| `Patch12_1.StrictRemoval.WrapperTiming` | untriaged | unsafe | Strict removal timing | changed | Deprecated wrappers remain available exactly until their required Blizzard callers finish. |

## Machine state totals

- implemented: 0
- best-effort: 0
- exception-requested: 0
- untriaged: 53

## Sources

- `data/patch-api/sources/12.1-behaviors.json` — normalized broader behavior boundaries and candidate disposition.
- [[patch-12-1-api-audit]] — broader audit context and family summaries.

## See Also

- [[patch-12-1-framexml-symbol-inventory]] — separate 432-row FrameXML symbol occurrence register.
- [[patch-api-audit-manifest]] — manifest validation and exception-approval rules.
