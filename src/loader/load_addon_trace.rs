use crate::lua_api::{LoaderEnv, SimState};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::OnceLock;

pub(crate) const TRACE_LOAD_ADDON_ENV: &str = "WOW_SIM_TRACE_LOAD_ADDON";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LoadAddonTraceOrigin {
    Lua,
    Xml,
    Toc,
}

impl LoadAddonTraceOrigin {
    fn prefix(self) -> &'static str {
        match self {
            Self::Lua => "[lua LoadAddOn]",
            Self::Xml => "[xml LoadAddOn]",
            Self::Toc => "[toc LoadAddOn]",
        }
    }
}

pub(crate) fn trace_load_addon_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        trace_load_addon_enabled_from(std::env::var(TRACE_LOAD_ADDON_ENV).ok().as_deref())
    })
}

fn trace_load_addon_enabled_from(value: Option<&str>) -> bool {
    !matches!(value, None | Some("") | Some("0") | Some("false") | Some("FALSE"))
}

pub(crate) fn trace_load_addon(origin: LoadAddonTraceOrigin, message: impl AsRef<str>) {
    if !trace_load_addon_enabled() {
        return;
    }
    eprintln!("{} {}", origin.prefix(), message.as_ref());
}

pub(crate) fn runtime_load_addon_origin(xml_depth: u32) -> LoadAddonTraceOrigin {
    if xml_depth > 0 {
        LoadAddonTraceOrigin::Xml
    } else {
        LoadAddonTraceOrigin::Lua
    }
}

pub(crate) fn enter_xml_load_addon_context(env: &LoaderEnv<'_>) -> XmlLoadAddonTraceGuard {
    env.state().borrow_mut().xml_load_addon_depth += 1;
    XmlLoadAddonTraceGuard {
        state: Rc::clone(env.state()),
    }
}

pub(crate) struct XmlLoadAddonTraceGuard {
    state: Rc<RefCell<SimState>>,
}

impl Drop for XmlLoadAddonTraceGuard {
    fn drop(&mut self) {
        let mut state = self.state.borrow_mut();
        state.xml_load_addon_depth = state.xml_load_addon_depth.saturating_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_load_addon_enabled_from_var() {
        assert!(!trace_load_addon_enabled_from(None));
        assert!(!trace_load_addon_enabled_from(Some("")));
        assert!(!trace_load_addon_enabled_from(Some("0")));
        assert!(!trace_load_addon_enabled_from(Some("false")));
        assert!(trace_load_addon_enabled_from(Some("1")));
        assert!(trace_load_addon_enabled_from(Some("yes")));
    }

    #[test]
    fn test_runtime_load_addon_origin_uses_xml_depth() {
        assert_eq!(runtime_load_addon_origin(0), LoadAddonTraceOrigin::Lua);
        assert_eq!(runtime_load_addon_origin(1), LoadAddonTraceOrigin::Xml);
    }

    #[test]
    fn test_load_addon_trace_prefixes() {
        assert_eq!(LoadAddonTraceOrigin::Lua.prefix(), "[lua LoadAddOn]");
        assert_eq!(LoadAddonTraceOrigin::Xml.prefix(), "[xml LoadAddOn]");
        assert_eq!(LoadAddonTraceOrigin::Toc.prefix(), "[toc LoadAddOn]");
    }
}
