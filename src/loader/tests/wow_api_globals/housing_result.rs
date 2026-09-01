//! Versioned startup publication and values for the HousingResult enum.

#![cfg(feature = "retail-12-0-0")]

use super::super::*;

#[cfg(feature = "retail-12-1-0")]
const RETAIL_12_1_HOUSING_RESULT_NAMES: &[&str] = &[
    "Success",
    "AccountBanned",
    "ActionLockedByCombat",
    "BlueprintCodeInvalid",
    "BlueprintDyeFailed",
    "BlueprintGenericExportError",
    "BlueprintGenericImportError",
    "BlueprintLocationInvalid",
    "BlueprintNameInvalid",
    "BlueprintNotFound",
    "BlueprintRequirementsUnmet",
    "BlueprintRoomPlacementRequired",
    "BlueprintTypeInvalid",
    "BlueprintTypeLocationInvalid",
    "BlueprintStorageLimit",
    "BlueprintVersionInvalid",
    "BoundsFailureChildren",
    "BoundsFailurePlot",
    "BoundsFailureRoom",
    "BoundToStartingArea",
    "CannotAfford",
    "CharterComplete",
    "CollisionInvalid",
    "DbError",
    "DecorCannotBeRedeemed",
    "DecorItemNotDestroyable",
    "DecorNotFound",
    "DecorNotFoundInStorage",
    "DuplicateCharterSignature",
    "FilterRejected",
    "FixtureCantDeleteDoor",
    "FixtureHookEmpty",
    "FixtureHookOccupied",
    "FixtureHouseTypeMismatch",
    "FixtureNotFound",
    "FixtureSizeMismatch",
    "FixtureTypeMismatch",
    "GenericFailure",
    "GuildMoreAccountsNeeded",
    "GuildMoreActivePlayersNeeded",
    "GuildNotLoaded",
    "HouseEditLockFailed",
    "HouseExteriorAlreadyThatSize",
    "HouseExteriorAlreadyThatType",
    "HouseExteriorRootNotFound",
    "HouseExteriorTypeNeighborhoodMismatch",
    "HouseExteriorTypeNotFound",
    "HouseExteriorTypeSizeMismatch",
    "HouseExteriorSizeNotAvailable",
    "HookNotChildOfFixture",
    "HouseNotFound",
    "IncorrectFaction",
    "InvalidDecorItem",
    "InvalidDistance",
    "InvalidExteriorDocument",
    "InvalidGuild",
    "InvalidHouse",
    "InvalidInstance",
    "InvalidInteraction",
    "InvalidInteriorDocument",
    "InvalidLightOverlap",
    "InvalidMap",
    "InvalidNeighborhoodName",
    "InvalidRoomLayout",
    "InsufficientRoomBudget",
    "LockedByOtherPlayer",
    "LockOperationFailed",
    "MaxPlacedDecorReached",
    "MaxPetDecorReached",
    "MaxPreviewDecorReached",
    "MaxStorageDecorReached",
    "MissingCoreFixture",
    "MissingDye",
    "MissingExpansionAccess",
    "MissingFactionMap",
    "MissingPrivateNeighborhoodInvite",
    "MoreHouseSlotsNeeded",
    "MoreSignaturesNeeded",
    "NeighborhoodNotFound",
    "NoNeighborhoodOwnershipRequests",
    "NotInDecorEditMode",
    "NotInFixtureEditMode",
    "NotInLayoutEditMode",
    "NotInsideHouse",
    "NotOnOwnedPlot",
    "OperationAborted",
    "OwnerNotInGuild",
    "PermissionDenied",
    "PlacementTargetInvalid",
    "PlayerNotFound",
    "PlayerNotInInstance",
    "PlotNotFound",
    "PlotNotVacant",
    "PlotReservationCooldown",
    "PlotReserved",
    "RoomNotFound",
    "RoomPlacementOutOfBounds",
    "RoomUpdateFailed",
    "RpcFailure",
    "ServiceNotAvailable",
    "StaticDataNotFound",
    "TimeoutLimit",
    "TimerunningNotAllowed",
    "TokenRequired",
    "TooManyRequests",
    "TransactionFailure",
    "UncollectedExteriorFixture",
    "UncollectedHouseType",
    "UncollectedRoom",
    "UncollectedRoomMaterial",
    "UncollectedRoomTheme",
    "UnlockOperationFailed",
];

