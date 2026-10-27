# Wrath spellbook slot presentation

`FrameXML/FrameXML.toc` selects `SpellBookFrame.lua`. Its button update queries `GetSpellTexture(slot, bookType)` at line 447 and `GetSpellName(slot, bookType)` at line 495.

For Wrath only, these functions read the explicitly supplied per-book slot records from `A_Admin.SetWrathSpellBookSlot`. Numeric slots are one-based, from 1 through 1024. An unresolved slot returns one nil for texture and two nils for name; an empty supplied icon returns nil. Invalid slot or book inputs error. Names, subtext and icon filenames come from the supplied record. An empty rank is a scenario default; whether native rankless records return nil or an empty string is unknown.

The original button selection update calls `IsSelectedSpell(slot, bookType)` at `SpellBookFrame.lua:409`. Historical patch source `data/patch-api/source-cache/legacy-2026-10-09/2.3.0-wikitext.txt:34` lists `IsSelectedSpell(slot)`, and `4.0.1-wikitext.txt:68` lists its removal. This Wrath scenario returns 1 for explicitly selected records and nil otherwise, including unknown selection; this is a scenario default pending aso comparison. It uses the same slot and book validation as the other two queries.

The slot interpretation and return boundaries are based on the aso 3.3.5a comparison, not a Wrath Classic runtime capture. This does not model spell learning, tab membership, pet book population, tooltip behavior or native rank defaults. Other profiles retain their existing globals.
