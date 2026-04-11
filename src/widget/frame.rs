//! Frame widget - the base container for UI elements.

pub use super::frame_types::*;
use super::{Anchor, AnchorPoint, WidgetType, next_widget_id};
use crate::BlendMode;
use crate::atlas::NineSliceAtlasInfo;
use std::collections::{BTreeSet, HashMap, HashSet};

/// A Frame is the base widget type in WoW's UI system.
#[derive(Debug)]
pub struct Frame {
    /// Unique widget ID.
    pub id: u64,
    /// Widget type.
    pub widget_type: WidgetType,
    /// Original type name for GetObjectType() when mapped to a generic WidgetType.
    /// E.g., "ArchaeologyDigSiteFrame" maps to WidgetType::Frame but GetObjectType
    /// should still return "ArchaeologyDigSiteFrame".
    pub object_type_name: Option<String>,
    /// Global name (optional).
    pub name: Option<String>,
    /// Parent widget ID.
    pub parent_id: Option<u64>,
    /// Child widget IDs.
    pub children: Vec<u64>,
    /// Width in pixels.
    pub width: f32,
    /// Whether width was set by text auto-sizing (vs explicit SetWidth/anchors).
    pub width_is_text_auto: bool,
    /// Height in pixels.
    pub height: f32,
    /// Anchors defining position.
    pub anchors: Vec<Anchor>,
    pub visible: bool,
    pub show_hide_depth: u16, // reentry depth for Show/Hide mutual recursion limit
    pub collapses_layout: bool,
    /// Events this frame is registered to receive.
    pub registered_events: HashSet<String>,
    /// Frame level (draw order within strata).
    pub frame_level: i32,
    /// Raise/Lower order offset within same strata+level siblings.
    /// Raise() increments, Lower() decrements. Does not affect frame_level.
    pub raise_order: i32,
    /// Whether frame level was explicitly set (not inherited from parent).
    pub has_fixed_frame_level: bool,
    /// Optional level offset from parent (from XML frameLevel attribute).
    /// When set, propagate_strata_level uses parent_level + offset instead
    /// of the default parent_level + 1.
    pub frame_level_offset: Option<i32>,
    /// Frame strata (major draw order).
    pub frame_strata: FrameStrata,
    /// Whether frame strata was explicitly set (not inherited from parent).
    pub has_fixed_frame_strata: bool,
    /// Whether this frame locally flattens descendant render layers.
    pub flattens_render_layers: bool,
    /// Top-level frame: auto-raised above siblings when shown.
    pub toplevel: bool,
    /// Alpha transparency (0.0 - 1.0).
    pub alpha: f32,
    /// Whether this frame ignores its parent's effective alpha.
    pub ignore_parent_alpha: bool,
    /// Effective alpha (product of all ancestor alphas × own alpha).
    /// Updated eagerly when visibility or alpha changes. 0.0 when any ancestor is hidden.
    pub effective_alpha: f32,
    /// Additive animation translation offset (from Animation Translation, not anchors).
    pub anim_offset_x: f32,
    /// Additive animation translation offset (from Animation Translation, not anchors).
    pub anim_offset_y: f32,
    /// Scale factor (affects visible size; default 1.0).
    pub scale: f32,
    /// Whether this frame ignores its parent's effective scale.
    pub ignore_parent_scale: bool,
    /// Effective scale (product of all ancestor scales × own scale).
    /// Updated eagerly when scale changes or frame is reparented.
    pub effective_scale: f32,
    /// Whether mouse is enabled.
    pub mouse_enabled: bool,
    /// Whether mouse wheel events are enabled for this frame.
    pub mouse_wheel_enabled: bool,
    /// Whether hover/wheel motion scripts should still run while the frame is disabled.
    pub motion_scripts_while_disabled: bool,
    /// Whether gamepad button input is enabled for this frame.
    pub gamepad_button_enabled: bool,
    /// Whether gamepad stick input is enabled for this frame.
    pub gamepad_stick_enabled: bool,
    /// Whether mouse clicks propagate to parent frames.
    pub propagate_mouse_clicks: bool,
    /// Whether mouse motion propagates to parent frames.
    pub propagate_mouse_motion: bool,
    /// Whether hyperlinks propagate to parent frames.
    pub propagate_hyperlinks_to_parent: bool,
    /// Mouse buttons that should click through this frame to underlying targets.
    pub pass_through_buttons: HashSet<String>,
    /// Hit rect insets (left, right, top, bottom) — shrinks the clickable area.
    pub hit_rect_insets: (f32, f32, f32, f32),
    /// Clamp rect insets (left, right, top, bottom) used by clamped dragging.
    pub clamp_rect_insets: (f32, f32, f32, f32),
    /// Whether keyboard input is enabled for this frame.
    pub keyboard_enabled: bool,
    /// Whether keyboard input propagates to parent frames.
    pub propagate_keyboard_input: bool,
    /// Texture path (for Texture widgets).
    pub texture: Option<String>,
    pub texture_file_data_id: Option<i64>,
    /// Solid color texture (from SetColorTexture).
    pub color_texture: Option<Color>,
    /// Vertex color for textures (tinting).
    pub vertex_color: Option<Color>,
    /// Vertex color gradient for textures.
    pub gradient: Option<Gradient>,
    /// Alpha gradients keyed by surface index.
    pub alpha_gradients: HashMap<i32, AlphaGradient>,
    /// Text content (for FontString widgets).
    pub text: Option<String>,
    /// Pre-stripped text (WoW markup removed). Updated when `text` is set.
    pub text_stripped: Option<String>,
    /// Title text (for DefaultPanelTemplate frames).
    pub title: Option<String>,
    /// Text color for FontStrings.
    pub text_color: Color,
    /// Shadow color for FontStrings (defaults to transparent = no shadow).
    pub shadow_color: Color,
    /// Shadow offset (x, y) in pixels for FontStrings.
    pub shadow_offset: (f32, f32),
    /// Font name (for FontString widgets).
    pub font: Option<String>,
    /// Font size (for FontString widgets).
    pub font_size: f32,
    /// Font outline style (OUTLINE, THICKOUTLINE).
    pub font_outline: TextOutline,
    /// Horizontal text justification (LEFT, CENTER, RIGHT).
    pub justify_h: TextJustify,
    /// Vertical text justification (TOP, MIDDLE, BOTTOM).
    pub justify_v: TextJustify,
    /// Named attributes (for secure frames, unit frames, etc.).
    pub attributes: HashMap<String, AttributeValue>,
    /// Backdrop configuration.
    pub backdrop: Backdrop,
    /// Named child references (e.g., "Text" -> FontString child ID for CheckButtons).
    pub children_keys: HashMap<String, u64>,
    /// Key under which this frame is stored on its parent (e.g., "NormalTexture").
    pub parent_key: Option<String>,
    pub movable: bool,
    pub resizable: bool,
    pub resize_bounds_min: (f32, f32),
    pub resize_bounds_max: Option<(f32, f32)>,
    pub user_placed: bool,
    pub dont_save_position: bool,
    pub registered_drag_buttons: HashSet<String>,
    pub highlight_locked: bool,
    pub ignoring_children_for_bounds: bool,
    pub clamped_to_screen: bool,
    pub is_moving: bool,
    /// Whether text should word-wrap (for FontString widgets).
    pub word_wrap: bool,
    /// Maximum number of lines to display (0 = unlimited, for FontString widgets).
    pub max_lines: u32,
    /// Text scale factor (for FontString widgets).
    pub text_scale: f64,
    /// Normal texture path (for Button widgets).
    pub normal_texture: Option<String>,
    /// Normal texture UV coords (left, right, top, bottom) for atlas-based buttons.
    pub normal_tex_coords: Option<(f32, f32, f32, f32)>,
    /// Pushed texture path (for Button widgets).
    pub pushed_texture: Option<String>,
    /// Pushed texture UV coords for atlas-based buttons.
    pub pushed_tex_coords: Option<(f32, f32, f32, f32)>,
    /// Highlight texture path (for Button widgets).
    pub highlight_texture: Option<String>,
    /// Highlight texture UV coords for atlas-based buttons.
    pub highlight_tex_coords: Option<(f32, f32, f32, f32)>,
    /// Disabled texture path (for Button widgets).
    pub disabled_texture: Option<String>,
    /// Disabled texture UV coords for atlas-based buttons.
    pub disabled_tex_coords: Option<(f32, f32, f32, f32)>,
    /// Checked texture path (for CheckButton widgets).
    pub checked_texture: Option<String>,
    /// Checked texture UV coords for atlas-based check buttons.
    pub checked_tex_coords: Option<(f32, f32, f32, f32)>,
    /// Disabled-checked texture path (for CheckButton widgets).
    pub disabled_checked_texture: Option<String>,
    /// Disabled-checked texture UV coords for atlas-based check buttons.
    pub disabled_checked_tex_coords: Option<(f32, f32, f32, f32)>,
    /// Left cap texture for 3-slice buttons.
    pub left_texture: Option<String>,
    /// Middle (stretchable) texture for 3-slice buttons.
    pub middle_texture: Option<String>,
    /// Right cap texture for 3-slice buttons.
    pub right_texture: Option<String>,
    /// Draw layer for regions (textures/fontstrings).
    pub draw_layer: DrawLayer,
    /// Sub-layer within draw layer (for fine-grained ordering).
    pub draw_sub_layer: i32,
    /// Parent-controlled disabled region layers.
    pub disabled_draw_layers: BTreeSet<DrawLayer>,
    /// Tile texture horizontally.
    pub horiz_tile: bool,
    /// Tile texture vertically.
    pub vert_tile: bool,
    /// Texture coordinates (left, right, top, bottom) — final UV coords used for rendering.
    pub tex_coords: Option<(f32, f32, f32, f32)>,
    /// Raw 8-arg SetTexCoord values: [ULx, ULy, LLx, LLy, URx, URy, LRx, LRy].
    /// Stored when 8-arg SetTexCoord is called, used by tiling code to detect
    /// UV-based repeat tiling (BackdropTemplateMixin) where values >1.0 encode repeat counts.
    pub tex_coords_quad: Option<[f32; 8]>,
    /// Atlas base texture coordinates — the sub-region on the texture file.
    /// SetTexCoord remaps relative to these when an atlas is active.
    pub atlas_tex_coords: Option<(f32, f32, f32, f32)>,
    /// Atlas name (if set via SetAtlas).
    pub atlas: Option<String>,
    /// NineSlice layout type (e.g., "PortraitFrameTemplate", "ButtonFrameTemplateNoPortrait").
    pub nine_slice_layout: Option<String>,
    /// Nine-slice atlas kit (detected from SetAtlas when name is a kit prefix).
    pub nine_slice_atlas: Option<NineSliceAtlasInfo>,
    /// Horizontal three-slice caps for stretched atlas textures.
    /// (left_cap_px, right_cap_px, atlas_entry_width_px)
    pub three_slice_h: Option<(f32, f32, f32)>,
    /// Raw WoW blend mode string from SetBlendMode/XML (e.g. "ADD", "MOD").
    pub alpha_mode: Option<String>,
    /// Blend mode for texture rendering (Alpha or Additive).
    pub blend_mode: BlendMode,
    /// Whether texture sampling should snap to the pixel grid.
    pub snap_to_pixel_grid: bool,
    /// Bias applied to texel snapping for textures.
    pub texel_snapping_bias: f32,
    /// Manually configured nine-slice margins (left, right, top, bottom).
    pub texture_slice_margins: (f32, f32, f32, f32),
    /// Manually configured nine-slice mode.
    pub texture_slice_mode: i32,
    /// Whether this frame receives ALL events (set by RegisterAllEvents).
    pub register_all_events: bool,
    /// Whether this frame clips its children to its bounds.
    pub clips_children: bool,
    /// Whether this frame should behave like a frame buffer for texture rotation APIs.
    pub is_frame_buffer: bool,
    /// Stored Minimap blip atlas/texture asset.
    pub minimap_blip_texture: Option<String>,
    /// Stored Minimap mask asset.
    pub minimap_mask_texture: Option<String>,
    /// Stored Minimap icon texture asset.
    pub minimap_icon_texture: Option<String>,
    /// Stored Minimap player arrow texture asset.
    pub minimap_player_texture: Option<String>,
    /// Stored Minimap POI arrow texture asset.
    pub minimap_poi_arrow_texture: Option<String>,
    /// Stored Minimap corpse POI arrow texture asset.
    pub minimap_corpse_poi_arrow_texture: Option<String>,
    /// Stored Minimap static POI arrow texture asset.
    pub minimap_static_poi_arrow_texture: Option<String>,
    /// Latest Minimap ping position in normalized map coordinates.
    pub minimap_ping_position: Option<(f32, f32)>,
    /// Monotonic revision bumped by UpdateBlips().
    pub minimap_blip_update_revision: u64,
    /// Stored Minimap quest blob inside style.
    pub quest_blob_inside: MinimapBlobLayerStyle,
    /// Stored Minimap quest blob outside style.
    pub quest_blob_outside: MinimapBlobLayerStyle,
    /// Stored Minimap quest blob ring style.
    pub quest_blob_ring: MinimapBlobRingStyle,
    /// Stored Minimap task blob inside style.
    pub task_blob_inside: MinimapBlobLayerStyle,
    /// Stored Minimap task blob outside style.
    pub task_blob_outside: MinimapBlobLayerStyle,
    /// Stored Minimap task blob ring style.
    pub task_blob_ring: MinimapBlobRingStyle,
    /// Stored Minimap archaeology blob inside style.
    pub arch_blob_inside: MinimapBlobLayerStyle,
    /// Stored Minimap archaeology blob outside style.
    pub arch_blob_outside: MinimapBlobLayerStyle,
    /// Stored Minimap archaeology blob ring style.
    pub arch_blob_ring: MinimapBlobRingStyle,
    /// Whether this texture is a MaskTexture (should not render).
    pub is_mask: bool,
    /// Mask texture IDs applied to this texture (for circular clipping etc.).
    pub mask_textures: Vec<u64>,
    /// Texture rotation in radians (for SetRotation on Texture widgets).
    pub rotation: f32,
    /// Model asset path set via Model:SetModel().
    pub model_path: Option<String>,
    /// Model file data ID when a model is sourced by file ID instead of path.
    pub model_file_id: Option<i64>,
    /// Persisted model transform and camera state.
    pub model_transform: ModelTransformState,
    /// Persisted model appearance and playback state.
    pub model_appearance: ModelAppearanceState,
    /// Persisted model rendering flags.
    pub model_rendering: ModelRenderingState,
    /// Persisted ModelScene state.
    pub model_scene_state: ModelSceneState,
    /// ModelScene actor IDs in scene order.
    pub model_scene_actor_ids: Vec<u64>,
    /// Persisted PlayerModel-only state.
    pub player_model_state: PlayerModelState,
    /// Whether mouse motion events are enabled.
    pub mouse_motion_enabled: bool,
    /// User-set frame ID (from XML `id` attribute or SetID()).
    pub user_id: i32,
    /// Button state: 0=NORMAL, 1=PUSHED (set by SetButtonState from Lua).
    pub button_state: u8,
    /// Text offset applied while a button is visually pressed.
    pub pushed_text_offset: (f32, f32),
    /// Eagerly computed layout rect (updated on SetPoint, SetSize, etc.).
    pub layout_rect: Option<crate::LayoutRect>,
    // --- Slider fields ---
    /// Current slider value.
    pub slider_value: f64,
    /// Slider minimum value.
    pub slider_min: f64,
    /// Slider maximum value.
    pub slider_max: f64,
    /// Slider step size.
    pub slider_step: f64,
    /// Slider orientation ("HORIZONTAL" or "VERTICAL").
    pub slider_orientation: String,
    /// Whether slider obeys step on drag.
    pub slider_obey_step_on_drag: bool,
    /// Number of steps per page for slider.
    pub slider_steps_per_page: i32,

