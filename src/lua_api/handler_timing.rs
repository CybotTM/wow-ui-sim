use std::borrow::Cow;
use std::sync::OnceLock;
use std::time::Duration;

use rilua::Val;
use rilua::vm::closure::Closure;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;

const BUILTIN_ADDON_NAME: &str = "__BuiltIn";
const HANDLER_TIMING_ENV: &str = "WOW_SIM_LOG_HANDLER_TIMINGS";

pub(crate) fn is_enabled() -> bool {
    min_duration_ms().is_some()
}

pub(crate) fn should_log(duration: Duration) -> bool {
    let Some(min_duration_ms) = min_duration_ms() else {
        return false;
    };
    duration.as_secs_f64() * 1000.0 >= min_duration_ms
}

pub(crate) fn log_with_source(
    addon_name: Option<&str>,
    handler_name: &str,
    frame_name: Option<&str>,
    widget_id: u64,
    duration: Duration,
    source: Option<&str>,
) {
    if !should_log(duration) {
        return;
    }
    eprintln!(
        "{}",
        format_log(
            addon_name,
            handler_name,
            frame_name,
            widget_id,
            duration,
            source,
        )
    );
}

pub(crate) fn lua_closure_source_label(
    state: &LuaState,
    func_ref: GcRef<Closure>,
) -> Option<String> {
    let closure = state.gc.closures.get(func_ref)?;
    let lua_closure = closure.as_lua()?;
    let proto = lua_closure.proto.as_ref();
    if proto.source.is_empty() {
        return None;
    }

    Some(format!("{}:{}", proto.source, proto.line_defined))
}

/// Source identities captured once when the native timer stores its callback.
#[derive(Clone)]
pub(crate) struct TimerCallbackSource {
    source: Option<String>,
    callback_source: Option<String>,
}

impl TimerCallbackSource {
    pub(crate) fn capture(state: &mut LuaState, callback: Val) -> Self {
        let originals = super::methods::registry_get(state, "__wow_timer_original_callbacks");
        let association = match originals {
            Val::Table(table) => state
                .gc
                .tables
                .get(table)
                .map(|table| table.get(callback, &state.gc.string_arena)),
            _ => None,
        };
        let callback_source = match association {
            // Wrappers are associated before registration whenever diagnostics
            // are enabled. An absent key denotes a direct callback (TimedSignalMap).
            Some(Val::Nil) => function_source_label(state, callback),
            Some(original @ Val::Function(_)) => function_source_label(state, original),
            // Corrupt association/table state must not masquerade as direct registration.
            _ => Some("<invalid-association>".to_owned()),
        };
        Self {
            source: function_source_label(state, callback),
            callback_source,
        }
    }

    pub(crate) fn log_entry(&self, addon_name: Option<&str>, timer_id: u64) {
        let addon = addon_name.unwrap_or(BUILTIN_ADDON_NAME);
        let mut line = format!("[handler-entry] addon={addon} handler=C_Timer frame=#0");
        if let Some(source) = self.source.as_deref() {
            line.push_str(" source=");
            line.push_str(source);
        }
        line.push_str(&self.format_timer_fields(timer_id));
        eprintln!("{line}");
    }

    pub(crate) fn log_return(
        &self,
        addon_name: Option<&str>,
        timer_id: u64,
        duration: Duration,
        failed: bool,
    ) {
        if !should_log(duration) {
            return;
        }
        let mut line = format_log(
            addon_name,
            "C_Timer",
            None,
            0,
            duration,
            self.source.as_deref(),
        );
        line.push_str(&self.format_timer_fields(timer_id));
        let outcome = if failed { "error" } else { "ok" };
        line.push_str(&format!(" outcome={outcome}"));
        eprintln!("{line}");
    }

    fn format_timer_fields(&self, timer_id: u64) -> String {
        let callback_source = self.callback_source.as_deref().unwrap_or("<unknown>");
        format!(" timer_id={timer_id} callback_source={callback_source}")
    }
}

fn function_source_label(state: &LuaState, callback: Val) -> Option<String> {
    let Val::Function(func_ref) = callback else {
        return None;
    };
    lua_closure_source_label(state, func_ref)
}

fn min_duration_ms() -> Option<f64> {
    static MIN_DURATION_MS: OnceLock<Option<f64>> = OnceLock::new();
    *MIN_DURATION_MS
        .get_or_init(|| parse_min_duration_ms(std::env::var(HANDLER_TIMING_ENV).ok().as_deref()))
}

fn parse_min_duration_ms(raw: Option<&str>) -> Option<f64> {
    let raw = raw?;
    let value = raw.trim();
    if value.is_empty() {
        return Some(0.0);
    }
    Some(value.parse::<f64>().ok().unwrap_or(0.0).max(0.0))
}

fn format_log(
    addon_name: Option<&str>,
    handler_name: &str,
    frame_name: Option<&str>,
    widget_id: u64,
    duration: Duration,
    source: Option<&str>,
) -> String {
    let addon = addon_name.unwrap_or(BUILTIN_ADDON_NAME);
    let frame = frame_label(frame_name, widget_id);
    let mut line = format!(
        "[handler] addon={addon} handler={handler_name} frame={frame} duration_ms={:.3}",
        duration.as_secs_f64() * 1000.0
    );
    if let Some(source) = source.filter(|source| !source.is_empty()) {
        line.push_str(" source=");
        line.push_str(source);
    }
    line
}

