//! Bound generated-code compilation during real XML reconstruction after reload.

use super::{AddonContext, LoadTiming, WowLuaEnv, load_xml_file};
use std::path::Path;
use std::process::Command;
use std::rc::Rc;

const TEST_NAME: &str =
    "loader::tests::reload_xml_cost::reload_xml_layer_reconstruction_bounds_source_compiles";
const CHILD_ENV: &str = "WOW_SIM_RELOAD_XML_COST_CHILD";
const RELOAD_BEGIN: &str = "[reload-xml-cost-begin]";
const RELOAD_END: &str = "[reload-xml-cost-end]";
const INSTANCE_COUNT: usize = 16;
const RELOAD_COUNT: usize = 2;
// One homogeneous XML operation may prepare code once in each fresh VM,
// but must not compile again for every instance's generated name/frame ID.
const MAX_SOURCE_COMPILES_PER_RELOAD: usize = 1;

fn write_xml_fixture(root: &Path) {
    let frames: String = (0..INSTANCE_COUNT)
        .map(|index| {
            format!(
                r#"<Frame name="ReloadCost{index}" parent="UIParent">
                    <Layers><Layer level="BACKGROUND">
                        <Texture parentKey="Ink">
                            <Size x="32" y="16"/>
                            <Color r="0.25" g="0.5" b="0.75" a="1"/>
                            <Anchors><Anchor point="TOPLEFT"/></Anchors>
                        </Texture>
                    </Layer></Layers>
                </Frame>"#
            )
        })
        .collect();
    std::fs::write(root.join("reload-cost.xml"), format!("<Ui>{frames}</Ui>")).unwrap();
}

fn load_xml_fixture(env: &WowLuaEnv, root: &Path) -> crate::Result<()> {
    let addon_table = env.create_addon_table()?;
    let context = AddonContext::new(env.lua(), "ReloadCost", addon_table, root, false, false)?;
    let mut warnings = Vec::new();
    let result = load_xml_file(
        &env.loader_env(),
        &root.join("reload-cost.xml"),
        &context,
        &mut LoadTiming::default(),
        &mut warnings,
    );
    assert!(warnings.is_empty(), "unexpected XML warnings: {warnings:?}");
    result
        .map(|_| ())
        .map_err(|error| crate::Error::Other(error.to_string()))
}

fn assert_fresh_regions(env: &WowLuaEnv) {
    env.exec(&format!(
        r#"
        assert(ReloadCostOldSentinel == nil,
            'ReloadCostOldSentinel: expected nil, actual ' .. tostring(ReloadCostOldSentinel))
        local seen = {{}}
        for index = 0, {} do
            local name = 'ReloadCost' .. index
            local frame = _G[name]
            assert(frame, name .. ': expected frame, actual ' .. tostring(frame))
            assert(frame:GetParent() == UIParent,
                name .. ': expected parent UIParent, actual ' .. tostring(frame:GetParent()))
            local ink = frame.Ink
            local region = name .. '.Ink'
            assert(ink, region .. ': expected region, actual ' .. tostring(ink))
            assert(ink:GetParent() == frame,
                region .. ': expected parent ' .. tostring(frame) .. ', actual ' .. tostring(ink:GetParent()))
            assert(not seen[ink],
                region .. ': expected distinct region, actual reused ' .. tostring(ink))
            seen[ink] = true
            local width, height = ink:GetWidth(), ink:GetHeight()
            assert(width == 32 and height == 16,
                region .. ': expected size 32x16, actual ' .. tostring(width) .. 'x' .. tostring(height))
            local layer = ink:GetDrawLayer()
            assert(layer == 'BACKGROUND',
                region .. ': expected draw layer BACKGROUND, actual ' .. tostring(layer))
        end
        ReloadCost0.Ink:SetWidth(7)
        local width = ReloadCost1.Ink:GetWidth()
        assert(width == 32,
            'ReloadCost1.Ink: expected independent width 32, actual ' .. tostring(width))
        "#,
        INSTANCE_COUNT - 1
    ))
    .unwrap();
    assert!(env.state().borrow().lua_errors.is_empty());
}

fn reconstruct_xml_across_reloads() {
    let fixture = tempfile::tempdir().unwrap();
    write_xml_fixture(fixture.path());
    let mut env = WowLuaEnv::new().unwrap();
    let root = fixture.path().to_path_buf();
    env.set_reload_loader(move |env, _| {
        eprintln!("{RELOAD_BEGIN}");
        let result = load_xml_fixture(env, &root);
        eprintln!("{RELOAD_END}");
        result
    });
    load_xml_fixture(&env, fixture.path()).unwrap();
    assert_fresh_regions(&env);
    for _ in 0..RELOAD_COUNT {
        let old_lua = Rc::downgrade(&env.lua);
        env.exec("ReloadCostOldSentinel = {}; ReloadUI()").unwrap();
        assert!(env.service_reload(&mut None).unwrap());
        assert!(old_lua.upgrade().is_none());
        assert_fresh_regions(&env);
    }
}

fn assert_reload_compile_budget(stderr: &str) {
    let reloads: Vec<_> = stderr.split(RELOAD_BEGIN).skip(1).collect();
    assert_eq!(reloads.len(), RELOAD_COUNT, "missing reload observations");
    for (index, reload) in reloads.iter().enumerate() {
        let (loading, _) = reload.split_once(RELOAD_END).expect("reload must finish");
        let compiles = loading
            .lines()
            .filter(|line| line.contains("[generated-chunk]"))
            .filter(|line| line.split_whitespace().any(|field| field == "mode=compile"))
            .filter(|line| {
                line.split_whitespace()
                    .any(|field| field == "phase=xml-frame")
            })
            .count();
        eprintln!(
            "[reload-xml-cost] reload={} source_compiles={compiles} budget={MAX_SOURCE_COMPILES_PER_RELOAD}",
            index + 1
        );
        assert!(
            compiles <= MAX_SOURCE_COMPILES_PER_RELOAD,
            "reload {} compiled {compiles} generated XML chunks for {INSTANCE_COUNT} identical \
             layer operations; budget is {MAX_SOURCE_COMPILES_PER_RELOAD}\n{loading}",
            index + 1
        );
    }
}

#[test]
fn reload_xml_layer_reconstruction_bounds_source_compiles() {
    if std::env::var_os(CHILD_ENV).is_some() {
        reconstruct_xml_across_reloads();
        return;
    }
    // Fresh process: profiling/cache OnceLocks must not inherit other tests.
    // Fresh cache: unrelated previous runs must not hide ID-key churn.
    let cache = tempfile::tempdir().unwrap();
    let output = Command::new("timeout")
        .arg("90")
        .arg(std::env::current_exe().unwrap())
        .args(["--exact", TEST_NAME, "--nocapture", "--test-threads=1"])
        .env(CHILD_ENV, "1")
        .env("WOW_SIM_PROFILE_GENERATED_CHUNKS", "1")
        .env("WOW_SIM_ENABLE_BYTECODE_CACHE", "1")
        .env("XDG_CACHE_HOME", cache.path())
        .output()
        .unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        output.status.success(),
        "XML/reload behavior failed before cost assertion:\n{}\n{stderr}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert_reload_compile_budget(&stderr);
}
