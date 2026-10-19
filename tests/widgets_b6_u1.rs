use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;
fn wrath() -> bool {
    ACTIVE == ClientProfile::Wrath
}
#[path = "common/wrath_label_fixtures.rs"]
mod originals;
use originals::{lfr_env, tooltip_env};

#[test]
fn b6_u1_original_lfr_row_attachment() {
    if !wrath() {
        return;
    }
    let env = lfr_env();
    env.exec(r#"
        assert(B6Tip:NumLines()==5,'texture calls must not add rows')
        assert(B6Tip:GetLeftLine(2):GetText()=='Leader')
        assert(B6Tip:GetLeftLine(3):GetText()=='2 members')
        assert(B6TipTexture10:GetWidth()>0 and B6TipTexture10:GetHeight()>0)
        assert(B6TipTexture1:GetWidth()==B6TipTexture10:GetWidth())
        assert(B6TipTexture1:GetHeight()==B6TipTexture10:GetHeight())
        local tlx,tly,blx,bly,trx,try,brx,bry=B6TipTexture1:GetTexCoord()
        assert(tlx==0 and tly==0 and blx==0 and bly==1 and trx==0.25 and try==0 and brx==0.25 and bry==1,'UV crop retained')
    "#).unwrap();
}
#[test]
fn b6_u1_fresh_empty_path_keeps_layout_without_image() {
    if !wrath() {
        return;
    }
    let env = tooltip_env();
    env.exec(
        r#"
        B6Tip:AddLine('header'); B6Tip:AddLine('body'); B6Tip:Show()
        B6TipTexture1:SetSize(12,12)
        B6WidthBefore=B6Tip:GetWidth(); B6HeightBefore=B6Tip:GetHeight()
        B6Tip:AddTexture('')
        assert(B6Tip:NumLines()==2)
        assert(B6TipTexture1:GetTexture()==nil,'fresh empty path has no handle')
        -- Show/measure continuation is required for the width assertion.
    "#,
    )
    .unwrap();
    // See cases.md: glyph-backed measured width grows by the slot width + converted gap,
    // left-string row height stays unchanged, emitted image requests remain empty.
}
#[test]
fn b6_u1_no_slot_and_too_few_rows_do_not_insert() {
    if !wrath() {
        return;
    }
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local tip=CreateFrame('GameTooltip','B6Bare',UIParent)
        tip:AddTexture(''); assert(tip:NumLines()==0)
        tip:AddLine('header'); tip:AddTexture(''); assert(tip:NumLines()==1)
        tip:AddLine('body'); tip:AddTexture(''); assert(tip:NumLines()==2)
        assert(B6BareTexture1==nil)
    "#,
    )
    .unwrap();
}

