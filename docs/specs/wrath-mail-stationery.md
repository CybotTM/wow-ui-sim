# Wrath stationery catalog

The Wrath scenario starts with an empty stationery catalog. `A_Admin.SetStationeryCatalog` replaces it with an ordered array of entries: `name` is a string, `texture` is a path or file ID, and `cost` is nonnegative copper or nil. Nil means free or owned for display purposes. These are simulator scenario inputs and defaults, not verified native inventory or prices.

On Wrath, `GetNumStationeries()` returns the catalog length and `GetStationeryInfo(index)` returns name, texture and cost for a one-based index; an out-of-range index returns no values. These tuple and invalid-index conventions are simulator assumptions, not verified native behavior. The pinned Wrath `FrameXML/MailFrame.lua:1012-1040` consumes the count and three fields, treats nil cost as free or owned, and displays each entry in the original popup. No other pinned profile's MailFrame uses these functions.

Stationery selection is not modeled by this contract.
