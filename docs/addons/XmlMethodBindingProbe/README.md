# XmlMethodBindingProbe

Live-client probe for XML `<Scripts>` `method=` binding timing.

## Cases

- `siblingMutation`: mixin defines `OnHide`; inline `OnLoad` snapshots `GetScript`, `self.OnHide`, `rawget`, and `pairs`, replaces `self.OnHide`, directly calls `self:OnHide()`, then replaces the script with `SetScript("OnHide", hide3)` and calls `self:Hide()`. Expected: `load,hide2,hide3`.
- `inheritedMethod`: base virtual template defines `Foo=base` and `<OnLoad method="Foo"/>`; derived virtual template overrides `Foo=override`; concrete frame inherits derived template. Expected: `override`.
- `inheritedScriptText`: same inheritance shape, but base uses inline script text `self:Foo()` instead of `method="Foo"`. This records script-text control behavior. Expected if lookup happens when OnLoad runs: `override`.

## Install

```bash
scp -r docs/addons/XmlMethodBindingProbe desktop:'C:/World of Warcraft/_ptr_/Interface/AddOns/'
```

If testing retail instead of PTR, update `XmlMethodBindingProbe.toc` `## Interface` to the active retail interface and install under `_retail_`.

After enabling the addon in WoW, log in and run `/reload` or log out to flush SavedVariables.

## Pull results

```bash
scp desktop:'C:/World of Warcraft/_ptr_/WTF/Account/*/SavedVariables/XmlMethodBindingProbe.lua' /tmp/
```

The result is a Lua SavedVariables file containing `XmlMethodBindingProbeDB`.
