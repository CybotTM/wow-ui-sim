pub(super) fn wrap_headless_exec_lua(code: &str) -> String {
    format!(
        r#"
local __wow_sim_exec_lua_old_print = print
if type(A_Print) == "function" then
    print = A_Print
end
local __wow_sim_exec_lua_ok, __wow_sim_exec_lua_err = xpcall(function()
{code}
end, debug.traceback)
print = __wow_sim_exec_lua_old_print
if not __wow_sim_exec_lua_ok then
    error(__wow_sim_exec_lua_err, 0)
end
"#
    )
}

#[cfg(test)]
mod tests {
    use super::wrap_headless_exec_lua;

    #[test]
    fn headless_exec_lua_wrapper_routes_print_through_a_print() {
        let wrapped = wrap_headless_exec_lua("print('marker')");

        assert!(wrapped.contains("local __wow_sim_exec_lua_old_print = print"));
        assert!(wrapped.contains("print = A_Print"));
        assert!(wrapped.contains("print('marker')"));
        assert!(wrapped.contains("print = __wow_sim_exec_lua_old_print"));
    }

    #[test]
    fn headless_exec_lua_wrapper_preserves_errors() {
        let wrapped = wrap_headless_exec_lua("error('marker')");

        assert!(wrapped.contains("xpcall(function()"));
        assert!(wrapped.contains("debug.traceback"));
        assert!(wrapped.contains("error(__wow_sim_exec_lua_err, 0)"));
    }
}
