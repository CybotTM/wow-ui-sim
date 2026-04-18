use crate::lua_api::WowLuaEnv;

pub use crate::lua_api::global_slots::SlotCoverageReport;

pub fn slot_coverage_report(env: &WowLuaEnv) -> SlotCoverageReport {
    crate::lua_api::global_slots::slot_coverage_report(env)
}