    // --- StatusBar fields ---
    /// Current statusbar value.
    pub statusbar_value: f64,
    /// StatusBar minimum value.
    pub statusbar_min: f64,
    /// StatusBar maximum value.
    pub statusbar_max: f64,
    /// Currently displayed interpolated status bar value.
    pub statusbar_interpolated_value: f64,
    /// Target value when the status bar is interpolating.
    pub statusbar_interpolation_target: Option<f64>,
    /// StatusBar color.
    pub statusbar_color: Option<Color>,
    /// StatusBar texture path.
    pub statusbar_texture_path: Option<String>,
    /// StatusBar bar texture child ID (set by SetStatusBarTexture).
    pub statusbar_bar_id: Option<u64>,
    /// StatusBar desaturation amount in the normalized 0..1 range.
    pub statusbar_desaturation: f64,
    /// StatusBar fill style ("STANDARD", "CENTER", etc.).
    pub statusbar_fill_style: String,
    /// Whether statusbar fills in reverse.
    pub statusbar_reverse_fill: bool,
    /// StatusBar orientation ("HORIZONTAL" or "VERTICAL").
    pub statusbar_orientation: String,

    // --- EditBox fields ---
    /// Cursor position in editbox.
    pub editbox_cursor_pos: i32,
    /// Maximum letters allowed (0 = unlimited).
    pub editbox_max_letters: i32,
    /// Maximum bytes allowed (0 = unlimited).
    pub editbox_max_bytes: i32,
    /// Whether editbox is multi-line.
    pub editbox_multi_line: bool,
    /// Whether editbox auto-focuses.
    pub editbox_auto_focus: bool,
    /// Whether editbox is numeric-only.
    pub editbox_numeric: bool,
    /// Whether editbox masks input as password.
    pub editbox_password: bool,
    /// Cursor blink speed in seconds.
    pub editbox_blink_speed: f64,
    /// History lines.
    pub editbox_history: Vec<String>,
    /// Maximum history lines (0 = unlimited).
    pub editbox_history_max: i32,
    /// Text insets (left, right, top, bottom).
    pub editbox_text_insets: (f32, f32, f32, f32),
    /// Whether to count invisible letters.
    pub editbox_count_invisible_letters: bool,
    /// Whether the editbox is currently in IME composition mode.
    pub editbox_in_ime_composition_mode: bool,
    /// Whether only alphabetic input should be accepted.
    pub editbox_alphabetic_only: bool,
    /// Whether alt+arrow mode is enabled.
    pub editbox_alt_arrow_key_mode: bool,
    /// Whether numeric input should use the full range behavior.
    pub editbox_numeric_full_range: bool,
    /// Whether the text is marked as secure.
    pub editbox_secure_text: bool,
    /// Whether SetText should be disabled for the editbox.
    pub editbox_security_disable_set_text: bool,
    /// Whether paste is disabled for the editbox.
    pub editbox_security_disable_paste: bool,
    /// Maximum number of visible text bytes (0 = unlimited).
    pub editbox_visible_text_byte_limit: i32,
    /// Current input language identifier.
    pub editbox_input_language: String,
    /// Currently highlighted text range as character offsets.
    pub editbox_highlight_range: Option<(i32, i32)>,
    /// Highlight color for selected text.
    pub editbox_highlight_color: Color,