fn frame_label<'a>(frame_name: Option<&'a str>, widget_id: u64) -> Cow<'a, str> {
    match frame_name {
        Some(name) if !name.is_empty() => Cow::Borrowed(name),
        _ => Cow::Owned(format!("#{widget_id}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rilua::LuaApiMut;

    #[test]
    fn missing_source_and_invalid_registry_states_are_explicit() {
        let env = crate::lua_api::WowLuaEnv::new().unwrap();
        for (registry, callback, expected) in [
            ("{}", "collectgarbage", "<unknown>"),
            ("nil", "function() return 37 end", "<invalid-association>"),
            ("42", "function() return 37 end", "<invalid-association>"),
        ] {
            let callback = env
                .eval::<Val>(&format!(
                    "debug.getregistry().__wow_timer_original_callbacks = {registry}; \
                     CallbackUnderTest = {callback}; return CallbackUnderTest"
                ))
                .unwrap();
            let captured = {
                let mut lua = env.rilua_mut();
                TimerCallbackSource::capture(lua.state_mut(), callback)
            };
            assert_eq!(
                captured.format_timer_fields(23),
                format!(" timer_id=23 callback_source={expected}")
            );
            let result: i64 = env
                .eval(if registry == "{}" {
                    "return CallbackUnderTest('count') > 0 and 37 or 0"
                } else {
                    "return CallbackUnderTest()"
                })
                .unwrap();
            assert_eq!(result, 37, "capture must not change invocation");
        }
        let callback = env
            .eval::<Val>(
                "debug.getregistry().__wow_timer_original_callbacks = {}; \
                 CallbackUnderTest = function() return 37 end; return CallbackUnderTest",
            )
            .unwrap();
        let captured = {
            let mut lua = env.rilua_mut();
            TimerCallbackSource::capture(lua.state_mut(), callback)
        };
        assert_eq!(
            captured.format_timer_fields(24),
            " timer_id=24 callback_source==(string):1"
        );
        assert_eq!(env.eval::<i64>("return CallbackUnderTest()").unwrap(), 37);
    }

    #[test]
    fn invalid_callback_association_has_explicit_source_marker() {
        let env = crate::lua_api::WowLuaEnv::new().unwrap();
        let callback = env
            .eval::<Val>(
                r#"
            local callback = function() end
            debug.getregistry().__wow_timer_original_callbacks = { [callback] = 42 }
            return callback
            "#,
            )
            .unwrap();
        let mut lua = env.rilua_mut();
        let captured = TimerCallbackSource::capture(lua.state_mut(), callback);
        assert_eq!(
            captured.format_timer_fields(17),
            " timer_id=17 callback_source=<invalid-association>"
        );
    }

    #[test]
    fn direct_callback_without_association_uses_its_own_source() {
        let env = crate::lua_api::WowLuaEnv::new().unwrap();
        let callback = env
            .eval::<Val>(
                "debug.getregistry().__wow_timer_original_callbacks = {}; return function() end",
            )
            .unwrap();
        let mut lua = env.rilua_mut();
        let captured = TimerCallbackSource::capture(lua.state_mut(), callback);
        assert!(captured.source.is_some());
        assert_eq!(captured.callback_source, captured.source);
    }

    #[test]
    fn parse_min_duration_ms_disables_logging_when_unset() {
        assert_eq!(parse_min_duration_ms(None), None);
    }

    #[test]
    fn parse_min_duration_ms_reads_threshold() {
        assert_eq!(parse_min_duration_ms(Some("10")), Some(10.0));
        assert_eq!(parse_min_duration_ms(Some(" 2.5 ")), Some(2.5));
    }

    #[test]
    fn parse_min_duration_ms_falls_back_to_zero_for_invalid_values() {
        assert_eq!(parse_min_duration_ms(Some("")), Some(0.0));
        assert_eq!(parse_min_duration_ms(Some("abc")), Some(0.0));
        assert_eq!(parse_min_duration_ms(Some("-3")), Some(0.0));
    }

    #[test]
    fn duration_filter_is_inclusive() {
        assert!(!should_log_with_threshold(
            Duration::from_micros(9_999),
            Some(10.0)
        ));
        assert!(should_log_with_threshold(
            Duration::from_millis(10),
            Some(10.0)
        ));
        assert!(should_log_with_threshold(
            Duration::from_millis(11),
            Some(10.0)
        ));
    }

    #[test]
    fn format_log_uses_builtin_fallbacks() {
        let line = format_log(
            None,
            "OnUpdate",
            None,
            42,
            Duration::from_micros(1250),
            None,
        );
        assert_eq!(
            line,
            "[handler] addon=__BuiltIn handler=OnUpdate frame=#42 duration_ms=1.250"
        );
    }

    #[test]
    fn format_log_preserves_named_frame_and_addon() {
        let line = format_log(
            Some("Blizzard_UIParent"),
            "OnShow",
            Some("GameMenuFrame"),
            7,
            Duration::from_micros(500),
            None,
        );
        assert_eq!(
            line,
            "[handler] addon=Blizzard_UIParent handler=OnShow frame=GameMenuFrame duration_ms=0.500"
        );
    }

    #[test]
    fn format_log_includes_source_when_known() {
        let line = format_log(
            Some("MacroToolkit"),
            "OnUpdate",
            None,
            56392,
            Duration::from_millis(51),
            Some("@Interface/AddOns/MacroToolkit/modules/showtooltipMock.lua:42"),
        );
        assert_eq!(
            line,
            "[handler] addon=MacroToolkit handler=OnUpdate frame=#56392 duration_ms=51.000 source=@Interface/AddOns/MacroToolkit/modules/showtooltipMock.lua:42"
        );
    }

    fn should_log_with_threshold(duration: Duration, threshold_ms: Option<f64>) -> bool {
        let Some(threshold_ms) = threshold_ms else {
            return false;
        };
        duration.as_secs_f64() * 1000.0 >= threshold_ms
    }
}
