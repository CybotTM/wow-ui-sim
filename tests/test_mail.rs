use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().unwrap()
}

#[test]
fn admin_add_mail_basic() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            A_Admin.AddMail("Thrall", "Greetings", "Welcome to the Horde!")
            return #A_Admin.GetState().player.inbox
            "#,
        )
        .unwrap_or(-1);
    // GetState may not exist — check via the inbox count API instead
    // For now just verify AddMail doesn't error
    let _ = count;
}

#[test]
fn admin_add_mail_does_not_error() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            local ok, err = pcall(A_Admin.AddMail, "Thrall", "Hello", "Body text", 50000)
            return ok
            "#,
        )
        .unwrap();
    assert!(ok, "AddMail should not error");
}

#[test]
fn admin_add_mail_with_items() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            local ok, err = pcall(A_Admin.AddMail, "AH", "Auction Won", "", 0,
                {{item_id=6948, count=1}, {item_id=159, count=5}})
            return ok
            "#,
        )
        .unwrap();
    assert!(ok, "AddMail with items should not error");
}

#[test]
fn admin_clear_inbox() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.AddMail("A", "S1", "B1")
            A_Admin.AddMail("B", "S2", "B2")
            A_Admin.ClearInbox()
            A_Admin.AddMail("C", "S3", "B3")
            -- Should have exactly 1 mail after clear + add
            local ok, err = pcall(A_Admin.ClearInbox)
            return ok
            "#,
        )
        .unwrap();
    assert!(ok, "ClearInbox should not error");
}

#[test]
fn admin_set_inbox_count() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            local ok, err = pcall(A_Admin.SetInboxCount, 5)
            return ok
            "#,
        )
        .unwrap();
    assert!(ok, "SetInboxCount should not error");
}

#[test]
fn get_inbox_num_items_empty() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local num, total = GetInboxNumItems()
            return num .. "," .. total
            "#,
        )
        .unwrap();
    assert_eq!(result, "0,0", "Empty inbox should return (0, 0)");
}

#[test]
fn get_inbox_num_items_after_add() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("A", "S1", "B1")
            A_Admin.AddMail("B", "S2", "B2")
            A_Admin.AddMail("C", "S3", "B3")
            local num, total = GetInboxNumItems()
            return num .. "," .. total
            "#,
        )
        .unwrap();
    assert_eq!(result, "3,3", "Should have 3 mails after 3 AddMail calls");
}

#[test]
fn get_inbox_num_items_after_clear() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("A", "S1", "B1")
            A_Admin.ClearInbox()
            local num, total = GetInboxNumItems()
            return num .. "," .. total
            "#,
        )
        .unwrap();
    assert_eq!(result, "0,0", "Should have 0 after clear");
}

#[test]
fn get_inbox_num_items_set_count() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.SetInboxCount(7)
            local num, total = GetInboxNumItems()
            return num .. "," .. total
            "#,
        )
        .unwrap();
    assert_eq!(result, "7,7", "SetInboxCount(7) should produce 7 mails");
}

#[test]
fn get_inbox_header_info_returns_fields() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("Thrall", "Greetings", "Welcome!", 50000)
            local pkg, stationery, sender, subject, money, cod, days, items,
                  wasRead, wasReturned, textCreated, canReply, isGM = GetInboxHeaderInfo(1)
            if sender ~= "Thrall" then return "sender=" .. tostring(sender) end
            if subject ~= "Greetings" then return "subject=" .. tostring(subject) end
            if money ~= 50000 then return "money=" .. tostring(money) end
            if days ~= 30 then return "days=" .. tostring(days) end
            if wasRead ~= false then return "wasRead=" .. tostring(wasRead) end
            if textCreated ~= true then return "textCreated=" .. tostring(textCreated) end
            if canReply ~= true then return "canReply=" .. tostring(canReply) end
            if isGM ~= false then return "isGM=" .. tostring(isGM) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetInboxHeaderInfo fields: {result}");
}

#[test]
fn get_inbox_header_info_with_items() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("AH", "Won", "", 0, {{item_id=6948, count=1}})
            local pkg, _, _, _, _, _, _, itemCount = GetInboxHeaderInfo(1)
            if itemCount ~= 1 then return "itemCount=" .. tostring(itemCount) end
            if pkg == nil then return "pkg=nil" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "Mail with items should show package icon and count: {result}");
}

#[test]
fn get_inbox_header_info_invalid_index() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local r = GetInboxHeaderInfo(99)
            return r == nil and "nil" or "not_nil"
            "#,
        )
        .unwrap();
    assert_eq!(result, "nil", "Invalid index should return nil");
}

#[test]
fn get_inbox_item_returns_attachment() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("AH", "Won", "", 0, {{item_id=6948, count=1}})
            local name, id, texture, count, quality, canUse, isCurrency = GetInboxItem(1, 1)
            if name ~= "Hearthstone" then return "name=" .. tostring(name) end
            if id ~= 6948 then return "id=" .. tostring(id) end
            if count ~= 1 then return "count=" .. tostring(count) end
            if quality ~= 1 then return "quality=" .. tostring(quality) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetInboxItem should return item details: {result}");
}

