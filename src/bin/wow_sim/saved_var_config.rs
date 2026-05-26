use wow_ui_sim::logging;
use wow_ui_sim::saved_variables::SavedVariablesManager;

pub fn configure_saved_vars(no_saved_vars_arg: bool) -> Option<SavedVariablesManager> {
    if should_skip_saved_vars(no_saved_vars_arg) {
        logging::println_elapsed("SavedVariables loading disabled");
        return None;
    }

    let mut saved_vars = SavedVariablesManager::new();
    if let Some(wtf) = wow_ui_sim::paths::default_wtf_config() {
        logging::println_elapsed(&format!(
            "WTF import source (read-only): {} @ {}/{}",
            wtf.account, wtf.realm, wtf.character
        ));
        saved_vars.set_wtf_config(wtf);
    } else {
        logging::println_elapsed("WTF config: no WoW WTF directory found");
    }
    Some(saved_vars)
}

pub fn configure_edit_mode_cache_vars() -> Option<SavedVariablesManager> {
    let mut saved_vars = SavedVariablesManager::new();
    if let Some(wtf) = wow_ui_sim::paths::default_wtf_config() {
        logging::println_elapsed(&format!(
            "EditMode cache source (read-only): {} @ {}/{}",
            wtf.account, wtf.realm, wtf.character
        ));
        saved_vars.set_wtf_config(wtf);
        Some(saved_vars)
    } else {
        logging::println_elapsed("EditMode cache: no WoW WTF directory found");
        None
    }
}

fn should_skip_saved_vars(no_saved_vars_arg: bool) -> bool {
    no_saved_vars_arg
        || std::env::var("WOW_SIM_NO_SAVED_VARS")
            .map(|value| value == "1")
            .unwrap_or(false)
}
