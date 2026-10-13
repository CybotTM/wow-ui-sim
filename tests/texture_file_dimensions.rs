//! Public actual-file dimension contract. No renderer, GPU, or CASC fixture required.

use std::fs;

use tempfile::TempDir;
use wow_ui_sim::lua_api::WowLuaEnv;

const WIDE_PATH: &str = r"Interface\AddOns\FileDimensionProbe\wide";
const TALL_PATH: &str = "Interface/AddOns/FileDimensionProbe/tall.webp";
const MISSING_PATH: &str = "Interface/AddOns/FileDimensionProbe/missing";

struct Fixture {
    env: WowLuaEnv,
    _empty_root: TempDir,
    _image_root: TempDir,
}

impl Fixture {
    fn new() -> Self {
        let empty_root = TempDir::new().unwrap();
        let image_root = TempDir::new().unwrap();
        let directory = image_root.path().join("FileDimensionProbe");
        fs::create_dir_all(&directory).unwrap();
        for (name, width, height) in [("wide", 7, 5), ("tall", 3, 11)] {
            image::RgbaImage::from_pixel(width, height, image::Rgba([10, 20, 30, 255]))
                .save(directory.join(format!("{name}.webp")))
                .unwrap();
        }
        fs::write(directory.join("invalid.webp"), b"not an image").unwrap();

        let env = WowLuaEnv::new().unwrap();
        env.state().borrow_mut().addon_base_paths = vec![
            empty_root.path().to_path_buf(),
            image_root.path().to_path_buf(),
        ];
        env.exec(
            r#"
            local parent = CreateFrame('Frame', nil, UIParent)
            FileDimensionTexture = parent:CreateTexture('FileDimensionTexture')
            FileDimensionTexture:SetSize(101, 103)
            "#,
        )
        .unwrap();
        Self {
            env,
            _empty_root: empty_root,
            _image_root: image_root,
        }
    }

    fn set_texture(&self, path: &str) {
        self.env
            .exec(&format!("FileDimensionTexture:SetTexture([[{path}]])"))
            .unwrap();
    }

    fn assert_dimensions(&self, expected: (i32, i32)) {
        let dimensions: (i32, i32) = self
            .env
            .eval(
                "return FileDimensionTexture:GetFileWidth(), FileDimensionTexture:GetFileHeight()",
            )
            .expect("both public Texture file-dimension methods must be callable");
        assert_eq!(dimensions, expected);
    }
}

#[test]
fn texture_get_file_width_reads_actual_source_not_widget_width() {
    let fixture = Fixture::new();
    for (path, expected) in [(WIDE_PATH, 7), (TALL_PATH, 3)] {
        fixture.set_texture(path);
        let (file_width, widget_width): (i32, i32) = fixture
            .env
            .eval("return FileDimensionTexture:GetFileWidth(), FileDimensionTexture:GetWidth()")
            .expect("GetFileWidth must be published on the public Texture object");
        assert_eq!(file_width, expected);
        assert_eq!(widget_width, 101);
    }
    fixture
        .env
        .exec("FileDimensionTexture:SetWidth(211)")
        .unwrap();
    let (file_width, widget_width): (i32, i32) = fixture
        .env
        .eval("return FileDimensionTexture:GetFileWidth(), FileDimensionTexture:GetWidth()")
        .unwrap();
    assert_eq!((file_width, widget_width), (3, 211));
}

#[test]
fn texture_get_file_height_reads_actual_source_not_widget_height() {
    let fixture = Fixture::new();
    for (path, expected) in [(WIDE_PATH, 5), (TALL_PATH, 11)] {
        fixture.set_texture(path);
        let (file_height, widget_height): (i32, i32) = fixture
            .env
            .eval("return FileDimensionTexture:GetFileHeight(), FileDimensionTexture:GetHeight()")
            .expect("GetFileHeight must be published on the public Texture object");
        assert_eq!(file_height, expected);
        assert_eq!(widget_height, 103);
    }
    fixture
        .env
        .exec("FileDimensionTexture:SetHeight(223)")
        .unwrap();
    let (file_height, widget_height): (i32, i32) = fixture
        .env
        .eval("return FileDimensionTexture:GetFileHeight(), FileDimensionTexture:GetHeight()")
        .unwrap();
    assert_eq!((file_height, widget_height), (11, 223));
}

#[test]
fn texture_file_dimensions_follow_source_changes_and_clearing() {
    let fixture = Fixture::new();
    fixture.set_texture(WIDE_PATH);
    fixture.assert_dimensions((7, 5));
    fixture.set_texture(TALL_PATH);
    fixture.assert_dimensions((3, 11));
    fixture.set_texture(MISSING_PATH);
    fixture.assert_dimensions((0, 0));
    fixture.set_texture(WIDE_PATH);
    fixture.assert_dimensions((7, 5));
    for clear in ["nil", "''", ""] {
        fixture
            .env
            .exec(&format!("FileDimensionTexture:SetTexture({clear})"))
            .unwrap();
        fixture.assert_dimensions((0, 0));
        fixture.set_texture(TALL_PATH);
        fixture.assert_dimensions((3, 11));
    }
}

#[test]
fn texture_file_dimensions_are_zero_without_accessible_image_metadata() {
    let fixture = Fixture::new();
    fixture.assert_dimensions((0, 0));
    for path in [
        MISSING_PATH,
        "Interface/AddOns/FileDimensionProbe/invalid.webp",
    ] {
        fixture.set_texture(path);
        fixture.assert_dimensions((0, 0));
    }
    fixture.set_texture(WIDE_PATH);
    fixture.assert_dimensions((7, 5));
    fixture
        .env
        .exec("FileDimensionTexture:SetColorTexture(1, 0, 0, 1)")
        .unwrap();
    fixture.assert_dimensions((0, 0));
}

