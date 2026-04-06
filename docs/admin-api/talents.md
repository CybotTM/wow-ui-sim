# Specialization & Talents

### A_Admin.SetSpec(specIndex)

Sets the player's active specialization.

- **specIndex** `number` -- 1-based spec index within the player's class
- **Affects:** `GetSpecialization()`, `GetSpecializationInfo()`
- **Fires:** `PLAYER_SPECIALIZATION_CHANGED`
- **Example:**
```lua
-- For a Paladin: 1=Holy, 2=Protection, 3=Retribution
A_Admin.SetSpec(3)
print(GetSpecialization())  -- 3
local id, name = GetSpecializationInfo(3)
-- name="Retribution"
```

### A_Admin.SetTalentRank(nodeId, rank)

Sets the purchased rank on a talent tree node.

- **nodeId** `number` -- Trait node ID from the talent database
- **rank** `number` -- Number of ranks purchased (0 = unlearned)
- **Affects:** `C_Traits.GetNodeInfo(configID, nodeId).currentRank`
- **Fires:** `TRAIT_CONFIG_UPDATED`
- **Example:**
```lua
A_Admin.SetTalentRank(87800, 1)     -- learn a specific talent node
A_Admin.SetTalentRank(87800, 0)     -- unlearn it
```

### A_Admin.SetTalentSelection(nodeId, entryId)

Selects which entry is active for a choice node in the talent tree.

- **nodeId** `number` -- Trait node ID
- **entryId** `number` -- Entry ID of the selected choice
- **Affects:** `C_Traits.GetNodeInfo(configID, nodeId).activeEntry`
- **Fires:** `TRAIT_CONFIG_UPDATED`
- **Example:**
```lua
-- Select the right-side talent on a choice node
A_Admin.SetTalentSelection(88100, 88101)
```

### A_Admin.ResetTalents()

Clears all talent node ranks and selections, returning the tree to an unspent state.

- **Affects:** All `C_Traits.GetNodeInfo()` calls return rank 0 and no selection
- **Fires:** `TRAIT_CONFIG_UPDATED`
- **Example:**
```lua
A_Admin.ResetTalents()
```
