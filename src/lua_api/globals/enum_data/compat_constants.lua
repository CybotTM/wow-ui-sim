-- Hand-maintained compatibility constants for Blizzard UI surfaces that are
-- missing from the generated wowless globals snapshot.

Constants.TalentTierConstants.MAX_TALENT_TIERS = Constants.TalentTierConstants.MAX_TALENT_TIERS or 7
Constants.TalentConsts.NumTalentColumns = Constants.TalentConsts.NumTalentColumns or 3

MAX_TALENT_TIERS = MAX_TALENT_TIERS or Constants.TalentTierConstants.MAX_TALENT_TIERS
NUM_TALENT_COLUMNS = NUM_TALENT_COLUMNS or Constants.TalentConsts.NumTalentColumns
