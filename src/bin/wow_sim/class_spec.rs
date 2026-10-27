use sim_data::specializations::specs_for_class;
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::CLASS_LABELS;

pub(super) fn resolve_class(value: &str) -> Result<i32, String> {
    let index = value
        .parse::<usize>()
        .ok()
        .filter(|&n| n > 0 && n <= CLASS_LABELS.len())
        .or_else(|| {
            CLASS_LABELS
                .iter()
                .position(|name| name.eq_ignore_ascii_case(value))
                .map(|n| n + 1)
        });
    index.map(|n| n as i32).ok_or_else(|| {
        format!(
            "unknown class '{value}'; valid classes: {}",
            CLASS_LABELS.join(", ")
        )
    })
}

pub(super) fn resolve_spec(class: i32, value: &str) -> Result<i32, String> {
    let specs: Vec<_> = specs_for_class(class as u32).collect();
    let index = value
        .parse::<u32>()
        .ok()
        .and_then(|n| specs.iter().find(|s| s.order_index + 1 == n))
        .or_else(|| specs.iter().find(|s| s.name.eq_ignore_ascii_case(value)));
    index.map(|s| (s.order_index + 1) as i32).ok_or_else(|| {
        let valid = specs.iter().map(|s| s.name).collect::<Vec<_>>().join(", ");
        format!(
            "unknown specialization '{value}' for class {}; valid specializations: {valid}",
            CLASS_LABELS
                .get(class.saturating_sub(1) as usize)
                .copied()
                .unwrap_or("Unknown")
        )
    })
}

pub(super) struct Selection {
    class: Option<i32>,
    spec: Option<String>,
}

fn profile_supports_spec(profile: ClientProfile) -> bool {
    !matches!(
        profile,
        ClientProfile::Wrath | ClientProfile::Era | ClientProfile::Anniversary
    )
}

pub(super) fn parse_selection(
    class: Option<&str>,
    spec: Option<&str>,
) -> Result<Option<Selection>, String> {
    if class.is_none() && spec.is_none() {
        return Ok(None);
    }
    let class = class.map(resolve_class).transpose()?;
    if spec.is_some() && !profile_supports_spec(ACTIVE) {
        return Err(format!(
            "--spec is not available for the {:?} profile",
            ACTIVE
        ));
    }
    if let (Some(class), Some(spec)) = (class, spec) {
        resolve_spec(class, spec)?;
    }
    Ok(Some(Selection {
        class,
        spec: spec.map(str::to_owned),
    }))
}

pub(super) fn schedule_selection(env: &WowLuaEnv, selection: Selection) {
    env.set_post_startup_exec_hook(move |env| apply_selection(env, selection));
}

fn apply_selection(env: &WowLuaEnv, selection: Selection) {
    let current_class = selection
        .class
        .unwrap_or_else(|| env.state().borrow().player.class_index);
    let spec_index = selection
        .spec
        .as_deref()
        .map(|name| resolve_spec(current_class, name))
        .transpose()
        .unwrap_or_else(|error| {
            eprintln!("{error}");
            std::process::exit(2);
        });
    let mut code = String::new();
    if let Some(class) = selection.class {
        code.push_str(&format!("A_Admin.SetPlayerClass({class})\n"));
    }
    if let Some(spec) = spec_index {
        code.push_str(&format!("A_Admin.SetSpec({spec})\n"));
    }
    code.push_str("A_Admin.FireEvent('PLAYER_SPECIALIZATION_CHANGED', 'player')");
    if let Err(error) = env.exec(&code) {
        eprintln!("failed to apply class/spec context: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{profile_supports_spec, resolve_class, resolve_spec};
    use wow_ui_sim::client_profile::ClientProfile;

    #[test]
    fn class_names_and_indices() {
        assert_eq!(resolve_class("dRuId").unwrap(), 11);
        assert_eq!(resolve_class("Death Knight").unwrap(), 6);
        assert_eq!(resolve_class("8").unwrap(), 8);
        assert!(resolve_class("0").unwrap_err().contains("Warrior"));
        assert!(resolve_class("unknown").unwrap_err().contains("Druid"));
    }

    #[test]
    fn specialization_support_matches_client_profiles() {
        for profile in [
            ClientProfile::Retail,
            ClientProfile::Ptr,
            ClientProfile::Mists,
            ClientProfile::WowForever,
        ] {
            assert!(profile_supports_spec(profile), "{profile:?}");
        }
        for profile in [
            ClientProfile::Wrath,
            ClientProfile::Era,
            ClientProfile::Anniversary,
        ] {
            assert!(!profile_supports_spec(profile), "{profile:?}");
        }
    }

    #[test]
    fn specialization_names_and_indices_are_class_scoped() {
        assert_eq!(resolve_spec(11, "restoration").unwrap(), 4);
        assert_eq!(resolve_spec(8, "FIRE").unwrap(), 2);
        assert_eq!(resolve_spec(8, "3").unwrap(), 3);
        assert!(
            resolve_spec(8, "Restoration")
                .unwrap_err()
                .contains("Arcane")
        );
        assert!(resolve_spec(8, "4").is_err());
    }
}
