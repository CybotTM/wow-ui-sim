use std::path::PathBuf;
use wow_ui_sim::xml::{XmlElement, parse_xml, parse_xml_file};

fn blizzard_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path()
        .expect("Blizzard UI cache should be available")
        .join("Blizzard_SharedXMLBase")
}

#[test]
fn test_parse_callback_registrant_xml() {
    let path = blizzard_dir().join("CallbackRegistrant.xml");
    let ui = parse_xml_file(&path).expect("Failed to parse XML");

    // Should have Script and Frame elements
    assert!(!ui.elements.is_empty());

    let mut has_script = false;
    let mut has_frame = false;

    for element in &ui.elements {
        match element {
            XmlElement::Script(s) => {
                assert_eq!(s.file.as_deref(), Some("CallbackRegistrant.lua"));
                has_script = true;
            }
            XmlElement::Frame(f) => {
                assert_eq!(f.name.as_deref(), Some("CallbackRegistrantTemplate"));
                assert_eq!(f.mixin.as_deref(), Some("CallbackRegistrantMixin"));
                assert_eq!(f.is_virtual, Some(true));
                has_frame = true;
            }
            _ => {}
        }
    }

    assert!(has_script, "Expected Script element");
    assert!(has_frame, "Expected Frame element");
}

#[test]
fn test_parse_color_swatch_xml() {
    let path = blizzard_dir().join("ColorSwatch.xml");
    let ui = parse_xml_file(&path).expect("Failed to parse XML");

    // Find the ColorSwatchTemplate frame
    for element in &ui.elements {
        if let XmlElement::Frame(f) = element {
            if f.name.as_deref() == Some("ColorSwatchTemplate") {
                // Check mixin
                assert_eq!(f.mixin.as_deref(), Some("ColorSwatchMixin"));

                // Check size
                let size = f.size().expect("Expected size");
                assert_eq!(size.x, Some(16.0));
                assert_eq!(size.y, Some(16.0));

                // Check layers exist
                let layers: Vec<_> = f.layers().collect();
                assert!(!layers.is_empty(), "Expected at least one layer");

                // Check for textures in layers
                let has_textures = layers.iter().any(|l| {
                    l.layers
                        .iter()
                        .any(|layer| layer.textures().next().is_some())
                });
                assert!(has_textures, "Expected textures in layers");

                return;
            }
        }
    }
    panic!("ColorSwatchTemplate frame not found");
}

#[test]
fn test_parse_all_xml_in_shared_xml_base() {
    // Try to parse all XML files in the directory
    let dir = blizzard_dir();
    let mut parsed = 0;
    let mut failed = Vec::new();

    for entry in std::fs::read_dir(dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.extension().map(|e| e == "xml").unwrap_or(false) {
            match parse_xml_file(&path) {
                Ok(_) => parsed += 1,
                Err(e) => failed.push((path.clone(), e)),
            }
        }
    }

    // Report results
    println!("Parsed {} XML files successfully", parsed);
    if !failed.is_empty() {
        println!("Failed to parse {} files:", failed.len());
        for (path, error) in &failed {
            println!("  {:?}: {}", path.file_name().unwrap(), error);
        }
    }

    // At least some files should parse
    assert!(parsed > 0, "Expected to parse at least some XML files");

    // Allow some failures for now (complex elements we haven't implemented)
    // but most should parse
    let total = parsed + failed.len();
    let success_rate = parsed as f64 / total as f64;
    assert!(
        success_rate >= 0.5,
        "Expected at least 50% success rate, got {:.0}%",
        success_rate * 100.0
    );
}