    // --- ScrollFrame fields ---
    /// Scroll child frame ID.
    pub scroll_child_id: Option<u64>,
    /// Cached scroll child subtree bounds after UpdateScrollChildRect().
    pub scroll_child_rect_size: Option<(f32, f32)>,
    /// Horizontal scroll offset.
    pub scroll_horizontal: f64,
    /// Vertical scroll offset.
    pub scroll_vertical: f64,

    // --- Cooldown fields ---
    /// Cooldown start time.
    pub cooldown_start: f64,
    /// Cooldown duration in seconds.
    pub cooldown_duration: f64,
    /// Whether cooldown is reversed.
    pub cooldown_reverse: bool,
    /// Whether to draw the swipe animation.
    pub cooldown_draw_swipe: bool,
    /// Whether to draw the edge highlight.
    pub cooldown_draw_edge: bool,
    /// Whether to draw the bling animation at end.
    pub cooldown_draw_bling: bool,
    /// Whether to hide countdown numbers.
    pub cooldown_hide_countdown: bool,
    /// Optional swipe fill texture path.
    pub cooldown_swipe_texture: Option<String>,
    /// Optional edge overlay texture path.
    pub cooldown_edge_texture: Option<String>,
    /// Optional bling overlay texture path.
    pub cooldown_bling_texture: Option<String>,
    /// Countdown display duration in milliseconds, unaffected by modRate.
    pub cooldown_display_duration_ms: f64,
    /// Cooldown modRate value from SetCooldown* APIs.
    pub cooldown_mod_rate: f64,
    /// Edge highlight scale factor.
    pub cooldown_edge_scale: f64,
    /// Whether edge rendering should use a circular clip.
    pub cooldown_use_circular_edge: bool,
    /// Threshold in seconds at which countdown text switches to abbreviated output.
    pub cooldown_countdown_abbrev_threshold_seconds: f64,
    /// Minimum countdown duration before text should show, in milliseconds.
    pub cooldown_min_countdown_duration_ms: f64,
    /// Whether countdown text should use aura-style display rules.
    pub cooldown_use_aura_display_time: bool,
    /// Edge highlight color.
    pub cooldown_edge_color: Color,
    /// UV range for cooldown swipe atlases: (low_x, low_y, high_x, high_y).
    pub cooldown_tex_coord_range: Option<(f32, f32, f32, f32)>,
    /// Countdown font string child used by GetCountdownFontString.
    pub cooldown_countdown_font_string_id: Option<u64>,
    /// Whether cooldown is paused.
    pub cooldown_paused: bool,

