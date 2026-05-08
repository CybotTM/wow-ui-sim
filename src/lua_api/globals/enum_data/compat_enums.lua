-- Hand-maintained compatibility enums for Blizzard UI surfaces that are
-- missing from the generated wowless globals snapshot.

if not Enum.CooldownLayoutStatus then
  Enum.CooldownLayoutStatus = {
    Success = 0,
    InvalidLayoutName = 1,
    TooManyLayouts = 2,
    AttemptToModifyDefaultLayoutWouldCreateTooManyLayouts = 3,
    TooManyAlerts = 4,
    InvalidOrderChange = 5,
    NoValidAlerts = 6,
  }
end

if not Enum.CDMLayoutMode then
  Enum.CDMLayoutMode = {
    AccessOnly = false,
    AllowCreate = true,
  }
end

if not Enum.CooldownLayoutAction then
  Enum.CooldownLayoutAction = {
    ChangeOrder = 0,
    ChangeCategory = 1,
    AddLayout = 2,
    AddAlert = 3,
  }
end

if not Enum.CooldownLayoutType then
  Enum.CooldownLayoutType = {
    Character = 1,
    Account = 2,
  }
end

if not Enum.CharacterCreateRaceMode then
  Enum.CharacterCreateRaceMode = {
    Normal = 0,
    Allied = 1,
  }
end

if not Enum.TransmogOutfitSlotOptionSheatheCategory then
  Enum.TransmogOutfitSlotOptionSheatheCategory = {
    Default = 0,
    Back = 1,
    Side = 2,
    Hide = 3,
  }
end

if not Enum.ExpansionLandingPageType then
  Enum.ExpansionLandingPageType = {
    None = 0,
    Dragonflight = 1,
    WarWithin = 2,
  }
end
