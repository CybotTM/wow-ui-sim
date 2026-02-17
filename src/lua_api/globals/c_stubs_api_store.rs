//! C_AccountStore namespace stub.
//!
//! Returns minimal stub data so the Account Store UI code doesn't nil-error.
//! GetCategories returns one stub category (Creature type) and GetCategoryItems
//! returns an empty table, which is enough for the downstream code to function.

use mlua::{Lua, Result, Value};

/// Stub category ID used by C_AccountStore stubs.
const STUB_CATEGORY_ID: i32 = 1;

/// C_AccountStore namespace stub.
pub fn register_c_account_store(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("GetCategories", lua.create_function(|lua, _store_id: Value| {
        // Return one stub category so downstream code has a non-nil categories[1].
        let categories = lua.create_table()?;
        categories.raw_set(1, STUB_CATEGORY_ID)?;
        Ok(categories)
    })?)?;
    t.set("GetCategoryInfo", lua.create_function(|lua, _cat_id: Value| {
        let info = lua.create_table()?;
        // Enum.AccountStoreCategoryType.Creature = 1
        info.set("type", 1i32)?;
        info.set("name", "Store")?;
        info.set("icon", 0i32)?;
        Ok(Value::Table(info))
    })?)?;
    t.set("GetCategoryItems", lua.create_function(|lua, _cat_id: Value| {
        // Return empty table (not nil) so #self.categoryItems works.
        lua.create_table()
    })?)?;
    t.set("GetCurrencyIDForStore", lua.create_function(|_, _store_id: Value| Ok(Value::Nil))?)?;
    t.set("GetCurrencyInfo", lua.create_function(|_, _currency_id: Value| Ok(Value::Nil))?)?;
    t.set("GetCurrencyAvailable", lua.create_function(|_, _currency_id: Value| Ok(0i32))?)?;
    t.set("GetItemInfo", lua.create_function(|_, _item_id: Value| Ok(Value::Nil))?)?;
    t.set("GetStoreFrontState", lua.create_function(|_, _store_id: Value| Ok(0i32))?)?;
    t.set("RequestStoreFrontInfoUpdate", lua.create_function(|_, _store_id: Value| Ok(()))?)?;
    t.set("BeginPurchase", lua.create_function(|_, _item_id: Value| Ok(()))?)?;
    t.set("RefundItem", lua.create_function(|_, _item_id: Value| Ok(()))?)?;
    lua.globals().set("C_AccountStore", t)?;
    Ok(())
}
