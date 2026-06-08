# Create and Install a WoW Addon

This is the reliable path for creating a small addon and putting it where World of Warcraft actually loads it.

## Critical Rules

- Desktop upload is only staging. WoW does not load addons from the Desktop.
- The addon must be installed under the active client folder, for example:

```text
C:\World of Warcraft\_retail_\Interface\AddOns\MyAddon\
```

- On the desktop machine, the current retail install has been found at:

```text
C:\World of Warcraft\_retail_\
```

- The folder name and `.toc` basename must match:

```text
MyAddon\MyAddon.toc
```

- Do not create a nested folder like:

```text
MyAddon\MyAddon\MyAddon.toc
```

## Interface Number

The TOC `## Interface` must match the client. If it is stale, WoW marks the addon incompatible or out of date.

Current observed retail desktop client:

```text
Version: 12.0.5.67823
Interface: 120005
Install: C:\World of Warcraft\_retail_\
```

Use this in the TOC for that client:

```toc
## Interface: 120005
```

In game, this prints the current interface number:

```lua
/run print(select(4, GetBuildInfo()))
```

When checking from the desktop over SSH, prefer existing installed addon TOCs or `.build.info` rather than guessing. Installed addons often show the active retail interface:

```powershell
Get-ChildItem 'C:\World of Warcraft\_retail_\Interface\AddOns' -Filter *.toc -Recurse |
  Select-String -Pattern '^## Interface:' |
  ForEach-Object { $_.Line.Trim() } |
  Group-Object |
  Sort-Object Count -Descending
```

## Minimal Addon

Layout:

```text
MyAddon\
  MyAddon.toc
  MyAddon.lua
```

`MyAddon.toc`:

```toc
## Interface: 120005
## Title: My Addon
## Notes: Minimal test addon
## Author: Alessio
## Version: 0.1.0

MyAddon.lua
```

`MyAddon.lua`:

```lua
local addonName = ...

local frame = CreateFrame("Frame")
frame:RegisterEvent("PLAYER_LOGIN")
frame:SetScript("OnEvent", function()
    print(addonName .. " loaded")
end)
```

## SavedVariables

To persist data, declare the database in the TOC:

```toc
## SavedVariables: MyAddonDB
```

Initialize it on `ADDON_LOADED` and write to it during runtime:

```lua
local addonName = ...

local frame = CreateFrame("Frame")
frame:RegisterEvent("ADDON_LOADED")
frame:RegisterEvent("PLAYER_LOGOUT")

frame:SetScript("OnEvent", function(_, event, loadedAddon)
    if event == "ADDON_LOADED" and loadedAddon == addonName then
        MyAddonDB = MyAddonDB or {}
    elseif event == "PLAYER_LOGOUT" then
        MyAddonDB.lastLogout = time()
    end
end)
```

WoW writes account-wide SavedVariables on logout or `/reload`:

```text
C:\World of Warcraft\_retail_\WTF\Account\<ACCOUNT>\SavedVariables\MyAddon.lua
```

If the SavedVariables file does not appear:

- confirm the addon is installed in the real WoW AddOns directory;
- confirm it is enabled in the AddOns list;
- confirm the TOC interface is compatible or "Load out of date AddOns" is enabled;
- run `/reload` or log out after the addon has loaded.

## XML Addons

If XML creates globals used by Lua, list XML before Lua:

```toc
## Interface: 120005
## Title: My Addon

MyAddon.xml
MyAddon.lua
```

Example XML:

```xml
<Ui xmlns="http://www.blizzard.com/wow/ui/">
    <Frame name="MyAddonFrame" parent="UIParent" hidden="true">
        <Size x="200" y="100"/>
        <Anchors>
            <Anchor point="CENTER"/>
        </Anchors>
    </Frame>
</Ui>
```

## Install on Desktop

Known desktop SSH alias:

```text
desktop -> 192.168.2.185
```

Correct live retail install path on desktop:

```text
C:\World of Warcraft\_retail_\Interface\AddOns\
```

Install from this repo:

```bash
scp -r docs/addons/MyAddon desktop:'C:/World of Warcraft/_retail_/Interface/AddOns/'
```

Then in WoW:

1. `/reload` or restart the client.
2. Open AddOns.
3. Enable the addon.
4. Log in.
5. For SavedVariables, `/reload` or log out once after the addon runs.

## Package for Sharing

Zip the addon folder itself:

```bash
zip -r MyAddon.zip MyAddon
```

The zip should contain:

```text
MyAddon/MyAddon.toc
MyAddon/MyAddon.lua
```

## Common Failures

- Incompatible: stale `## Interface`.
- Not visible in AddOns list: copied to Desktop or wrong folder, not the real `Interface\AddOns`.
- Loads but does nothing: Lua file missing from TOC or XML/Lua order is wrong.
- SavedVariables missing: addon never loaded, not enabled, or no `/reload`/logout after writing.
- SSH/PowerShell path bugs: paths with spaces like `C:\World of Warcraft` need careful quoting; `scp` with `desktop:'C:/World of Warcraft/_retail_/Interface/AddOns/'` worked reliably.
