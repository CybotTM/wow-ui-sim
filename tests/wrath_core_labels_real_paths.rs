//! Real cached vendor consumers, supplied adversarial state, production dispatch/output.
#![cfg(feature = "client-wrath")]

use std::path::Path;

use sha2::{Digest, Sha256};
use wow_ui_sim::loader::{LoadTiming, create_frame_from_xml, load_addon_from_toc};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::toc::TocFile;
use wow_ui_sim::xml::{FrameChildElement, FrameElement, FrameXml, XmlElement, parse_xml};

const CACHE: &str = "/home/osso/.cache/wow-ui-sim/blizzard-ui/wrath/FrameXML";

fn read_original(name: &str) -> String {
    let path = Path::new(CACHE).join(name);
    let bytes = std::fs::read(&path).expect("SETUP: cached vendor source");
    eprintln!(
        "original={} sha256={:x}",
        path.display(),
        Sha256::digest(&bytes)
    );
    String::from_utf8(bytes).expect("SETUP: original UTF-8")
}

fn assert_no_script_errors(env: &WowLuaEnv) {
    let sim = env.state().borrow();
    assert!(
        sim.lua_errors.is_empty(),
        "SETUP: original script errors: {:?}",
        sim.lua_errors
    );
}

fn exec_original(env: &WowLuaEnv, name: &str) {
    env.loader_env()
        .exec(&read_original(name))
        .expect("SETUP: whole original Lua");
}

fn load_originals(env: &WowLuaEnv, names: &[&str]) {
    for name in names {
        read_original(name);
    }
    // Distinct TOC roots avoid addon-directory deduplication between prerequisite loads.
    let fixture = tempfile::tempdir().unwrap();
    for entry in std::fs::read_dir(CACHE).unwrap() {
        let entry = entry.unwrap();
        std::os::unix::fs::symlink(entry.path(), fixture.path().join(entry.file_name())).unwrap();
    }
    let toc = TocFile::parse(
        fixture.path(),
        &format!(
            "## Interface: 30300\n## AllowLoad: Both\n{}\n",
            names.join("\n")
        ),
    );
    let loaded = load_addon_from_toc(&env.loader_env(), &toc).expect("SETUP: original files");
    assert!(loaded.warnings.is_empty(), "SETUP: {:?}", loaded.warnings);
}

fn read_original_frame(name: &str, wanted: &str) -> FrameXml {
    let ui = parse_xml(&read_original(name)).expect("SETUP: complete original XML");
    ui.elements
        .into_iter()
        .find_map(|element| {
            let frame = match element {
                XmlElement::Frame(frame)
                | XmlElement::Button(frame)
                | XmlElement::EditBox(frame)
                | XmlElement::ScrollingMessageFrame(frame) => frame,
                _ => return None,
            };
            (frame.name.as_deref() == Some(wanted)).then_some(frame)
        })
        .expect("SETUP: original node")
}

fn instantiate(env: &WowLuaEnv, frame: &FrameXml, kind: &str, parent: Option<&str>) {
    create_frame_from_xml(
        &env.loader_env(),
        frame,
        kind,
        parent,
        None,
        None,
        &mut LoadTiming::default(),
    )
    .expect("SETUP: instantiate original node");
}

fn create_autocomplete_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    load_originals(
        &env,
        &[
            "Fonts.xml",
            "FontStyles.xml",
            "UIPanelTemplates.xml",
            "AutoComplete.xml",
        ],
    );
    env.exec("assert(AutoCompleteButton1:GetWidth()==120 and AutoCompleteButton1:GetHeight()==14); assert(AutoCompleteButton1:GetFontString():GetNumPoints()>0)").expect("SETUP: original autocomplete lifecycle");
    assert_no_script_errors(&env);
    env
}

