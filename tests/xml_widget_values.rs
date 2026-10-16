//! XML range/default attributes must reach the real widget before scripts run.
use wow_ui_sim::lua_api::WowLuaEnv;

#[cfg(any(feature = "client-retail", feature = "client-wowforever"))]
#[test]
fn original_tracker_progressbar_uses_its_cached_xml_range_and_default() {
    let cache = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    let (env, _) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &cache,
        &["Blizzard_ObjectiveTracker"],
        &[],
    );
    env.exec("XmlRangeOriginal = CreateFrame('Frame', nil, UIParent, 'ObjectiveTrackerProgressBarTemplate')").unwrap();
    let initial: (f64, f64, f64) = env.eval("local low, high = XmlRangeOriginal.Bar:GetMinMaxValues(); return low, high, XmlRangeOriginal.Bar:GetValue()").unwrap();
    assert_eq!(
        initial,
        (0.0, 100.0, 0.0),
        "original cached XML range and default"
    );
    env.exec("XmlRangeOriginal:SetPercent(50)").unwrap();
    let rendered: (f64, String) = env
        .eval("return XmlRangeOriginal.Bar:GetValue(), XmlRangeOriginal.Bar.Label:GetText()")
        .unwrap();
    assert_eq!(rendered, (50.0, "50%".into()));
}

#[test]
fn xml_statusbar_and_slider_values_preserve_inheritance_and_precede_onload() {
    let env = WowLuaEnv::new().unwrap();
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("WidgetValues.toc"),
        "## Title: WidgetValues\nWidgetValues.xml\n",
    )
    .unwrap();
    std::fs::write(dir.path().join("WidgetValues.xml"), r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <StatusBar name="XmlRangeBase" virtual="true" minValue="10" maxValue="100" defaultValue="25"/>
            <StatusBar name="XmlRangeDirect" inherits="XmlRangeBase" maxValue="80" defaultValue="50">
                <Scripts><OnLoad>local low, high = self:GetMinMaxValues(); self.loaded = low == 10 and high == 80 and self:GetValue() == 50</OnLoad></Scripts>
            </StatusBar>
            <Slider name="XmlSliderRangeBase" virtual="true" minValue="10" maxValue="20" defaultValue="15"/>
            <Slider name="XmlSliderRange" inherits="XmlSliderRangeBase">
                <Scripts><OnLoad>local low, high = self:GetMinMaxValues(); self.loaded = low == 10 and high == 20 and self:GetValue() == 15</OnLoad></Scripts>
            </Slider>
        </Ui>
    "#).unwrap();
    wow_ui_sim::loader::load_addon(&env.loader_env(), &dir.path().join("WidgetValues.toc"))
        .unwrap();
    let initial: (f64, f64) = env
        .eval("return XmlRangeDirect:GetValue(), XmlSliderRange:GetValue()")
        .unwrap();
    assert_eq!(
        initial,
        (50.0, 15.0),
        "shared XML parser must retain range/default attributes"
    );
    env.exec(
        r#"
        assert(XmlRangeDirect.loaded and XmlSliderRange.loaded)
        local runtime = CreateFrame('StatusBar', nil, UIParent, 'XmlRangeBase')
        local low, high = runtime:GetMinMaxValues()
        assert(low == 10 and high == 100 and runtime:GetValue() == 25)
        local slider = CreateFrame('Slider', nil, UIParent, 'XmlSliderRangeBase')
        local sliderLow, sliderHigh = slider:GetMinMaxValues()
        assert(sliderLow == 10 and sliderHigh == 20 and slider:GetValue() == 15)
    "#,
    )
    .unwrap();
}