#[test]
fn b6_u1_numeric_legacy_input_attaches_not_new_row() {
    if !wrath() {
        return;
    }
    let env = tooltip_env();
    env.exec(
        r#"
        B6Tip:AddLine('header'); B6Tip:AddLine('body')
        B6Tip:AddTexture(136243)
        assert(B6Tip:NumLines()==2,'numeric file identifier must not add a row')
    "#,
    )
    .unwrap();
}
#[cfg(feature = "gui")]
fn b6_measure_tip(env: &WowLuaEnv, fonts: &mut wow_ui_sim::render::WowFontSystem) -> (f64, f64) {
    env.exec("B6Tip:Show()").unwrap();
    wow_ui_sim::iced_app::tooltip::update_tooltip_sizes(&mut env.state().borrow_mut(), fonts);
    env.eval("return B6Tip:GetWidth(),B6Tip:GetHeight()")
        .unwrap()
}
#[cfg(feature = "gui")]
#[test]
fn b6_u1_slot_size_blank_spacing_and_ten_slot_cap() {
    if !wrath() {
        return;
    }
    let env = tooltip_env();
    let mut fonts = wow_ui_sim::render::WowFontSystem::new_without_casc();
    assert!(
        fonts.font_system.db().faces().next().is_some(),
        "SETUP: fonts required"
    );
    // Every body line is longer than the header; space additions therefore
    // change the max body width rather than being masked by header width.
    env.exec("B6Tip:AddLine('h'); B6Tip:AddLine(string.rep('W',30)); B6TipTexture1:SetSize(18,7); B6SlotWidth=B6TipTexture1:GetWidth(); B6SlotHeight=B6TipTexture1:GetHeight()")
        .unwrap();
    let before = b6_measure_tip(&env, &mut fonts);
    env.exec("B6Tip:AddTexture('')").unwrap();
    let after = b6_measure_tip(&env, &mut fonts);
    assert!(
        after.0 > before.0,
        "fresh empty attachment must reserve width"
    );
    assert!(
        (after.1 - before.1).abs() < 0.01,
        "slot height must not replace left height"
    );
    env.exec("assert(B6TipTexture1:GetWidth()==B6SlotWidth and B6TipTexture1:GetHeight()==B6SlotHeight); assert(B6TipTexture1:GetTexture()==nil)").unwrap();
    for index in 2..=10 {
        env.exec(&format!(
            "B6Tip:AddLine(string.rep('W',{})); B6Tip:AddTexture('')",
            30 + index * 2
        ))
        .unwrap();
    }
    env.exec("B6Tip:AddLine(string.rep('W',60))").unwrap();
    let cap_before = b6_measure_tip(&env, &mut fonts);
    env.exec("B6Tip:AddTexture('')").unwrap();
    let cap_after = b6_measure_tip(&env, &mut fonts);
    assert_eq!(cap_before, cap_after, "11th attachment is a no-op at cap");
    env.exec("assert(B6Tip:NumLines()==12 and B6TipTexture11==nil)")
        .unwrap();
}

#[test]
fn b6_u1_previous_most_recent_texture_aligns_captions_not_icon_left_edges() {
    if !wrath() {
        return;
    }
    let env = tooltip_env();
    env.exec(
        r#"
        B6Tip:AddLine('header');B6Tip:AddLine('caption two')
        B6TipTexture1:SetSize(20,12);B6TipTexture2:SetSize(30,12)
        B6Tip:AddTexture('');B6Tip:Show()
    "#,
    )
    .unwrap();
    env.state().borrow_mut().ensure_layout_rects();
    env.exec(
        r#"
        B6Row2X=B6TipTextLeft2:GetLeft()
        B6Icon1Left=B6TipTexture1:GetLeft()
        B6Gap=B6Row2X-B6TipTexture1:GetRight()
        assert(B6Gap>0,'icon right must be left of text')
        B6Tip:AddLine('caption three');B6Tip:AddTexture('');B6Tip:Show()
    "#,
    )
    .unwrap();
    env.state().borrow_mut().ensure_layout_rects();
    env.exec(r#"
        local function near(a,b) assert(math.abs(a-b)<0.01) end
        near(B6TipTextLeft2:GetLeft(),B6Row2X)
        near(B6TipTextLeft3:GetLeft(),B6Row2X)
        near(B6TipTextLeft3:GetLeft()-B6TipTexture2:GetRight(),B6Gap)
        assert(math.abs(B6TipTexture2:GetLeft()-B6Icon1Left)>0.01,'unequal icons have different left edges')
        -- Preanchored NEXT template slot baseline uses CURRENT width, not old width.
        near(B6TipTextLeft4:GetLeft(),B6TipTextLeft3:GetLeft()-B6TipTexture2:GetWidth()-B6Gap)
        B6Tip:AddLine('no attachment four')
        B6Tip:AddLine('caption five');B6TipTexture3:SetWidth(16);B6Tip:AddTexture('');B6Tip:Show()
    "#).unwrap();
    env.state().borrow_mut().ensure_layout_rects();
    env.exec(
        r#"
        local offset=B6TipTextLeft5:GetLeft()-B6TipTextLeft4:GetLeft()
        assert(math.abs(offset-B6TipTexture3:GetWidth()-B6Gap)<0.01,
            'nonadjacent most-recent texture uses width+gap offset')
    "#,
    )
    .unwrap();
}
