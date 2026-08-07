# Patch 12.1 Broader Behavior Inventory
Non-FrameXML behavioral fidelity register. Family names group rows; status and exception approval remain item-specific.

## Content
- **Source:** `data/patch-api/sources/12.1-behaviors.json`
- **Source SHA-256:** `b26a9ae0939b770782c4a4fddf745b9692dc677205cbea9a58cb91d5c3e218eb`
- **Target:** PTR build `12.1.0`
- **Rows:** 54 changed behavioral boundaries — 0 implemented, 30 best-effort, 24 exception-requested, 0 untriaged
- **Candidate split:** 30 safe best-effort, 21 unsafe, 3 impossible; no exception is approved

| Symbol | Machine Status | Candidate | Family | Direction | Contract |
|---|---|---|---|---|---|
| `Patch12_1.UnitAura.AddonSecretError` | exception-requested  | unsafe  | UnitAura secrecy | changed | Addon-tainted UnitAura and aura access raise the retail secret-value error shape. |
| `Patch12_1.UnitAura.BlizzardSecretAccess` | exception-requested  | unsafe  | UnitAura secrecy | changed | Blizzard/internal callers receive the permitted secret-aura behavior distinct from addon-tainted callers. |
| `Patch12_1.UnitAura.SecretAuraData` | exception-requested  | unsafe  | UnitAura secrecy | changed | Fully secret AuraData fields remain inaccessible to addons while preserving the retail object shape. |
| `Patch12_1.UnitAura.SecretEventPayload` | exception-requested  | unsafe  | UnitAura secrecy | changed | Secret UNIT_AURA payload values preserve retail secrecy and tuple shape. |
| `Patch12_1.PrivateScriptObjects.PrivateIdentity` | exception-requested  | unsafe  | Private Script Objects | changed | Private or forbidden objects have identity distinct from their public frame view. |
| `Patch12_1.PrivateScriptObjects.InaccessiblePublicKeys` | exception-requested  | unsafe  | Private Script Objects | changed | Private keys remain inaccessible through the public object. |
| `Patch12_1.PrivateScriptObjects.ChildVisibility` | exception-requested  | unsafe  | Private Script Objects | changed | Public traversal cannot expose forbidden or private children. |
| `Patch12_1.PrivateScriptObjects.HookBoundary` | exception-requested  | unsafe  | Private Script Objects | changed | Hooks cannot cross private or forbidden partitions except through permitted delegates. |
| `Patch12_1.PrivateScriptObjects.ScriptStorage` | exception-requested  | unsafe  | Private Script Objects | changed | Script handlers stored in private partitions are not publicly readable or writable. |
| `Patch12_1.PrivateScriptObjects.SecureDelegateEnforcement` | exception-requested  | unsafe  | Private Script Objects | changed | Public delegates invoke permitted private behavior without exposing private receiver state. |
| `Patch12_1.ForbiddenAspects.UntrustedScriptExecution` | exception-requested  | unsafe  | Forbidden Aspects | changed | Operations requiring trusted script execution reject insecure callers. |
| `Patch12_1.ForbiddenAspects.UntrustedLayoutScriptExecution` | exception-requested  | unsafe  | Forbidden Aspects | changed | Layout-script operations reject insecure callers lacking the required aspect. |
| `Patch12_1.ForbiddenAspects.EventRegistrations` | exception-requested  | unsafe  | Forbidden Aspects | changed | Event registration operations enforce the EventRegistrations aspect restriction. |
| `Patch12_1.ForbiddenAspects.AlwaysPropagateInput` | exception-requested  | unsafe  | Forbidden Aspects | changed | Input propagation changes enforce the AlwaysPropagateInput aspect restriction. |
| `Patch12_1.ForbiddenAspects.ScriptedInput` | exception-requested  | unsafe  | Forbidden Aspects | changed | Scripted-input operations enforce the ScriptedInput aspect restriction. |
| `Patch12_1.ForbiddenAspects.QueryFocus` | exception-requested  | unsafe  | Forbidden Aspects | changed | Focus-query operations enforce the QueryFocus aspect restriction. |
| `Patch12_1.AuraContainer.CreationTypes` | best-effort  | best-effort  | AuraContainer | changed | AuraContainer, AuraButton, and ManagedAuraContainer can be created with compatible object types. |
| `Patch12_1.AuraContainer.Assignment` | best-effort  | best-effort  | AuraContainer | changed | Aura groups assign aura data to frames by auraInstanceID with compatible ownership; retained frames remain owned while removed entries are released. |
| `Patch12_1.AuraContainer.Filtering` | best-effort  | best-effort  | AuraContainer | changed | Aura groups apply HELPFUL/HARMFUL/PLAYER filtering to select compatible aura subsets. |
| `Patch12_1.AuraContainer.Sorting` | best-effort  | best-effort  | AuraContainer | changed | Aura groups honor configured comparator ordering; this does not claim the retail default comparator. |
| `Patch12_1.AuraContainer.PartitionPlacement` | best-effort  | best-effort  | AuraContainer | changed | Managed AuraContainer selects public-only, public-and-private, and edit-mode aura source partitions compatibly. |
| `Patch12_1.AuraContainer.ManagedButtonLifecycle` | best-effort  | best-effort  | AuraContainer | changed | Aura groups implement an acquire-release-reacquire lifecycle for managed frames as auraInstanceID entries change. |
| `Patch12_1.AuraContainer.TooltipBinding` | best-effort  | best-effort  | AuraContainer | changed | Aura buttons bind tooltip filter, aura-instance lookup, and leave-hide behavior. |
| `Patch12_1.AuraContainer.SecretVisibility` | exception-requested  | unsafe  | AuraContainer | changed | Secret aura values remain hidden while container and button structure stays usable. |
| `Patch12_1.TextureRadialProgress.Surface` | best-effort  | behavioral  | Texture radial progress | changed | A created Texture exposes the radial method family; no standalone constructor claim is made. Texture method availability and value storage are tested; exact retail clamping and visual rendering remain best-effort. |
| `Patch12_1.TextureRadialProgress.MethodDispatch` | best-effort  | behavioral  | Texture radial progress | changed | Radial progress methods dispatch on a Texture receiver. Texture method availability and value storage are tested; exact retail clamping and visual rendering remain best-effort. |
| `Patch12_1.TextureRadialProgress.StateBehavior` | best-effort  | behavioral  | Texture radial progress | changed | Texture-backed radial progress defaults, setters/getters, visual mode, and Clear reset are modeled. Texture method availability and value storage are tested; exact retail clamping and visual rendering remain best-effort. |
| `Patch12_1.DurationTextBinding.Lifetime` | best-effort  | best-effort  | DurationTextBinding | changed | A binding remains usable while retained by Lua references; exact Blizzard ownership and invalidation semantics remain unproven. |
| `Patch12_1.DurationTextBinding.StableIdentity` | best-effort  | best-effort  | DurationTextBinding | changed | Factory calls return distinct Lua tables with stable object identity and method lookup while referenced. |
| `Patch12_1.DurationTextBinding.RepresentationFidelity` | exception-requested  | unsafe  | DurationTextBinding | changed | The binding type, metatable, userdata representation, finalization, and ownership match Blizzard exactly. |
| `Patch12_1.DurationTextBinding.Formatter` | best-effort  | best-effort  | DurationTextBinding | changed | Duration formatting and interpolation use the documented compatible contract. |
| `Patch12_1.DurationTextBinding.ColorCurve` | best-effort  | best-effort  | DurationTextBinding | changed | Color-curve methods preserve compatible binding state. |
| `Patch12_1.DurationTextBinding.FontStringUpdate` | best-effort  | best-effort  | DurationTextBinding | changed | The binding updates a FontString through a documented compatible lifetime and update contract. |
| `Patch12_1.Service.Discord.OAuthState` | best-effort  | best-effort  | Service payloads | changed | Discord authorization and refresh state transitions expose compatible result payloads. |
| `Patch12_1.Service.Discord.GuildState` | best-effort  | best-effort  | Service payloads | changed | Discord guild link, unlink, and setting operations expose compatible state payloads. |
| `Patch12_1.Service.Discord.ServerChannelPayload` | best-effort  | best-effort  | Service payloads | changed | Discord server and channel lists, names, counts, and linkable-channel payloads are compatible. |
| `Patch12_1.Service.Housing.OwnedHouseState` | best-effort  | best-effort  | Service payloads | changed | Owned-house and plot state plus ResetHouse behavior follow the local compatibility model. |
| `Patch12_1.Service.Housing.BlueprintPayload` | best-effort  | best-effort  | Service payloads | changed | Housing blueprint export, import, and share-code payloads follow the local compatibility model. |
| `Patch12_1.Service.Housing.AvailabilityCodes` | best-effort  | best-effort  | Service payloads | changed | Housing availability and result codes plus import validation follow the local compatibility model. |
| `Patch12_1.Service.Housing.EditorDecorLayoutPayload` | best-effort  | best-effort  | Service payloads | changed | Housing editor, decor, room, budget, and floorplan payloads follow the local compatibility model. |
| `Patch12_1.Service.BattleNet.FriendInvitePayload` | best-effort  | best-effort  | Service payloads | changed | Verified Battle.net friend invite creation, deduplication, and info fields follow the local compatibility model. |
| `Patch12_1.Service.BattleNet.TitleFriendPayload` | best-effort  | best-effort  | Service payloads | changed | Battle.net title-friend custom names, tags, feature flags, and appear-offline state follow the local compatibility model. |
| `Patch12_1.Service.BattleNet.TitleFriendUnitInvite` | best-effort  | best-effort  | Service payloads | changed | Battle.net title-friend unit invite eligibility uses a documented deterministic compatibility result. |
| `Patch12_1.Service.EncounterJournal.DifficultyPayload` | best-effort  | best-effort  | Service payloads | changed | Encounter Journal base and valid difficulty IDs follow generated instance-data guesses. |
| `Patch12_1.Service.Cooldown.Payloads` | best-effort  | best-effort  | Service payloads | changed | Cooldown query structures, secret fields, and update payloads follow a documented compatibility contract. |
| `Patch12_1.Service.Pet.Payloads` | best-effort  | best-effort  | Service payloads | changed | Pet-related structures and state payloads follow a documented compatibility contract. |
| `Patch12_1.Service.LFG.Payloads` | best-effort  | best-effort  | Service payloads | changed | LFG service-result structures follow a documented compatibility contract. |
| `Patch12_1.Service.PlayerChoice.Payloads` | best-effort  | best-effort  | Service payloads | changed | Player-choice structures, options, and state payloads follow a documented compatibility contract. |
| `Patch12_1.Service.TieredEntrance.Payloads` | best-effort  | best-effort  | Service payloads | changed | C_DelvesUI TieredEntranceTierInfo rows expose tier, suggestedILvl, unlocked, tierDescription, modifierUIWidgetSetID, lockedReason, and rewards with id, quantity, rewardType, and context. Deterministic rows/rewards are modeled; live reward IDs, quantities, unlock timing, eligibility, and economics are not claimed. |
| `Patch12_1.Service.PrivateAura.Payloads` | exception-requested  | unsafe  | Service payloads | changed | Private-aura payloads preserve inaccessible and secret structural boundaries. |
| `Patch12_1.StrictRemoval.PreStartupVisibility` | exception-requested  | unsafe  | Strict removal timing | changed | Removed APIs are absent from addon-facing globals before Blizzard startup completes. |
| `Patch12_1.StrictRemoval.BlizzardLoadCompatibility` | best-effort  | best-effort  | Strict removal timing | changed | Pinned Blizzard UI loads while required removed symbols remain temporarily available. |
| `Patch12_1.StrictRemoval.PostStartupHiding` | best-effort  | best-effort  | Strict removal timing | changed | Removed symbols are hidden from addon-facing checks after startup. |
| `Patch12_1.StrictRemoval.WrapperTiming` | exception-requested  | unsafe  | Strict removal timing | changed | Deprecated wrappers remain available exactly until their required Blizzard callers finish. |

## Machine state totals

- implemented: 0
- best-effort: 28
- exception-requested: 0
- untriaged: 26

## Sources

- `data/patch-api/sources/12.1-behaviors.json` — normalized broader behavior boundaries and candidate disposition.
- [[patch-12-1-api-audit]] — broader audit context and family summaries.

## See Also

- [[patch-12-1-framexml-symbol-inventory]] — separate 432-row FrameXML symbol occurrence register.
- [[patch-api-audit-manifest]] — manifest validation and exception-approval rules.
