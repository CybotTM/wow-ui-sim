use super::CreateFrameTemplateTiming;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
struct CreateFrameProfileConfig {
    log_all: bool,
    min_duration: Duration,
}

#[derive(Clone, Copy)]
pub(super) struct CreateFrameCallProfile {
    config: Option<CreateFrameProfileConfig>,
    total_start: Option<Instant>,
    pub(super) runtime_call: bool,
}

pub(super) fn begin_create_frame_call_profile(lua: &mlua::Lua) -> CreateFrameCallProfile {
    let config = create_frame_profile_config();
    let runtime_call = config.is_some() && current_create_frame_suppress_depth(lua) <= 0;
    CreateFrameCallProfile {
        config,
        total_start: runtime_call.then(Instant::now),
        runtime_call,
    }
}

pub(super) fn maybe_log_create_frame_profile(
    profile: CreateFrameCallProfile,
    finalize_start: Option<Instant>,
    frame_type: &str,
    ref_name: &str,
    template: Option<&str>,
    template_timing: &CreateFrameTemplateTiming,
) {
    if let (Some(cfg), Some(total_start)) = (profile.config, profile.total_start) {
        let total = total_start.elapsed();
        if cfg.log_all || total >= cfg.min_duration {
            let finalize = finalize_start
                .map(|start| start.elapsed())
                .unwrap_or_default();
            log_create_frame_profile(
                frame_type,
                ref_name,
                template,
                total,
                finalize,
                template_timing,
            );
        }
    }
}

fn create_frame_profile_config() -> Option<CreateFrameProfileConfig> {
    static CONFIG: OnceLock<Option<CreateFrameProfileConfig>> = OnceLock::new();
    *CONFIG.get_or_init(|| {
        let raw = std::env::var("WOW_SIM_PROFILE_CREATE_FRAME").ok()?;
        let value = raw.trim();
        if value.is_empty()
            || value == "0"
            || value.eq_ignore_ascii_case("false")
            || value.eq_ignore_ascii_case("off")
        {
            return None;
        }
        Some(CreateFrameProfileConfig {
            log_all: value.eq_ignore_ascii_case("all"),
            min_duration: parse_create_frame_profile_min_duration(
                std::env::var("WOW_SIM_PROFILE_CREATE_FRAME_MIN_MS")
                    .ok()
                    .as_deref(),
            ),
        })
    })
}

fn parse_create_frame_profile_min_duration(raw: Option<&str>) -> Duration {
    raw.and_then(|s| s.trim().parse::<u64>().ok())
        .map(Duration::from_millis)
        .unwrap_or_else(|| Duration::from_millis(5))
}

fn current_create_frame_suppress_depth(lua: &mlua::Lua) -> i32 {
    lua.globals()
        .get("__suppress_create_frame_onload")
        .unwrap_or(0)
}

fn log_create_frame_profile(
    frame_type: &str,
    frame_name: &str,
    template: Option<&str>,
    total: Duration,
    finalize: Duration,
    template_timing: &CreateFrameTemplateTiming,
) {
    crate::logging::println_elapsed(&format!(
        "[CreateFrame] name={} type={} template={} total={:.2?} finalize={:.2?} intrinsic={:.2?} explicit={:.2?} deferred_onloads={:.2?} child_onloads={} self_onload={:.2?}",
        frame_name,
        frame_type,
        template.unwrap_or("-"),
        total,
        finalize,
        template_timing.intrinsic_templates,
        template_timing.explicit_templates,
        template_timing.deferred_child_onloads,
        template_timing.deferred_child_count,
        template_timing.self_onload,
    ));
}

#[cfg(test)]
mod tests {
    use super::parse_create_frame_profile_min_duration;
    use std::time::Duration;

    #[test]
    fn create_frame_profile_min_duration_defaults_to_five_ms() {
        assert_eq!(
            parse_create_frame_profile_min_duration(None),
            Duration::from_millis(5)
        );
        assert_eq!(
            parse_create_frame_profile_min_duration(Some("")),
            Duration::from_millis(5)
        );
        assert_eq!(
            parse_create_frame_profile_min_duration(Some("garbage")),
            Duration::from_millis(5)
        );
    }

    #[test]
    fn create_frame_profile_min_duration_parses_ms() {
        assert_eq!(
            parse_create_frame_profile_min_duration(Some("17")),
            Duration::from_millis(17)
        );
    }
}