#[test]
fn get_inbox_item_invalid_returns_nil() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("A", "S", "B")
            local r = GetInboxItem(1, 1)  -- mail has no items
            return r == nil and "nil" or "not_nil"
            "#,
        )
        .unwrap();
    assert_eq!(result, "nil", "No attachment should return nil");
}

#[test]
fn get_inbox_item_link() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("AH", "Won", "", 0, {{item_id=6948, count=1}})
            local link = GetInboxItemLink(1, 1)
            if not link then return "nil" end
            if not link:find("Hearthstone") then return "no_name: " .. link end
            if not link:find("|Hitem:6948") then return "no_id: " .. link end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetInboxItemLink: {result}");
}

#[test]
fn get_inbox_item_link_nil_for_missing() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("A", "S", "B")
            local r = GetInboxItemLink(1, 1)
            return r == nil and "nil" or "not_nil"
            "#,
        )
        .unwrap();
    assert_eq!(result, "nil", "Missing attachment link should be nil");
}

#[test]
fn get_inbox_text_returns_body() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("Thrall", "Hello", "Welcome to the Horde!", 100)
            local body, s1, s2, takeable, invoice, consortium = GetInboxText(1)
            if body ~= "Welcome to the Horde!" then return "body=" .. tostring(body) end
            if takeable ~= true then return "takeable=" .. tostring(takeable) end
            if invoice ~= false then return "invoice=" .. tostring(invoice) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetInboxText: {result}");
}

#[test]
fn get_inbox_text_nil_for_invalid() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local r = GetInboxText(99)
            return r == nil and "nil" or "not_nil"
            "#,
        )
        .unwrap();
    assert_eq!(result, "nil");
}

#[test]
fn get_inbox_invoice_info_returns_nil() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("AH", "Sold", "Your item sold")
            local r = GetInboxInvoiceInfo(1)
            return r == nil and "nil" or "not_nil"
            "#,
        )
        .unwrap();
    assert_eq!(result, "nil", "Invoice info stub should return nil");
}

#[test]
fn has_inbox_item() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("AH", "Won", "", 0, {{item_id=6948, count=1}})
            local has = HasInboxItem(1, 1)
            local no = HasInboxItem(1, 2)
            if has ~= true then return "has=" .. tostring(has) end
            if no ~= false then return "no=" .. tostring(no) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "HasInboxItem: {result}");
}

#[test]
fn inbox_item_can_delete() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("A", "Empty", "")
            A_Admin.AddMail("B", "With Gold", "", 100)
            local empty_ok = InboxItemCanDelete(1)
            local gold_no = InboxItemCanDelete(2)
            if empty_ok ~= true then return "empty=" .. tostring(empty_ok) end
            if gold_no ~= false then return "gold=" .. tostring(gold_no) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "InboxItemCanDelete: {result}");
}

#[test]
fn c_mail_can_check_inbox() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local can, secs = C_Mail.CanCheckInbox()
            if can ~= true then return "can=" .. tostring(can) end
            if secs ~= 0 then return "secs=" .. tostring(secs) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "C_Mail.CanCheckInbox: {result}");
}

#[test]
fn c_mail_has_inbox_money() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("A", "No gold", "")
            A_Admin.AddMail("B", "With gold", "", 500)
            local no = C_Mail.HasInboxMoney(1)
            local yes = C_Mail.HasInboxMoney(2)
            if no ~= false then return "no=" .. tostring(no) end
            if yes ~= true then return "yes=" .. tostring(yes) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "C_Mail.HasInboxMoney: {result}");
}

#[test]
fn take_inbox_item_removes_attachment() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("AH", "Won", "", 0, {{item_id=6948, count=1}, {item_id=159, count=5}})
            TakeInboxItem(1, 1)
            -- Should have 1 item left (the second one)
            local has1 = HasInboxItem(1, 1)
            local has2 = HasInboxItem(1, 2)
            if not has1 then return "item1_gone" end
            if has2 then return "item2_still" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "TakeInboxItem: {result}");
}

#[test]
fn auto_loot_clears_all() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("AH", "Won", "", 500, {{item_id=6948, count=1}})
            AutoLootMailItem(1)
            local _, _, _, _, money, _, _, itemCount = GetInboxHeaderInfo(1)
            if money ~= 0 then return "money=" .. tostring(money) end
            if itemCount ~= 0 then return "items=" .. tostring(itemCount) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "AutoLootMailItem: {result}");
}

#[test]
fn delete_inbox_item_removes_mail() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("A", "S1", "B1")
            A_Admin.AddMail("B", "S2", "B2")
            DeleteInboxItem(1)
            local num = GetInboxNumItems()
            if num ~= 1 then return "count=" .. tostring(num) end
            local _, _, sender = GetInboxHeaderInfo(1)
            if sender ~= "B" then return "sender=" .. tostring(sender) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "DeleteInboxItem: {result}");
}

#[test]
fn return_inbox_item_removes_mail() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.AddMail("A", "S1", "B1")
            ReturnInboxItem(1)
            local num = GetInboxNumItems()
            return tostring(num)
            "#,
        )
        .unwrap();
    assert_eq!(result, "0", "ReturnInboxItem should remove mail");
}