#[test]
fn t1_original_animation_rejects_range_without_mutating_original_uvs() {
    let env = WowLuaEnv::new().unwrap();
    exec_original(&env, "UIParent.lua");
    instantiate(
        &env,
        &read_original_frame("ComboFrame.xml", "ComboPointTemplate"),
        "Frame",
        None,
    );
    env.exec(
        r#"
        local control=CreateFrame('Frame','AnimationControl',UIParent,'ComboPointTemplate')
        local texture=control:GetRegions()
        AnimateTexCoords(texture,2,2,1,1,4,0.2)
        assert(texture.frame==2, 'SETUP: original animation control')
        local fresh=CreateFrame('Frame','AnimationRange',UIParent,'ComboPointTemplate')
        RangeTexture=fresh:GetRegions()
        assert(RangeTexture.frame==nil, 'SETUP: fresh animation metadata')
        OriginalUV={RangeTexture:GetTexCoord()}
        RangeAccepted,RangeError=pcall(AnimateTexCoords,RangeTexture,1,1,10001,1,1,0.2)
    "#,
    )
    .unwrap();
    env.exec(r#"
        assert(not RangeAccepted and type(RangeError)=='string', 'RED T1: original animation must reject range')
        local after={RangeTexture:GetTexCoord()}
        for i=1,8 do assert(after[i]==OriginalUV[i], 'RED T1: rejected animation changed UV') end
    "#).unwrap();
}

#[test]
fn f1_original_restricted_screen_handle_fills_root_not_offset_parent() {
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1600.0, 1200.0);
    exec_original(&env, "RestrictedFrames.lua");
    env.exec(r#"
        OffsetParent=CreateFrame('Frame','ScreenHandleParent',UIParent)
        OffsetParent:SetSize(200,100); OffsetParent:SetPoint('BOTTOMLEFT',UIParent,'BOTTOMLEFT',31,47)
        ScreenChild=CreateFrame('Frame','ScreenHandleChild',OffsetParent)
        ScreenChild:SetSize(15,17)
        ScreenHandle=GetFrameHandle(ScreenChild)
        assert(ScreenHandle and IsFrameHandle(ScreenHandle), 'SETUP: real restricted handle')
        assert(ScreenHandle:GetName()=='ScreenHandleChild' and ScreenHandle:GetWidth()==15, 'SETUP: original handle getter')
        ScreenHandle:SetAllPoints('$screen')
        local x,y,w,h=ScreenChild:GetRect()
        local rx,ry,rw,rh=UIParent:GetRect()
        assert(rw>200 and rh>100, 'SETUP: root geometry')
        assert(ScreenChild:GetParent()==OffsetParent, 'RED F1: parent identity')
        assert(x==rx and y==ry and w==rw and h==rh, 'RED F1: original screen handle must fill root')
    "#).unwrap();
}

fn create_chat_env() -> WowLuaEnv {
    let env = create_autocomplete_env();
    exec_original(&env, "GlobalStrings.lua");
    exec_original(&env, "Constants.lua");
    exec_original(&env, "UIParent.lua");
    load_originals(&env, &["UIDropDownMenu.xml", "ChatFrame.xml"]);
    exec_original(&env, "FloatingChatFrame.lua");
    env.exec(
        r#"
        ChatWindow=CreateFrame('ScrollingMessageFrame','RealPathChat',UIParent)
        ChatWindow:SetSize(430,120); ChatWindow:SetPoint('BOTTOMLEFT',UIParent,'BOTTOMLEFT',32,95)
        ChatWindow.clickAnywhereButton=CreateFrame('Button',nil,ChatWindow)
        -- Supplied chat window colour state normally populated by ChatFrame_OnLoad.
        ChatTypeInfo.SAY.r=1; ChatTypeInfo.SAY.g=1; ChatTypeInfo.SAY.b=1
    "#,
    )
    .unwrap();
    let edit = read_chat_editbox();
    instantiate(&env, &edit, "EditBox", Some("RealPathChat"));
    env.exec(r#"
        ChatBox=RealPathChatEditBox
        assert(ChatBox.chatFrame==ChatWindow and ChatWindow.editBox==ChatBox, 'SETUP: original window links before OnLoad')
        assert(ChatBox:GetAttribute('chatType')=='SAY' and ChatBox:GetScript('OnTextChanged'), 'SETUP: original chat lifecycle')
        ChatBox:Show(); ChatBox:SetFocus()
    "#).unwrap();
    assert_no_script_errors(&env);
    env
}

fn read_chat_editbox() -> FrameXml {
    let floating = read_original_frame("FloatingChatFrame.xml", "FloatingChatFrameTemplate");
    floating
        .children
        .iter()
        .find_map(|child| match child {
            FrameChildElement::Frames(frames) => {
                frames.elements.iter().find_map(|element| match element {
                    FrameElement::EditBox(frame)
                        if frame.name.as_deref() == Some("$parentEditBox") =>
                    {
                        Some(frame)
                    }
                    _ => None,
                })
            }
            _ => None,
        })
        .expect("SETUP: original floating chat editbox")
        .clone()
}

#[test]
fn c5_original_chat_history_keeps_duplicates_and_cycles_via_alt_dispatch() {
    let env = create_chat_env();
    env.exec(r#"
        HistoryBeforeParse={}
        SavedChatTextSet=ChatBox:GetScript('OnTextSet')
        assert(SavedChatTextSet, 'SETUP: original chat OnTextSet')
        -- Observe recalled text before the ORIGINAL callback strips its slash prefix.
        ChatBox:SetScript('OnTextSet',function(self,...)
            local text=self:GetText()
            if string.sub(text,1,#SLASH_SAY1+1)==SLASH_SAY1..' ' then
                HistoryBeforeParse[#HistoryBeforeParse+1]=text
            end
            return SavedChatTextSet(self,...)
        end)
        for _,text in ipairs({'A','A','B'}) do ChatBox:SetText(text); ChatEdit_AddHistory(ChatBox) end
        ChatBox:SetText('')
        ExpectedHistory={'B','A','A','B'}
    "#).unwrap();
    for index in 1..=4 {
        env.send_key_press("ALT-UP", None).unwrap();
        assert_no_script_errors(&env);
        env.exec(&format!(r#"
            assert(HistoryBeforeParse[{index}]==SLASH_SAY1..' '..ExpectedHistory[{index}], 'RED C5: original prefixed history recall {index}')
            assert(ChatBox:GetText()==ExpectedHistory[{index}], 'RED C5: original parsed chat recall {index}: '..ChatBox:GetText())
        "#)).unwrap();
    }
    env.exec("ChatBox:SetScript('OnTextSet',SavedChatTextSet)")
        .unwrap();
}

#[test]
fn arrows_original_chat_ignore_arrows_controls_real_alt_motion() {
    let env = create_chat_env();
    for text in ["a", "b", "c", "d"] {
        env.send_key_press("", Some(text)).unwrap();
    }
    assert_no_script_errors(&env);
    env.exec("assert(ChatBox:GetText()=='abcd' and ChatBox:GetCursorPosition()==4, 'SETUP: real chat text dispatch')").unwrap();
    env.send_key_press("LEFT", None).unwrap();
    env.exec("assert(ChatBox:GetCursorPosition()==4, 'RED arrows: original ignoreArrows blocks plain LEFT')").unwrap();
    env.send_key_press("ALT-LEFT", None).unwrap();
    env.exec(
        "assert(ChatBox:GetCursorPosition()==3, 'RED arrows: original chat ALT-LEFT moves once')",
    )
    .unwrap();
    env.send_key_press("ALT-RIGHT", None).unwrap();
    env.exec("assert(ChatBox:GetCursorPosition()==4 and ChatBox:GetAltArrowKeyMode(), 'RED arrows: original XML mode and ALT-RIGHT')").unwrap();
}

#[cfg(feature = "gui")]
#[test]
fn c4_original_editbox_input_renders_byte_mask_glyphs_and_end_caret() {
    use std::{cell::RefCell, rc::Rc};
    use wow_ui_sim::render::WowFontSystem;
    let env = create_autocomplete_env();
    let fonts = Rc::new(RefCell::new(WowFontSystem::new_without_casc()));
    assert!(
        fonts.borrow().font_system.db().faces().next().is_some(),
        "SETUP: font faces"
    );
    env.set_font_system(Rc::clone(&fonts));
    env.exec(r#"
        function ArrangeOriginal(name)
            local b=CreateFrame('EditBox',name,UIParent,'AutoCompleteEditBoxTemplate')
            b:SetAutoFocus(false); b:SetSize(500,60); b:SetPoint('TOPLEFT',UIParent,'TOPLEFT',20,-20)
            assert(b:SetFont('Fonts\\FRIZQT__.TTF',20)); b:SetJustifyH('LEFT'); b:SetJustifyV('TOP')
            b:SetScript('OnCursorChanged',function(self,x,y,w,h) self.caret={x,y,w,h} end)
            assert(b:GetScript('OnTextChanged')==AutoCompleteEditBox_OnTextChanged, 'SETUP: original text callback')
            b:Show(); return b
        end
        Masked=ArrangeOriginal('RealPathMasked'); Stars=ArrangeOriginal('RealPathStars')
        Masked:SetPassword(true); Masked:SetFocus()
    "#).unwrap();
    for text in ["é", "猫"] {
        env.send_key_press("", Some(text)).unwrap();
    }
    env.fire_on_update(0.016).unwrap();
    env.exec("assert(Masked:GetText()=='é猫' and Masked:GetCursorPosition()==5, 'SETUP: actual UTF-8 input'); Masked:ClearFocus(); Stars:SetFocus()").unwrap();
    for _ in 0..5 {
        env.send_key_press("", Some("*")).unwrap();
    }
    env.fire_on_update(0.016).unwrap();
    assert_no_script_errors(&env);
    env.exec(r#"
        assert(Stars:GetText()=='*****' and Masked.caret and Stars.caret, 'SETUP: star control and real caret output')
    "#).unwrap();
    let caret_matches = env
        .eval::<bool>(
            r#"
        for i=1,4 do
            if math.abs(Masked.caret[i]-Stars.caret[i])>=0.01 then return false end
        end
        return true
    "#,
        )
        .unwrap();
    let mask = render_glyphs(&env, &fonts, "RealPathMasked");
    let stars = render_glyphs(&env, &fonts, "RealPathStars");
    assert!(
        !mask.is_empty() && !stars.is_empty(),
        "SETUP: nonempty real glyph output"
    );
    assert!(
        mask == stars && caret_matches,
        "RED C4: five byte-mask glyphs and end caret; mask_vertices={} star_vertices={} caret_matches={caret_matches}",
        mask.len(),
        stars.len()
    );
}

#[cfg(feature = "gui")]
fn render_glyphs(
    env: &WowLuaEnv,
    fonts: &std::rc::Rc<std::cell::RefCell<wow_ui_sim::render::WowFontSystem>>,
    root: &str,
) -> Vec<([f32; 2], [f32; 2])> {
    use wow_ui_sim::iced_app::{RegistryQuadBatchParams, build_quad_batch_for_registry};
    use wow_ui_sim::render::{GlyphAtlas, shader::GLYPH_ATLAS_TEX_INDEX};
    let mut glyphs = GlyphAtlas::new();
    let mut sim = env.state().borrow_mut();
    sim.ensure_layout_rects();
    let _ = sim.get_strata_buckets();
    let mut fonts = fonts.borrow_mut();
    let batch = build_quad_batch_for_registry(
        RegistryQuadBatchParams::new(
            &sim.widgets,
            (1600.0, 1200.0),
            sim.strata_buckets.as_ref().unwrap(),
        )
        .root_name(Some(root))
        .text_ctx(Some((&mut fonts, &mut glyphs))),
    );
    batch
        .vertices
        .iter()
        .filter(|vertex| vertex.tex_index == GLYPH_ATLAS_TEX_INDEX)
        .map(|vertex| (vertex.position, vertex.tex_coords))
        .collect()
}

#[test]
fn x1_original_clean_template_survives_exact_tainted_replacement_on_replay() {
    let env = create_autocomplete_env();
    let dir = tempfile::tempdir().unwrap();
    let toc = TocFile::parse(dir.path(), "## Interface: 30300\nReplacement.xml\n");
    std::fs::write(dir.path().join("Replacement.xml"),r#"<Ui><Button name="AutoCompleteButtonTemplate" virtual="true"><Size x="222" y="29"/></Button></Ui>"#).unwrap();
    load_addon_from_toc(&env.loader_env(), &toc).unwrap();
    let original = read_original_frame("AutoComplete.xml", "AutoCompleteBox");
    let button = original
        .children
        .iter()
        .find_map(|child| match child {
            FrameChildElement::Frames(frames) => {
                frames.elements.iter().find_map(|element| match element {
                    FrameElement::Button(frame)
                        if frame.name.as_deref() == Some("AutoCompleteButton1") =>
                    {
                        Some(frame)
                    }
                    _ => None,
                })
            }
            _ => None,
        })
        .expect("SETUP: original inheriting node");
    instantiate(&env, button, "Button", Some("AutoCompleteBox"));
    env.exec(r#"
        assert(AutoCompleteButton1:GetWidth()==120 and AutoCompleteButton1:GetHeight()==14, 'RED X1: original clean template must win exact tainted replacement')
        local text=AutoCompleteButton1:GetFontString()
        local originalAnchor=false
        for i=1,text:GetNumPoints() do
            local p,r,rp,x,y=text:GetPoint(i)
            if p=='LEFT' and r==AutoCompleteButton1 and rp=='LEFT' and x==15 and y==0 then originalAnchor=true end
        end
        assert(originalAnchor, 'RED X1: original inherited font anchor')
    "#).unwrap();
}

#[test]
fn x2_original_xml_continues_after_missing_external_script() {
    let env = WowLuaEnv::new().unwrap();
    load_originals(
        &env,
        &["Fonts.xml", "FontStyles.xml", "UIPanelTemplates.xml"],
    );
    exec_original(&env, "AutoComplete.lua");
    let dir = tempfile::tempdir().unwrap();
    let xml = read_original("AutoComplete.xml");
    let fixture = dir.path().join("AutoComplete.xml");
    std::fs::write(&fixture, xml.as_bytes()).unwrap();
    assert_eq!(
        std::fs::read(&fixture).unwrap(),
        std::fs::read(Path::new(CACHE).join("AutoComplete.xml")).unwrap()
    );
    assert!(
        !dir.path().join("AutoComplete.lua").exists(),
        "SETUP: only external child unavailable"
    );
    let toc = TocFile::parse(
        dir.path(),
        "## Interface: 30300\n## AllowLoad: Both\nAutoComplete.xml\n",
    );
    let outcome = load_addon_from_toc(&env.loader_env(), &toc);
    eprintln!("controlled missing child load result={outcome:?}");
    env.exec(r#"
        assert(AutoCompleteBox and AutoCompleteButton1, 'RED X2: original later siblings must survive child I/O failure')
        assert(AutoCompleteButton1:GetWidth()==120 and AutoCompleteButton1:GetHeight()==14, 'RED X2: original sibling size')
        assert(AutoCompleteButton1:GetFontString():GetNumPoints()>0, 'RED X2: original sibling lifecycle')
    "#).unwrap();
}

#[test]
fn x2_original_pvp_xml_continues_after_missing_template_include() {
    let env = WowLuaEnv::new().unwrap();
    exec_original(&env, "GlobalStrings.lua");
    exec_original(&env, "Constants.lua");
    exec_original(&env, "FriendsFrame.lua");
    load_originals(
        &env,
        &[
            "Fonts.xml",
            "FontStyles.xml",
            "UIPanelTemplates.xml",
            "UIDropDownMenu.xml",
            "PVPFrameTemplates.xml",
        ],
    );
    assert_no_script_errors(&env);
    let dir = tempfile::tempdir().unwrap();
    for name in ["PVPFrame.xml", "PVPFrame.lua"] {
        let source = read_original(name);
        let path = dir.path().join(name);
        std::fs::write(&path, source.as_bytes()).unwrap();
        assert_eq!(
            std::fs::read(&path).unwrap(),
            std::fs::read(Path::new(CACHE).join(name)).unwrap()
        );
    }
    assert!(
        !dir.path().join("PVPFrameTemplates.xml").exists(),
        "SETUP: only original template include unavailable"
    );
    let toc = TocFile::parse(
        dir.path(),
        "## Interface: 30300\n## AllowLoad: Both\nPVPFrame.xml\n",
    );
    let outcome = load_addon_from_toc(&env.loader_env(), &toc);
    eprintln!("controlled missing include load result={outcome:?}");
    env.exec(r#"
        assert(PVPParentFrame, 'RED X2 include: original parent sibling must survive child I/O failure')
        assert(PVPParentFrame:GetWidth()==384 and PVPParentFrame:GetHeight()==512, 'RED X2 include: original sibling geometry')
    "#).unwrap();
    assert_no_script_errors(&env);
}

#[test]
fn g1_original_action_event_observes_real_public_nil_tuple_without_flashing() {
    let env = WowLuaEnv::new().unwrap();
    load_originals(
        &env,
        &[
            "Fonts.xml",
            "FontStyles.xml",
            "Cooldown.xml",
            "ActionButtonTemplate.xml",
        ],
    );
    exec_original(&env, "ActionButton.lua");
    {
        let mut sim = env.state().borrow_mut();
        for slot in [1, 7] {
            sim.action_bars.remove(&slot);
            sim.action_macros.remove(&slot);
            sim.action_outfits.remove(&slot);
        }
        sim.action_bars.insert(7, 1_000_001);
    }
    env.exec(r#"
        ObservedAction=CreateFrame('CheckButton','RealPathAction',UIParent,'ActionButtonTemplate')
        assert(RealPathActionFlash and not RealPathActionFlash:IsShown(), 'SETUP: original flash region')
        ObservedAction.flashing=0; ObservedAction:SetScript('OnEvent',ActionButton_OnEvent)
        ObservedAction:RegisterEvent('PLAYER_ENTER_COMBAT'); ObservedAction:RegisterEvent('PLAYER_LEAVE_COMBAT')
        SavedAttackBinding=IsAttackAction; AttackResults={}
        local function pack(...) return {n=select('#',...),...} end
        IsAttackAction=function(...)
            local result=pack(SavedAttackBinding(...))
            AttackResults[#AttackResults+1]=result
            return unpack(result,1,result.n)
        end
    "#).unwrap();
    for slot in [1, 7] {
        env.exec(&format!("ObservedAction.action={slot}")).unwrap();
        for event in ["PLAYER_ENTER_COMBAT", "PLAYER_LEAVE_COMBAT"] {
            env.fire_event(event).unwrap();
        }
    }
    assert_no_script_errors(&env);
    env.exec(r#"
        IsAttackAction=SavedAttackBinding
        assert(#AttackResults==4, 'SETUP: original event dispatch reached real public binding')
        for _,result in ipairs(AttackResults) do
            assert(result.n==1 and result[1]==nil, 'RED G1: original event must observe one nil, not false')
        end
        assert(ObservedAction.flashing==0 and not RealPathActionFlash:IsShown(), 'RED G1: no unintended original flashing')
    "#).unwrap();
}
