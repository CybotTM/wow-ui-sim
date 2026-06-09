# FrameIdentityProbe

Small retail-client probe for frame identity slot behavior.

It records whether assigning one frame's slot `0` token to another frame redirects method dispatch:

```lua
local a = CreateFrame("Frame")
local b = CreateFrame("Button", nil, UIParent, "SecureActionButtonTemplate")
a[0] = b[0]
a:IsProtected()
a:GetName()
```

The probe writes `FrameIdentityProbeDB` to account SavedVariables on `PLAYER_LOGIN`. Use `/reload` or logout once after it prints its captured message so WoW flushes the file.