#[test]
fn texture_file_dimensions_ignore_uv_crop_and_tiling() {
    let fixture = Fixture::new();
    fixture.set_texture(WIDE_PATH);
    fixture.assert_dimensions((7, 5));
    fixture
        .env
        .exec(
            r#"
            FileDimensionTexture:SetTexCoord(0.25, 0.75, 0.2, 0.6)
            FileDimensionTexture:SetHorizTile(true)
            FileDimensionTexture:SetVertTile(true)
            FileDimensionTexture:SetSize(401, 409)
            "#,
        )
        .unwrap();
    let controls: (f64, f64, f64, f64, i32, i32, bool, bool) = fixture
        .env
        .eval(
            r#"
            local left, top, _, bottom, right = FileDimensionTexture:GetTexCoord()
            return left, right, top, bottom,
                FileDimensionTexture:GetWidth(), FileDimensionTexture:GetHeight(),
                FileDimensionTexture:GetHorizTile(), FileDimensionTexture:GetVertTile()
            "#,
        )
        .unwrap();
    let (left, right, top, bottom, width, height, horizontal, vertical) = controls;
    for (actual, expected) in [(left, 0.25), (right, 0.75), (top, 0.2), (bottom, 0.6)] {
        assert!((actual - expected).abs() < 0.0001);
    }
    assert_eq!(
        (width, height, horizontal, vertical),
        (401, 409, true, true)
    );
    fixture.assert_dimensions((7, 5));
}

#[test]
fn texture_file_dimensions_obey_existing_secret_texture_read_policy() {
    let fixture = Fixture::new();
    fixture.set_texture(WIDE_PATH);
    // Set the existing backing flag directly: this tests read policy, not secret ingestion.
    {
        let mut state = fixture.env.state().borrow_mut();
        let id = state
            .widgets
            .get_id_by_name("FileDimensionTexture")
            .unwrap();
        state.widgets.get_mut(id).unwrap().secret_texture = true;
    }
    fixture.assert_dimensions((7, 5));
    fixture
        .env
        .exec(
            r#"
            local function addon()
                for _, method in ipairs({'GetTexture', 'GetFileWidth', 'GetFileHeight'}) do
                    assert(type(FileDimensionTexture[method]) == 'function', method .. ' missing')
                    local ok, err = pcall(FileDimensionTexture[method], FileDimensionTexture)
                    assert(not ok, method .. ' must reject a tainted caller')
                    assert(string.find(tostring(err),
                        'secret-origin widget readout requires an untainted caller', 1, true),
                        method .. ' must reject through existing texture read policy')
                end
            end
            debug.setobjecttaint(addon, 'FileDimensionProbe')
            addon()
            "#,
        )
        .unwrap();
    fixture.assert_dimensions((7, 5));
    fixture.set_texture(TALL_PATH);
    fixture
        .env
        .exec(
            r#"
            local function addon()
                assert(FileDimensionTexture:GetFileWidth() == 3)
                assert(FileDimensionTexture:GetFileHeight() == 11)
            end
            debug.setobjecttaint(addon, 'FileDimensionProbe')
            addon()
            "#,
        )
        .unwrap();
}

#[test]
fn texture_file_dimension_lookup_is_texture_only_and_preserves_world_frame_scroll() {
    let fixture = Fixture::new();
    fixture.set_texture(WIDE_PATH);
    fixture
        .env
        .exec(
            r#"
            local methods = {'GetFileWidth', 'GetFileHeight'}
            local function assert_absent(object, label)
                local index = getmetatable(object).__index
                for _, method in ipairs(methods) do
                    assert(object[method] == nil, label .. ':' .. method .. ' leaked in lookup')
                    assert(index[method] == nil, label .. ':' .. method .. ' leaked in index')
                end
            end
            local function assert_world_scroll()
                assert(type(WorldFrame.GetVerticalScroll) == 'function')
                assert(WorldFrame:GetVerticalScroll() == 0)
                assert(type(WorldFrame.GetScrollOffset) == 'function')
                assert(WorldFrame:GetScrollOffset() == 0)
            end
            assert_absent(WorldFrame, 'WorldFrame')
            assert_world_scroll()
            for _, kind in ipairs({
                'Frame', 'Button', 'EditBox', 'ScrollFrame', 'Slider', 'CheckButton',
                'StatusBar', 'Cooldown', 'Model', 'ModelScene', 'PlayerModel',
                'ColorSelect', 'MessageFrame', 'SimpleHTML', 'GameTooltip', 'Minimap',
            }) do
                assert_absent(CreateFrame(kind), kind)
            end
            local parent = CreateFrame('Frame')
            assert_absent(parent:CreateFontString(), 'FontString')
            assert_absent(parent:CreateLine(), 'Line')
            local texture = FileDimensionTexture
            local group = texture:CreateAnimationGroup()
            assert_absent(group, 'AnimationGroup')
            assert_absent(group:CreateAnimation(), 'Animation')
            assert_absent(group:CreateAnimation('VertexColor'), 'VertexColor')
            assert_absent(WorldFrame, 'WorldFrame after Frame construction')
            assert_world_scroll()
            local index = getmetatable(texture).__index
            assert(type(index.GetFileWidth) == 'function')
            assert(type(index.GetFileHeight) == 'function')
            assert(index.GetFileWidth(texture) == 7)
            assert(index.GetFileHeight(texture) == 5)
            "#,
        )
        .unwrap();
    fixture.assert_dimensions((7, 5));
}
