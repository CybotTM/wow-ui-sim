use super::AddonTiming;

pub(super) fn print_slowest_addons(addon_timings: &[AddonTiming]) {
    let sorted = slowest_addons(addon_timings);
    println!("\nSlowest addons:");
    for addon in sorted.iter().take(10) {
        let timing = &addon.timing;
        println!(
            "  {:>7.1?}  {} (lua={:.1?} compile={:.1?} call={:.1?} sv={:.1?} xml={:.1?})",
            timing.total(),
            addon.name,
            timing.lua_exec_time,
            timing.lua_compile_time,
            timing.lua_call_time,
            timing.saved_vars_time,
            timing.xml_parse_time + timing.xml_process_time
        );
    }
}

fn slowest_addons(addon_timings: &[AddonTiming]) -> Vec<AddonTiming> {
    let mut sorted = addon_timings.to_vec();
    sorted.sort_by(|a, b| b.timing.total().cmp(&a.timing.total()));
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;
    use wow_ui_sim::loader::LoadTiming;

    #[test]
    fn slowest_addons_sorts_by_total_timing() {
        let fast = AddonTiming {
            name: "FastAddon".to_string(),
            timing: LoadTiming {
                lua_exec_time: std::time::Duration::from_millis(10),
                ..Default::default()
            },
        };
        let slow = AddonTiming {
            name: "SlowAddon".to_string(),
            timing: LoadTiming {
                lua_exec_time: std::time::Duration::from_millis(20),
                ..Default::default()
            },
        };

        let sorted = slowest_addons(&[fast, slow]);

        assert_eq!(sorted[0].name, "SlowAddon");
        assert_eq!(sorted[1].name, "FastAddon");
    }
}