    // --- Line fields ---
    /// Line start anchor (for Line widgets).
    pub line_start: Option<LineAnchor>,
    /// Line end anchor (for Line widgets).
    pub line_end: Option<LineAnchor>,
    /// Line thickness in pixels (for Line widgets).
    pub line_thickness: f32,

    // --- Rendering effect fields ---
    /// Whether this texture/frame is desaturated (greyscale).
    pub desaturated: bool,

    /// Index into `SimState.addons` identifying which addon owns this frame.
    /// Set during frame creation based on `SimState.loading_addon_index`.
    pub owner_addon: Option<u16>,

    /// Whether the parent was defaulted (e.g. UIParent) rather than explicitly set.
    /// Used by SetAllPoints to match wowless headless behavior where default-parented
    /// frames anchor to screen (nil) rather than to the default parent.
    pub default_parent: bool,

    /// Whether this frame is forbidden (secure-restricted, e.g. created inside ScopedModifier forbidden="true").
    /// Forbidden frames are exposed as proxy tables in _G with a "Forbidden" metatable.
    pub forbidden: bool,

    /// Whether this frame is protected (inherits SecureFrameTemplate).
    /// Protected frames can use secure handler execution (SecureHandlerExecute, SetFrameRef, etc.).
    pub is_protected: bool,