#[cfg(not(feature = "retail-12-1-0"))]
const HOUSING_RESULT_VALUES: &[(&str, i64)] = &[
    ("BoundsFailureChildren", 2),
    ("BoundsFailurePlot", 3),
    ("BoundsFailureRoom", 4),
    ("CannotAfford", 5),
    ("CharterComplete", 6),
    ("CollisionInvalid", 7),
    ("DbError", 8),
    ("DecorCannotBeRedeemed", 9),
    ("DecorItemNotDestroyable", 10),
    ("DecorNotFound", 11),
    ("DecorNotFoundInStorage", 12),
    ("DuplicateCharterSignature", 13),
    ("FilterRejected", 14),
    ("FixtureCantDeleteDoor", 15),
    ("FixtureHookEmpty", 16),
    ("FixtureHookOccupied", 17),
    ("FixtureHouseTypeMismatch", 18),
    ("FixtureNotFound", 19),
    ("FixtureSizeMismatch", 20),
    ("FixtureTypeMismatch", 21),
    ("GenericFailure", 22),
    ("GuildMoreAccountsNeeded", 23),
    ("GuildMoreActivePlayersNeeded", 24),
    ("GuildNotLoaded", 25),
    ("HookNotChildOfFixture", 34),
    ("HouseEditLockFailed", 26),
    ("HouseExteriorAlreadyThatSize", 27),
    ("HouseExteriorAlreadyThatType", 28),
    ("HouseExteriorRootNotFound", 29),
    ("HouseExteriorSizeNotAvailable", 33),
    ("HouseExteriorTypeNeighborhoodMismatch", 30),
    ("HouseExteriorTypeNotFound", 31),
    ("HouseExteriorTypeSizeMismatch", 32),
    ("HouseNotFound", 35),
    ("IncorrectFaction", 36),
    ("InvalidDecorItem", 37),
    ("InvalidDistance", 38),
    ("InvalidGuild", 39),
    ("InvalidHouse", 40),
    ("InvalidInstance", 41),
    ("InvalidInteraction", 42),
    ("InvalidMap", 43),
    ("InvalidNeighborhoodName", 44),
    ("InvalidRoomLayout", 45),
    ("LockOperationFailed", 47),
    ("LockedByOtherPlayer", 46),
    ("MaxDecorReached", 48),
    ("MaxPreviewDecorReached", 49),
    ("MissingCoreFixture", 50),
    ("MissingDye", 51),
    ("MissingExpansionAccess", 52),
    ("MissingFactionMap", 53),
    ("MissingPrivateNeighborhoodInvite", 54),
    ("MoreHouseSlotsNeeded", 55),
    ("MoreSignaturesNeeded", 56),
    ("NeighborhoodNotFound", 57),
    ("NoNeighborhoodOwnershipRequests", 58),
    ("NotInDecorEditMode", 59),
    ("NotInFixtureEditMode", 60),
    ("NotInLayoutEditMode", 61),
    ("NotInsideHouse", 62),
    ("NotOnOwnedPlot", 63),
    ("OperationAborted", 64),
    ("OwnerNotInGuild", 65),
    ("PermissionDenied", 66),
    ("PlacementTargetInvalid", 67),
    ("PlayerNotFound", 68),
    ("PlayerNotInInstance", 69),
    ("PlotNotFound", 70),
    ("PlotNotVacant", 71),
    ("PlotReservationCooldown", 72),
    ("PlotReserved", 73),
    ("RoomNotFound", 74),
    ("RoomUpdateFailed", 75),
    ("RpcFailure", 76),
    ("ServiceNotAvailable", 77),
    ("StaticDataNotFound", 78),
    ("TimeoutLimit", 79),
    ("TimerunningNotAllowed", 80),
    ("TokenRequired", 81),
    ("TooManyRequests", 82),
    ("TransactionFailure", 83),
    ("UncollectedExteriorFixture", 84),
    ("UncollectedHouseType", 85),
    ("UncollectedRoom", 86),
    ("UncollectedRoomMaterial", 87),
    ("UncollectedRoomTheme", 88),
    ("UnlockOperationFailed", 89),
];

#[cfg(not(feature = "retail-12-1-0"))]
#[test]
fn test_patch_12_0_0_housing_result_values() {
    let env = WowLuaEnv::new().unwrap();
    let expected_lua = HOUSING_RESULT_VALUES
        .iter()
        .map(|(name, value)| format!("[{name:?}] = {value}"))
        .collect::<Vec<_>>()
        .join(",\n                ");
    let script = format!(
        r#"
            local namespace = Enum.HousingResult
            if type(namespace) ~= "table" then
                return "namespace:" .. type(namespace)
            end
            local expected = {{
                {expected_lua}
            }}
            for name, value in pairs(expected) do
                local actual = namespace[name]
                if type(actual) ~= "number" then
                    return name .. ":type=" .. type(actual)
                end
                if actual ~= value then
                    return name .. ":value=" .. tostring(actual)
                end
            end

            local metadata = Enum.HousingResultMeta
            local expected_metadata = {{
                MaxValue = 89,
                NumValues = 90,
            }}
            for name, value in pairs(expected_metadata) do
                local actual = metadata[name]
                if type(actual) ~= "number" or actual ~= value then
                    return "metadata." .. name .. "=" .. tostring(actual)
                end
            end
            return "ok"
        "#,
        expected_lua = expected_lua,
    );
    let result: String = env.eval(&script).unwrap();
    assert_eq!(
        result, "ok",
        "HousingResult did not match the 12.0.0 source register"
    );
}

#[cfg(feature = "retail-12-1-0")]
#[test]
fn test_patch_12_1_housing_result_values() {
    let env = WowLuaEnv::new().unwrap();
    let expected_lua = RETAIL_12_1_HOUSING_RESULT_NAMES
        .iter()
        .enumerate()
        .map(|(value, name)| format!("[{name:?}] = {value}"))
        .collect::<Vec<_>>()
        .join(",\n                ");
    let script = format!(
        r#"
            local namespace = Enum.HousingResult
            local metadata = Enum.HousingResultMeta
            if type(namespace) ~= "table" or type(metadata) ~= "table" then
                return "tables"
            end
            local expected = {{
                {expected_lua}
            }}
            for name, value in pairs(expected) do
                if namespace[name] ~= value then
                    return name .. ":value=" .. tostring(namespace[name])
                end
            end
            if table.count(namespace) ~= 112 then return "count" end
            if metadata.MinValue ~= 0 or metadata.MaxValue ~= 111 or metadata.NumValues ~= 112 then
                return "metadata"
            end
            return "ok"
        "#,
        expected_lua = expected_lua,
    );
    let result: String = env.eval(&script).unwrap();
    assert_eq!(
        result, "ok",
        "HousingResult did not match the 12.1 source register"
    );
}
