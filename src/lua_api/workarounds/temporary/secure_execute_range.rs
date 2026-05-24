//! Temporary secureexecuterange replacement for rilua's no-op stub.
//!
//! Rilua exposes the function today, but its native implementation does not
//! perform the Blizzard callback traversal yet. Keep the simulator-side Lua
//! behavior isolated until rilua owns the real implementation.

const SECURE_EXECUTE_RANGE_LUA: &str = r##"
-- Always install our Lua implementation to override rilua's no-op stub. It
-- must match Elune:
--   1. Visit numeric indices in order (ipairs) so ordered arrays -- e.g.
--      Menu element initializers built with table.insert(t, 1, x) -- run in
--      their intended sequence. Rilua's pairs() walks hash buckets, so it
--      would call the prepended factory init after initializers that read it.
--   2. Then visit any remaining hash-keyed entries -- CallbackRegistryMixin
--      keys callbacks by owner ID, so we must not stop at the array part.
--   3. Continue iterating even if the callback errors -- WoW routes errors
--      to the error handler but the loop keeps going.
function secureexecuterange(tbl, callback, ...)
  if type(tbl) ~= "table" or type(callback) ~= "function" then
    return
  end
  local extra = {...}
  local n = select("#", ...)
  local seen = {}
  for key, value in ipairs(tbl) do
    seen[key] = true
    pcall(callback, key, value, unpack(extra, 1, n))
  end
  for key, value in pairs(tbl) do
    if not seen[key] then
      pcall(callback, key, value, unpack(extra, 1, n))
    end
  end
end
"##;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(SECURE_EXECUTE_RANGE_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn visits_array_entries_before_hash_entries_and_continues_after_errors() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                local visited = {}
                local values = {[1] = "first", [2] = "second", owner = "hash"}
                secureexecuterange(values, function(key, value)
                  table.insert(visited, tostring(key) .. "=" .. value)
                  if key == 1 then error("keep going") end
                end)
                return table.concat(visited, ",")
                "#,
            )
            .expect("secureexecuterange probe should run");

        assert!(
            result.contains("1=first,2=second"),
            "array entries should be visited in numeric order before hash entries: {result}"
        );
        assert!(
            result.contains("owner=hash"),
            "hash entries should still be visited after array entries: {result}"
        );
    }
}