    /// Whether this frame marks script-visible values as secret.
    pub prevent_secret_values: bool,
}

/// Build a `Frame` with all defaults. `$id` is the expression for the `id` field.
macro_rules! frame_defaults {
    ($id:expr) => {
        Frame {
            id: $id,
            widget_type: WidgetType::Frame,
            object_type_name: None,
            name: None,
            parent_id: None,
            children: Vec::new(),
            width: 0.0,
            width_is_text_auto: false,
            height: 0.0,
            anchors: Vec::new(),
            visible: true,
            show_hide_depth: 0,
            collapses_layout: false,
            registered_events: HashSet::new(),
            frame_level: 0,
            raise_order: 0,
            has_fixed_frame_level: false,
            frame_level_offset: None,
            frame_strata: FrameStrata::Medium,
            has_fixed_frame_strata: false,
            flattens_render_layers: false,
            toplevel: false,
            alpha: 1.0,
            ignore_parent_alpha: false,
            effective_alpha: 1.0,
            anim_offset_x: 0.0,
            anim_offset_y: 0.0,
            scale: 1.0,
            ignore_parent_scale: false,
            effective_scale: 1.0,
            mouse_enabled: false,
            mouse_wheel_enabled: false,
            motion_scripts_while_disabled: false,
            gamepad_button_enabled: false,
            gamepad_stick_enabled: false,
            propagate_mouse_clicks: false,
            propagate_mouse_motion: false,
            propagate_hyperlinks_to_parent: false,
            pass_through_buttons: HashSet::new(),
            hit_rect_insets: (0.0, 0.0, 0.0, 0.0),
            clamp_rect_insets: (0.0, 0.0, 0.0, 0.0),
            keyboard_enabled: false,
            propagate_keyboard_input: false,
            texture: None,
            texture_file_data_id: None,
            color_texture: None,
            vertex_color: None,
            gradient: None,
            alpha_gradients: HashMap::new(),
            text: None,
            text_stripped: None,
            title: None,
            text_color: Color::new(1.0, 0.8, 0.2, 1.0),
            shadow_color: Color::new(0.0, 0.0, 0.0, 0.0),
            shadow_offset: (0.0, 0.0),
            font: None,
            font_size: 14.0,
            font_outline: TextOutline::None,
            justify_h: TextJustify::Center,
            justify_v: TextJustify::Center,
            attributes: HashMap::new(),
            backdrop: Backdrop::default(),
            children_keys: HashMap::new(),
            parent_key: None,
            movable: false,
            resizable: false,
            resize_bounds_min: (0.0, 0.0),
            resize_bounds_max: None,
            user_placed: false,
            dont_save_position: false,
            registered_drag_buttons: HashSet::new(),
            highlight_locked: false,
            ignoring_children_for_bounds: false,
            clamped_to_screen: false,
            is_moving: false,
            word_wrap: true,
            max_lines: 0,
            text_scale: 1.0,
            normal_texture: None,
            normal_tex_coords: None,
            pushed_texture: None,
            pushed_tex_coords: None,
            highlight_texture: None,
            highlight_tex_coords: None,
            disabled_texture: None,
            disabled_tex_coords: None,
            checked_texture: None,
            checked_tex_coords: None,
            disabled_checked_texture: None,
            disabled_checked_tex_coords: None,
            left_texture: None,
            middle_texture: None,
            right_texture: None,
            draw_layer: DrawLayer::Artwork,
            draw_sub_layer: 0,
            disabled_draw_layers: BTreeSet::new(),
            horiz_tile: false,
            vert_tile: false,
            tex_coords: None,
            tex_coords_quad: None,
            atlas_tex_coords: None,
            atlas: None,
            nine_slice_layout: None,
            nine_slice_atlas: None,
            three_slice_h: None,
            alpha_mode: None,
            blend_mode: BlendMode::Alpha,
            snap_to_pixel_grid: false,
            texel_snapping_bias: 0.0,
            texture_slice_margins: (0.0, 0.0, 0.0, 0.0),
            texture_slice_mode: 0,
            register_all_events: false,
            clips_children: false,
            is_frame_buffer: false,
            minimap_blip_texture: None,
            minimap_mask_texture: None,
            minimap_icon_texture: None,
            minimap_player_texture: None,
            minimap_poi_arrow_texture: None,
            minimap_corpse_poi_arrow_texture: None,
            minimap_static_poi_arrow_texture: None,
            minimap_ping_position: None,
            minimap_blip_update_revision: 0,
            quest_blob_inside: MinimapBlobLayerStyle::default(),
            quest_blob_outside: MinimapBlobLayerStyle::default(),
            quest_blob_ring: MinimapBlobRingStyle::default(),
            task_blob_inside: MinimapBlobLayerStyle::default(),
            task_blob_outside: MinimapBlobLayerStyle::default(),
            task_blob_ring: MinimapBlobRingStyle::default(),
            arch_blob_inside: MinimapBlobLayerStyle::default(),
            arch_blob_outside: MinimapBlobLayerStyle::default(),
            arch_blob_ring: MinimapBlobRingStyle::default(),
            is_mask: false,
            mask_textures: Vec::new(),
            rotation: 0.0,
            model_path: None,
            model_file_id: None,
            model_transform: ModelTransformState::default(),
            model_appearance: ModelAppearanceState::default(),
            model_rendering: ModelRenderingState::default(),
            model_scene_state: ModelSceneState::default(),
            model_scene_actor_ids: Vec::new(),
            player_model_state: PlayerModelState::default(),
            mouse_motion_enabled: false,
            user_id: 0,
            button_state: 0,
            pushed_text_offset: (0.0, 0.0),
            layout_rect: None,

            // Slider
            slider_value: 0.0,
            slider_min: 0.0,
            slider_max: 100.0,
            slider_step: 1.0,
            slider_orientation: "HORIZONTAL".to_string(),
            slider_obey_step_on_drag: false,
            slider_steps_per_page: 1,

            // StatusBar
            statusbar_value: 0.0,
            statusbar_min: 0.0,
            statusbar_max: 1.0,
            statusbar_interpolated_value: 0.0,
            statusbar_interpolation_target: None,
            statusbar_color: None,
            statusbar_texture_path: None,
            statusbar_bar_id: None,
            statusbar_desaturation: 0.0,
            statusbar_fill_style: "STANDARD".to_string(),
            statusbar_reverse_fill: false,
            statusbar_orientation: "HORIZONTAL".to_string(),

            // EditBox
            editbox_cursor_pos: 0,
            editbox_max_letters: 0,
            editbox_max_bytes: 0,
            editbox_multi_line: false,
            editbox_auto_focus: false,
            editbox_numeric: false,
            editbox_password: false,
            editbox_blink_speed: 0.5,
            editbox_history: Vec::new(),
            editbox_history_max: 0,
            editbox_text_insets: (0.0, 0.0, 0.0, 0.0),
            editbox_count_invisible_letters: false,
            editbox_in_ime_composition_mode: false,
            editbox_alphabetic_only: false,
            editbox_alt_arrow_key_mode: false,
            editbox_numeric_full_range: false,
            editbox_secure_text: false,
            editbox_security_disable_set_text: false,
            editbox_security_disable_paste: false,
            editbox_visible_text_byte_limit: 0,
            editbox_input_language: "ROMAN".to_string(),
            editbox_highlight_range: None,
            editbox_highlight_color: Color::new(1.0, 1.0, 1.0, 1.0),

            // ScrollFrame
            scroll_child_id: None,
            scroll_child_rect_size: None,
            scroll_horizontal: 0.0,
            scroll_vertical: 0.0,

            // Cooldown
            cooldown_start: 0.0,
            cooldown_duration: 0.0,
            cooldown_reverse: false,
            cooldown_draw_swipe: true,
            cooldown_draw_edge: false,
            cooldown_draw_bling: true,
            cooldown_hide_countdown: false,
            cooldown_swipe_texture: None,
            cooldown_edge_texture: None,
            cooldown_bling_texture: None,
            cooldown_display_duration_ms: 0.0,
            cooldown_mod_rate: 1.0,
            cooldown_edge_scale: 1.0,
            cooldown_use_circular_edge: false,
            cooldown_countdown_abbrev_threshold_seconds: 0.0,
            cooldown_min_countdown_duration_ms: 0.0,
            cooldown_use_aura_display_time: false,
            cooldown_edge_color: Color::rgb(1.0, 1.0, 1.0),
            cooldown_tex_coord_range: None,
            cooldown_countdown_font_string_id: None,
            cooldown_paused: false,

            // Line
            line_start: None,
            line_end: None,
            line_thickness: 1.0,

            // Rendering effects
            desaturated: false,

            // Profiler
            owner_addon: None,

            default_parent: false,
            forbidden: false,
            is_protected: false,
            prevent_secret_values: false,
        }
    };
}