#[test]
fn test_xml_with_scripts() {
    // Test parsing XML with inline scripts
    let xml = r#"
        <Ui>
            <Frame name="TestFrame">
                <Scripts>
                    <OnLoad>
                        self:RegisterEvent("PLAYER_LOGIN")
                    </OnLoad>
                    <OnEvent method="OnEvent"/>
                    <OnShow inherit="append">
                        print("shown")
                    </OnShow>
                </Scripts>
            </Frame>
        </Ui>
    "#;

    let ui = wow_ui_sim::xml::parse_xml(xml).expect("Failed to parse XML");

    if let XmlElement::Frame(f) = &ui.elements[0] {
        let scripts = f.scripts().expect("Expected scripts");

        // Check OnLoad has inline code
        let on_load = scripts.on_load.first().expect("Expected OnLoad");
        assert!(on_load.body.is_some());

        // Check OnEvent uses method reference
        let on_event = scripts.on_event.first().expect("Expected OnEvent");
        assert_eq!(on_event.method.as_deref(), Some("OnEvent"));

        // Check OnShow has inherit attribute
        let on_show = scripts.on_show.first().expect("Expected OnShow");
        assert_eq!(on_show.inherit.as_deref(), Some("append"));
    } else {
        panic!("Expected Frame element");
    }
}

// --- New frame type XML elements (#3-#18) ---

#[test]
fn test_parse_new_frame_types_top_level() {
    let frame_types = [
        "TaxiRouteFrame",
        "ModelFFX",
        "TabardModel",
        "UiCamera",
        "UnitPositionFrame",
        "OffScreenFrame",
        "Checkout",
        "FogOfWarFrame",
        "QuestPOIFrame",
        "ArchaeologyDigSiteFrame",
        "ScenarioPOIFrame",
        "UIThemeContainerFrame",
        "EventScrollFrame",
        "ContainedAlertFrame",
        "MapScene",
    ];

    for ft in &frame_types {
        let xml = format!(
            r#"<Ui><{ft} name="Test{ft}" virtual="true"><Size x="100" y="50"/></{ft}></Ui>"#,
        );
        let ui = parse_xml(&xml).unwrap_or_else(|e| panic!("Failed to parse {ft}: {e}"));
        assert_eq!(ui.elements.len(), 1, "{ft} should produce one element");

        // Verify name and size parsed
        let f = match &ui.elements[0] {
            XmlElement::TaxiRouteFrame(f)
            | XmlElement::ModelFFX(f)
            | XmlElement::TabardModel(f)
            | XmlElement::UiCamera(f)
            | XmlElement::UnitPositionFrame(f)
            | XmlElement::OffScreenFrame(f)
            | XmlElement::Checkout(f)
            | XmlElement::FogOfWarFrame(f)
            | XmlElement::QuestPOIFrame(f)
            | XmlElement::ArchaeologyDigSiteFrame(f)
            | XmlElement::ScenarioPOIFrame(f)
            | XmlElement::UIThemeContainerFrame(f)
            | XmlElement::EventScrollFrame(f)
            | XmlElement::ContainedAlertFrame(f)
            | XmlElement::MapScene(f) => f,
            other => panic!("Expected {ft} variant, got {other:?}"),
        };
        assert_eq!(f.name.as_deref(), Some(&format!("Test{ft}")[..]));
        assert_eq!(f.size().unwrap().x, Some(100.0));
    }
}

#[test]
fn test_parse_new_frame_types_as_children() {
    let xml = r#"
        <Ui>
            <Frame name="Parent">
                <Frames>
                    <TaxiRouteFrame name="Child1"/>
                    <ModelFFX name="Child2"/>
                    <TabardModel name="Child3"/>
                    <UiCamera name="Child4"/>
                    <UnitPositionFrame name="Child5"/>
                    <OffScreenFrame name="Child6"/>
                    <Checkout name="Child7"/>
                    <FogOfWarFrame name="Child8"/>
                    <QuestPOIFrame name="Child9"/>
                    <ArchaeologyDigSiteFrame name="Child10"/>
                    <ScenarioPOIFrame name="Child11"/>
                    <UIThemeContainerFrame name="Child12"/>
                    <EventScrollFrame name="Child13"/>
                    <ContainedAlertFrame name="Child14"/>
                    <MapScene name="Child15"/>
                    <ScopedModifier name="Child16"/>
                </Frames>
            </Frame>
        </Ui>
    "#;
    let ui = parse_xml(xml).expect("Failed to parse child frame types");
    let frame = match &ui.elements[0] {
        XmlElement::Frame(f) => f,
        _ => panic!("Expected Frame"),
    };
    let frames = frame.all_frame_elements();
    assert_eq!(frames.len(), 15);
}
