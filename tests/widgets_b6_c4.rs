use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;

#[cfg(feature = "gui")]
#[test]
fn b6_c4_utf8_password_matches_byte_count_stars_and_caret() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    use std::{cell::RefCell, rc::Rc};
    use wow_ui_sim::render::WowFontSystem;
    let env = WowLuaEnv::new().unwrap();
    let fonts = WowFontSystem::new_without_casc();
    assert!(
        fonts.font_system.db().faces().next().is_some(),
        "SETUP: fonts required"
    );
    env.set_font_system(Rc::new(RefCell::new(fonts)));
    env.exec(
        r#"
        function MakeB6Box(name)
            local b=CreateFrame('EditBox',name,UIParent)
            b:SetAutoFocus(false); b:SetSize(500,60)
            b:SetPoint('TOPLEFT',UIParent,'TOPLEFT',30,-30)
            assert(b:SetFont('Fonts\\FRIZQT__.TTF',20))
            b:SetJustifyH('LEFT'); b:SetJustifyV('TOP')
            b:SetTextInsets(5,5,3,3)
            b:SetScript('OnCursorChanged',function(self,x,y,w,h) self.caret={x,y,w,h} end)
            b:Show(); return b
        end
        Masked=MakeB6Box('B6Masked'); Stars=MakeB6Box('B6Stars')
        Masked:SetPassword(true); Masked:SetText('é猫')
        Stars:SetText('*****')
        assert(Masked:GetText()=='é猫' and Masked:IsPassword())
    "#,
    )
    .unwrap();
    for byte in [0, 2, 5] {
        env.exec(&format!(
            "Masked:SetCursorPosition({byte}); Stars:SetCursorPosition({byte})"
        ))
        .unwrap();
        env.fire_on_update(0.016).unwrap();
        env.exec(r#"
            assert(Masked.caret and Stars.caret,'SETUP: cursor layout callbacks required')
            for i=1,4 do assert(math.abs(Masked.caret[i]-Stars.caret[i])<0.01,'byte-mask caret mismatch') end
        "#).unwrap();
    }
    env.exec("Masked:SetPassword(false); Masked:SetCursorPosition(5)")
        .unwrap();
    env.fire_on_update(0.016).unwrap();
    env.exec("assert(Masked:GetText()=='é猫' and not Masked:IsPassword())")
        .unwrap();
}
#[test]
fn b6_c4_xml_password_one_precedes_onload() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = WowLuaEnv::new().unwrap();
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("B6.xml"),r#"<Ui><EditBox name="B6XmlPassword" password="1"><Scripts><OnLoad>self.loadedPassword=self:IsPassword()</OnLoad></Scripts></EditBox></Ui>"#).unwrap();
    let toc = dir.path().join("B6.toc");
    std::fs::write(&toc, "## Title: Password XML\nB6.xml\n").unwrap();
    let loaded = wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).unwrap();
    assert!(loaded.warnings.is_empty(), "SETUP: {:?}", loaded.warnings);
    env.exec("assert(B6XmlPassword:IsPassword() and B6XmlPassword.loadedPassword)")
        .unwrap();
}

#[cfg(feature = "gui")]
#[test]
fn b6_c4_masked_utf8_glyphs_match_five_stars() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    use std::{cell::RefCell, rc::Rc};
    use wow_ui_sim::iced_app::{RegistryQuadBatchParams, build_quad_batch_for_registry};
    use wow_ui_sim::render::shader::GLYPH_ATLAS_TEX_INDEX;
    use wow_ui_sim::render::{GlyphAtlas, WowFontSystem};
    let env = WowLuaEnv::new().unwrap();
    let fonts = Rc::new(RefCell::new(WowFontSystem::new_without_casc()));
    assert!(
        fonts.borrow().font_system.db().faces().next().is_some(),
        "SETUP: fonts required"
    );
    env.set_font_system(Rc::clone(&fonts));
    env.exec(
        r#"
        P=CreateFrame('EditBox','B6GlyphMask',UIParent)
        P:SetAutoFocus(false);P:SetSize(500,60)
        P:SetPoint('TOPLEFT',UIParent,'TOPLEFT',20,-20)
        assert(P:SetFont('Fonts\\FRIZQT__.TTF',20))
        P:SetText('é猫');P:SetPassword(true);P:Show()
    "#,
    )
    .unwrap();
    let mut glyphs = GlyphAtlas::new();
    let mut collect = || {
        env.fire_on_update(0.016).unwrap();
        let mut sim = env.state().borrow_mut();
        sim.ensure_layout_rects();
        let _ = sim.get_strata_buckets();
        let mut fs = fonts.borrow_mut();
        let batch = build_quad_batch_for_registry(
            RegistryQuadBatchParams::new(
                &sim.widgets,
                (1600.0, 1200.0),
                sim.strata_buckets.as_ref().unwrap(),
            )
            .root_name(Some("B6GlyphMask"))
            .text_ctx(Some((&mut fs, &mut glyphs))),
        );
        batch
            .vertices
            .iter()
            .filter(|v| v.tex_index == GLYPH_ATLAS_TEX_INDEX)
            .map(|v| (v.position, v.tex_coords))
            .collect::<Vec<_>>()
    };
    let mask = collect();
    assert!(!mask.is_empty(), "SETUP/output: missing glyphs");
    env.exec("assert(P:GetText()=='é猫');P:SetPassword(false);P:SetText('*****')")
        .unwrap();
    let stars = collect();
    assert_eq!(
        mask, stars,
        "masked glyph output must equal five ASCII stars"
    );
}
#[test]
fn b6_c4_virtual_xml_password_inherited_before_onload() {
    if ACTIVE != ClientProfile::Wrath {
        return;
    }
    let env = WowLuaEnv::new().unwrap();
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("P.xml"),r#"<Ui><EditBox name="B6PasswordTemplate" virtual="true" password="1"><Scripts><OnLoad>self.passwordAtLoad=self:IsPassword()</OnLoad></Scripts></EditBox></Ui>"#).unwrap();
    let toc = dir.path().join("P.toc");
    std::fs::write(&toc, "## Title: Password template\nP.xml\n").unwrap();
    let result = wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).unwrap();
    assert!(result.warnings.is_empty(), "SETUP: {:?}", result.warnings);
    env.exec("local p=CreateFrame('EditBox','B6PasswordInstance',UIParent,'B6PasswordTemplate');assert(p:IsPassword() and p.passwordAtLoad)").unwrap();
}