impl Default for Frame {
    fn default() -> Self {
        frame_defaults!(next_widget_id())
    }
}

impl Frame {
    pub fn new(widget_type: WidgetType, name: Option<String>, parent_id: Option<u64>) -> Self {
        Self {
            widget_type,
            name,
            parent_id,
            ..Default::default()
        }
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_point(
        &mut self,
        point: AnchorPoint,
        relative_to_id: Option<usize>,
        relative_point: AnchorPoint,
        x_offset: f32,
        y_offset: f32,
    ) {
        // Replace existing anchor with same point, or add new one
        let new_anchor = Anchor {
            point,
            relative_to: None,
            relative_to_id,
            relative_point,
            x_offset,
            y_offset,
        };
        if let Some(existing) = self.anchors.iter_mut().find(|a| a.point == point) {
            *existing = new_anchor;
        } else {
            self.anchors.push(new_anchor);
        }
    }

    pub fn set_point_with_name(
        &mut self,
        point: AnchorPoint,
        relative_to: Option<String>,
        relative_point: AnchorPoint,
        x_offset: f32,
        y_offset: f32,
    ) {
        // Replace existing anchor with same point, or add new one
        let new_anchor = Anchor {
            point,
            relative_to,
            relative_to_id: None,
            relative_point,
            x_offset,
            y_offset,
        };
        if let Some(existing) = self.anchors.iter_mut().find(|a| a.point == point) {
            *existing = new_anchor;
        } else {
            self.anchors.push(new_anchor);
        }
    }

    pub fn clear_all_points(&mut self) {
        self.anchors.clear();
    }

    pub fn register_event(&mut self, event: &str) {
        self.registered_events.insert(event.to_string());
    }

    pub fn unregister_event(&mut self, event: &str) {
        self.registered_events.remove(event);
    }

    pub fn is_registered_for_event(&self, event: &str) -> bool {
        self.register_all_events || self.registered_events.contains(event)
    }

    pub fn is_draw_layer_enabled(&self, layer: DrawLayer) -> bool {
        !self.disabled_draw_layers.contains(&layer)
    }

    pub fn set_draw_layer_enabled(&mut self, layer: DrawLayer, enabled: bool) {
        if enabled {
            self.disabled_draw_layers.remove(&layer);
        } else {
            self.disabled_draw_layers.insert(layer);
        }
    }

    pub fn storage_estimate_bytes(&self) -> usize {
        self.dynamic_string_bytes()
            + vec_bytes(&self.children)
            + vec_bytes(&self.anchors)
            + hash_set_string_bytes(&self.registered_events)
            + hash_set_string_bytes(&self.pass_through_buttons)
            + hash_map_string_attribute_value_bytes(&self.attributes)
            + hash_map_string_u64_bytes(&self.children_keys)
            + hash_set_string_bytes(&self.registered_drag_buttons)
            + btree_set_bytes(&self.disabled_draw_layers)
            + vec_bytes(&self.mask_textures)
            + vec_bytes(&self.model_scene_actor_ids)
            + vec_string_bytes(&self.editbox_history)
    }

    fn dynamic_string_bytes(&self) -> usize {
        option_string_bytes(&self.object_type_name)
            + option_string_bytes(&self.name)
            + option_string_bytes(&self.texture)
            + option_string_bytes(&self.text)
            + option_string_bytes(&self.font)
            + option_string_bytes(&self.parent_key)
            + option_string_bytes(&self.normal_texture)
            + option_string_bytes(&self.pushed_texture)
            + option_string_bytes(&self.highlight_texture)
            + option_string_bytes(&self.disabled_texture)
            + option_string_bytes(&self.checked_texture)
            + option_string_bytes(&self.disabled_checked_texture)
            + option_string_bytes(&self.left_texture)
            + option_string_bytes(&self.middle_texture)
            + option_string_bytes(&self.right_texture)
            + option_string_bytes(&self.atlas)
            + option_string_bytes(&self.nine_slice_layout)
            + option_string_bytes(&self.alpha_mode)
            + option_string_bytes(&self.minimap_blip_texture)
            + option_string_bytes(&self.minimap_mask_texture)
            + option_string_bytes(&self.minimap_icon_texture)
            + option_string_bytes(&self.minimap_player_texture)
            + option_string_bytes(&self.minimap_poi_arrow_texture)
            + option_string_bytes(&self.minimap_corpse_poi_arrow_texture)
            + option_string_bytes(&self.minimap_static_poi_arrow_texture)
            + option_string_bytes(&self.model_path)
            + option_string_bytes(&self.statusbar_texture_path)
            + self.slider_orientation.capacity()
            + self.statusbar_fill_style.capacity()
            + self.statusbar_orientation.capacity()
            + self.editbox_input_language.capacity()
    }
}

fn option_string_bytes(value: &Option<String>) -> usize {
    value.as_ref().map_or(0, String::capacity)
}

fn vec_bytes<T>(values: &[T]) -> usize {
    std::mem::size_of_val(values)
}

fn vec_string_bytes(values: &[String]) -> usize {
    vec_bytes(values) + values.iter().map(String::capacity).sum::<usize>()
}

fn hash_set_string_bytes(values: &HashSet<String>) -> usize {
    values.capacity() * std::mem::size_of::<String>()
        + values.iter().map(String::capacity).sum::<usize>()
}

fn hash_map_string_u64_bytes(values: &HashMap<String, u64>) -> usize {
    values.capacity() * std::mem::size_of::<(String, u64)>()
        + values.keys().map(String::capacity).sum::<usize>()
}

fn hash_map_string_attribute_value_bytes(values: &HashMap<String, AttributeValue>) -> usize {
    values.capacity() * std::mem::size_of::<(String, AttributeValue)>()
        + values
            .iter()
            .map(|(key, value)| key.capacity() + attribute_value_bytes(value))
            .sum::<usize>()
}

fn attribute_value_bytes(value: &AttributeValue) -> usize {
    match value {
        AttributeValue::String(text) => text.capacity(),
        AttributeValue::Number(_) | AttributeValue::Boolean(_) | AttributeValue::Nil => 0,
    }
}

fn btree_set_bytes<T>(values: &BTreeSet<T>) -> usize {
    values.len() * std::mem::size_of::<T>()
}

pub use super::frame_enums::{DrawLayer, FrameStrata};
