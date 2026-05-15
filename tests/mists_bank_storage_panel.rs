#![cfg(feature = "client-mists")]

use std::path::PathBuf;
use std::process::Command;

fn wow_sim_binary() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_wow-sim")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target")
                .join("debug")
                .join("wow-sim")
        })
}

#[test]
fn mists_bank_and_guild_bank_support_storage_flow() {
    let output = Command::new("timeout")
        .arg("90")
        .arg(wow_sim_binary())
        .args([
            "--no-addons",
            "--no-saved-vars",
            "--exec-lua",
            r#"
            A_Admin.ClearBags()
            A_Admin.AddBagItem(0, 1, 6948, 1)
            A_Admin.AddBagItem(-1, 1, 6948, 1)
            A_Admin.ClearGuildBank()
            A_Admin.AddGuildBankItem(1, 1, 6948, 1)

            FireEvent("BANKFRAME_OPENED")
            if not (BankFrame and BankFrame:IsShown()) then
                error("BankFrame did not open")
            end

            OpenAllBags()
            local hoveredBagItem = false
            for index = 1, 36 do
                local bagButton = _G["ContainerFrame1Item" .. index]
                if bagButton and bagButton:IsShown() then
                    ContainerFrameItemButton_OnEnter(bagButton)
                    if GameTooltip:IsShown() then
                        hoveredBagItem = true
                        break
                    end
                end
            end
            if not hoveredBagItem then
                error("bag item hover did not show GameTooltip")
            end

            local bankSlots, full = GetNumBankSlots()
            if bankSlots ~= 0 or full ~= false then
                error("bank slot state=" .. tostring(bankSlots) .. "/" .. tostring(full))
            end

            if C_Container.GetContainerNumSlots(BANK_CONTAINER) <= 0 then
                error("bank container has no slots")
            end

            PurchaseSlot()
            bankSlots, full = GetNumBankSlots()
            if bankSlots ~= 1 or full ~= false then
                error("bank slot state after purchase=" .. tostring(bankSlots) .. "/" .. tostring(full))
            end

            local bankInfo = C_Container.GetContainerItemInfo(BANK_CONTAINER, 1)
            if not bankInfo or bankInfo.itemID ~= 6948 then
                error("bank item missing")
            end

            local ok, reason = LoadAddOn("Blizzard_GuildBankUI")
            if ok == false then
                error("Blizzard_GuildBankUI failed to load: " .. tostring(reason))
            end

            if not C_GuildBank.IsGuildBankEnabled() then
                error("guild bank disabled")
            end

            FireEvent("GUILDBANKFRAME_OPENED")
            if not (GuildBankFrame and GuildBankFrame:IsShown()) then
                error("GuildBankFrame did not open")
            end

            if GetNumGuildBankTabs() < 1 then
                error("expected at least one guild bank tab")
            end

            local name, icon, isViewable = GetGuildBankTabInfo(1)
            if name ~= "General" or not isViewable then
                error("guild bank tab=" .. tostring(name) .. "/" .. tostring(isViewable))
            end

            local texture, itemCount, locked, isFiltered, quality = GetGuildBankItemInfo(1, 1)
            if itemCount ~= 1 or texture == nil or quality == nil then
                error("guild bank item=" .. tostring(texture) .. "/" .. tostring(itemCount) .. "/" .. tostring(quality))
            end

            if GetGuildBankMoney() <= 0 then
                error("guild bank money missing")
            end
            "#,
            "lua-errors",
        ])
        .output()
        .expect("failed to run wow-sim");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "wow-sim failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert_no_lua_errors(&stdout, &stderr);
}

fn assert_no_lua_errors(stdout: &str, stderr: &str) {
    assert!(
        !stdout.contains("Lua error") && !stderr.contains("Lua error"),
        "bank storage flow emitted Lua errors\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}
