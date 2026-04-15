//! XML element type definitions: layers, textures, fontstrings, animations, etc.

use serde::Deserialize;

use super::types::FrameXml;
use super::types_support::{
    AbsDimensionXml, AnchorsXml, AnimationsXml, ColorXml, KeyValuesXml, ScriptsXml, SizeXml,
};

/// Layers container (for textures and font strings).
#[derive(Debug, Deserialize, Clone)]
pub struct LayersXml {
    #[serde(rename = "Layer", default)]
    pub layers: Vec<LayerXml>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LayerXml {
    #[serde(rename = "@level")]
    pub level: Option<String>,
    #[serde(rename = "@textureSubLevel", default)]
    pub texture_sub_level: Option<i32>,
    #[serde(rename = "$value", default)]
    pub elements: Vec<LayerElement>,
}

impl LayerXml {
    /// Get all Texture elements in this layer (includes MaskTextures — they
    /// need to exist as child widgets so Lua code can reference them via parentKey).
    /// Returns (texture, is_mask, is_line) triples.
    pub fn textures(&self) -> impl Iterator<Item = (&TextureXml, bool, bool)> {
        self.elements.iter().filter_map(|e| match e {
            LayerElement::Texture(t) => Some((t, false, false)),
            LayerElement::Line(t) => Some((t, false, true)),
            LayerElement::MaskTexture(t) => Some((t, true, false)),
            _ => None,
        })
    }

    /// Get all FontString elements in this layer.
    pub fn font_strings(&self) -> impl Iterator<Item = &FontStringXml> {
        self.elements.iter().filter_map(|e| match e {
            LayerElement::FontString(f) => Some(f),
            _ => None,
        })
    }
}

/// Elements that can appear inside a Layer.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum LayerElement {
    Texture(TextureXml),
    FontString(FontStringXml),
    Line(TextureXml),
    MaskTexture(TextureXml),
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct TextureXml {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@inherits")]
    pub inherits: Option<String>,
    #[serde(rename = "@parentKey")]
    pub parent_key: Option<String>,
    #[serde(rename = "@file")]
    pub file: Option<String>,
    #[serde(rename = "@atlas")]
    pub atlas: Option<String>,
    #[serde(rename = "@useAtlasSize")]
    pub use_atlas_size: Option<bool>,
    #[serde(rename = "@virtual")]
    pub is_virtual: Option<bool>,
    #[serde(rename = "@mixin")]
    pub mixin: Option<String>,
    #[serde(rename = "@texelSnappingBias")]
    pub texel_snapping_bias: Option<String>,
    #[serde(rename = "@snapToPixelGrid")]
    pub snap_to_pixel_grid: Option<String>,
    #[serde(rename = "@horizTile")]
    pub horiz_tile: Option<bool>,
    #[serde(rename = "@vertTile")]
    pub vert_tile: Option<bool>,
    #[serde(rename = "@thickness")]
    pub thickness: Option<f32>,
    #[serde(rename = "@hidden")]
    pub hidden: Option<bool>,
    #[serde(rename = "@alpha")]
    pub alpha: Option<f32>,
    #[serde(rename = "@alphaMode")]
    pub alpha_mode: Option<String>,
    #[serde(rename = "@blendMode")]
    pub blend_mode: Option<String>,
    #[serde(rename = "@setAllPoints")]
    pub set_all_points: Option<bool>,
    #[serde(rename = "@parentArray")]
    pub parent_array: Option<String>,
    #[serde(rename = "Size")]
    pub size: Option<SizeXml>,
    #[serde(rename = "Anchors")]
    pub anchors: Option<AnchorsXml>,
    #[serde(rename = "Color")]
    pub color: Option<ColorXml>,
    #[serde(rename = "Gradient")]
    pub gradient: Option<crate::xml::GradientXml>,
    #[serde(rename = "Animations")]
    pub animations: Option<AnimationsXml>,
    #[serde(rename = "Scripts")]
    pub scripts: Option<ScriptsXml>,
    #[serde(rename = "KeyValues")]
    pub key_values: Option<KeyValuesXml>,
    /// Texture coordinates (left, right, top, bottom) for UV mapping.
    #[serde(rename = "TexCoords")]
    pub tex_coords: Option<TexCoordsXml>,
    /// MaskedTextures — declares which sibling textures this mask applies to.
    #[serde(rename = "MaskedTextures")]
    pub masked_textures: Option<MaskedTexturesXml>,
}

impl TextureXml {
    /// Effective blend mode from either WoW XML spelling.
    pub fn effective_blend_mode(&self) -> Option<&str> {
        self.blend_mode.as_deref().or(self.alpha_mode.as_deref())
    }
}

/// TexCoords element with left/right/top/bottom UV coordinates.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct TexCoordsXml {
    #[serde(rename = "@left")]
    pub left: Option<f32>,
    #[serde(rename = "@right")]
    pub right: Option<f32>,
    #[serde(rename = "@top")]
    pub top: Option<f32>,
    #[serde(rename = "@bottom")]
    pub bottom: Option<f32>,
    #[serde(rename = "Rect")]
    pub rect: Option<TexCoordsRectXml>,
}

/// Rect child for TexCoords — corner-based UV coordinates.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct TexCoordsRectXml {
    #[serde(rename = "@ULx")]
    pub ul_x: Option<f32>,
    #[serde(rename = "@ULy")]
    pub ul_y: Option<f32>,
    #[serde(rename = "@LLx")]
    pub ll_x: Option<f32>,
    #[serde(rename = "@LLy")]
    pub ll_y: Option<f32>,
    #[serde(rename = "@URx")]
    pub ur_x: Option<f32>,
    #[serde(rename = "@URy")]
    pub ur_y: Option<f32>,
    #[serde(rename = "@LRx")]
    pub lr_x: Option<f32>,
    #[serde(rename = "@LRy")]
    pub lr_y: Option<f32>,
}

/// Container for MaskedTexture entries inside a MaskTexture element.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct MaskedTexturesXml {
    #[serde(rename = "MaskedTexture", default)]
    pub entries: Vec<MaskedTextureEntryXml>,
}

/// A single MaskedTexture entry referencing a sibling texture by childKey.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct MaskedTextureEntryXml {
    #[serde(rename = "@childKey")]
    pub child_key: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct FontStringXml {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@parentKey")]
    pub parent_key: Option<String>,
    #[serde(rename = "@inherits")]
    pub inherits: Option<String>,
    #[serde(rename = "@mixin")]
    pub mixin: Option<String>,
    #[serde(rename = "@virtual")]
    pub is_virtual: Option<bool>,
    #[serde(rename = "@hidden")]
    pub hidden: Option<bool>,
    #[serde(rename = "@alpha")]
    pub alpha: Option<f32>,
    #[serde(rename = "@text")]
    pub text: Option<String>,
    #[serde(rename = "@justifyH")]
    pub justify_h: Option<String>,
    #[serde(rename = "@justifyV")]
    pub justify_v: Option<String>,
    #[serde(rename = "Size", default)]
    pub size: Vec<SizeXml>,
    #[serde(rename = "Anchors")]
    pub anchors: Option<AnchorsXml>,
    #[serde(rename = "Color")]
    pub color: Option<ColorXml>,
    #[serde(rename = "Shadow")]
    pub shadow: Option<ShadowXml>,
    #[serde(rename = "Scripts")]
    pub scripts: Option<ScriptsXml>,
    #[serde(rename = "KeyValues")]
    pub key_values: Option<KeyValuesXml>,
    #[serde(rename = "@setAllPoints")]
    pub set_all_points: Option<bool>,
    #[serde(rename = "@parentArray")]
    pub parent_array: Option<String>,
    #[serde(rename = "@wordwrap")]
    pub word_wrap: Option<bool>,
    #[serde(rename = "@maxLines")]
    pub max_lines: Option<u32>,
    #[serde(rename = "FontHeight")]
    pub font_height: Option<FontHeightXml>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct FontHeightXml {
    #[serde(rename = "@val")]
    pub val: Option<f32>,
    #[serde(rename = "AbsValue")]
    pub abs_value: Option<AbsValueXml>,
}

impl FontHeightXml {
    pub fn value(&self) -> Option<f32> {
        self.val.or_else(|| self.abs_value.as_ref()?.val)
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct AbsValueXml {
    #[serde(rename = "@val")]
    pub val: Option<f32>,
}

/// Shadow element for FontStrings - contains offset and color.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct ShadowXml {
    #[serde(rename = "Offset")]
    pub offset: Option<ShadowOffsetXml>,
    #[serde(rename = "Color")]
    pub color: Option<ColorXml>,
}

/// Shadow offset - can have direct x/y attributes or nested AbsDimension.
#[derive(Debug, Deserialize, Clone)]
pub struct ShadowOffsetXml {
    #[serde(rename = "@x")]
    pub x: Option<f32>,
    #[serde(rename = "@y")]
    pub y: Option<f32>,
    #[serde(rename = "AbsDimension")]
    pub abs_dimension: Option<AbsDimensionXml>,
}

impl ShadowOffsetXml {
    /// Get the x offset, preferring direct attribute over nested AbsDimension.
    pub fn x(&self) -> f32 {
        self.x
            .or_else(|| self.abs_dimension.as_ref().and_then(|d| d.x))
            .unwrap_or(0.0)
    }

    /// Get the y offset, preferring direct attribute over nested AbsDimension.
    pub fn y(&self) -> f32 {
        self.y
            .or_else(|| self.abs_dimension.as_ref().and_then(|d| d.y))
            .unwrap_or(0.0)
    }
}

/// Child frames container - can contain any frame-like element.
#[derive(Debug, Deserialize, Clone)]
pub struct FramesXml {
    #[serde(rename = "$value", default)]
    pub elements: Vec<FrameElement>,
}

/// Frame-like elements that can appear inside a <Frames> container.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum FrameElement {
    Frame(FrameXml),
    Button(FrameXml),
    ItemButton(FrameXml),
    CheckButton(FrameXml),
    EditBox(FrameXml),
    ScrollFrame(FrameXml),
    Slider(FrameXml),
    StatusBar(FrameXml),
    GameTooltip(FrameXml),
    ColorSelect(FrameXml),
    Model(FrameXml),
    ModelScene(FrameXml),
    EventFrame(FrameXml),
    CinematicModel(FrameXml),
    PlayerModel(FrameXml),
    DressUpModel(FrameXml),
    Browser(FrameXml),
    Minimap(FrameXml),
    MessageFrame(FrameXml),
    MovieFrame(FrameXml),
    ScrollingMessageFrame(FrameXml),
    SimpleHTML(FrameXml),
    WorldFrame(FrameXml),
    DropDownToggleButton(FrameXml),
    DropdownButton(FrameXml),
    EventButton(FrameXml),
    EventEditBox(FrameXml),
    Cooldown(FrameXml),
    TaxiRouteFrame(FrameXml),
    ModelFFX(FrameXml),
    TabardModel(FrameXml),
    UiCamera(FrameXml),
    UnitPositionFrame(FrameXml),
    OffScreenFrame(FrameXml),
    Checkout(FrameXml),
    FogOfWarFrame(FrameXml),
    QuestPOIFrame(FrameXml),
    ArchaeologyDigSiteFrame(FrameXml),
    ScenarioPOIFrame(FrameXml),
    UIThemeContainerFrame(FrameXml),
    EventScrollFrame(FrameXml),
    ContainedAlertFrame(FrameXml),
    MapScene(FrameXml),
    ScopedModifier(super::types::ScopedModifierXml),
    Line(FrameXml),
}

fn preserved_frame_alias_type(tag: &str) -> Option<&'static str> {
    match tag {
        "EventFrame" => Some("EventFrame"),
        "UnitPositionFrame" => Some("UnitPositionFrame"),
        "OffScreenFrame" => Some("OffScreenFrame"),
        "Checkout" => Some("Checkout"),
        "FogOfWarFrame" => Some("FogOfWarFrame"),
        "QuestPOIFrame" => Some("QuestPOIFrame"),
        "ArchaeologyDigSiteFrame" => Some("ArchaeologyDigSiteFrame"),
        "ScenarioPOIFrame" => Some("ScenarioPOIFrame"),
        "Browser" => Some("Browser"),
        "MovieFrame" => Some("MovieFrame"),
        _ => None,
    }
}

/// Shared mapping from XML element tag name to `(widget_type, intrinsic_name)`.
///
/// Covers the common mappings used by both `FrameElement` (inside `<Frames>`)
/// and `XmlElement` (top-level). Callers handle divergences before calling this.
pub fn widget_type_for_tag(tag: &str) -> Option<(&'static str, Option<&'static str>)> {
    if let Some(widget_type) = preserved_frame_alias_type(tag) {
        return Some((widget_type, None));
    }

    match tag {
        "Frame" => Some(("Frame", None)),
        "Button" => Some(("Button", None)),
        "ItemButton" => Some(("Button", Some("ItemButton"))),
        "CheckButton" => Some(("CheckButton", None)),
        "EditBox" | "EventEditBox" => Some(("EditBox", None)),
        "ScrollFrame" | "EventScrollFrame" => Some(("ScrollFrame", None)),
        "Slider" => Some(("Slider", None)),
        "StatusBar" => Some(("StatusBar", None)),
        "Cooldown" => Some(("Cooldown", None)),
        "GameTooltip" => Some(("GameTooltip", None)),
        "ColorSelect" => Some(("ColorSelect", None)),
        "Model" => Some(("Model", None)),
        "ModelScene" => Some(("ModelScene", None)),
        "PlayerModel" | "CinematicModel" | "TabardModel" | "DressUpModel" => {
            Some(("PlayerModel", None))
        }
        "MessageFrame" => Some(("MessageFrame", None)),
        "ScrollingMessageFrame" => Some(("MessageFrame", Some("ScrollingMessageFrame"))),
        "SimpleHTML" => Some(("SimpleHTML", None)),
        "Minimap" => Some(("Minimap", None)),
        "DropdownButton" => Some(("Button", Some("DropdownButton"))),
        "ContainedAlertFrame" => Some(("Button", Some("ContainedAlertFrame"))),
        // Frame-like elements without a creatable alias still fall back to Frame.
        "TaxiRouteFrame"
        | "ModelFFX"
        | "UiCamera"
        | "UIThemeContainerFrame"
        | "MapScene"
        | "Line"
        | "WorldFrame" => Some(("Frame", None)),
        _ => None,
    }
}

macro_rules! frame_variant_data {
    ($self:expr, $($variant:ident),+ $(,)?) => {
        match $self {
            $(Self::$variant(f) => Some((f, stringify!($variant))),)+
            _ => None,
        }
    };
}
pub(crate) use frame_variant_data;

impl FrameElement {
    /// Extract the inner `FrameXml` and variant tag name (e.g. `"Button"`, `"EditBox"`).
    /// Returns `None` for `ScopedModifier` which has no `FrameXml`.
    pub fn as_frame_data(&self) -> Option<(&FrameXml, &'static str)> {
        frame_variant_data!(
            self,
            Frame,
            Button,
            ItemButton,
            CheckButton,
            EditBox,
            ScrollFrame,
            Slider,
            StatusBar,
            GameTooltip,
            ColorSelect,
            Model,
            ModelScene,
            EventFrame,
            CinematicModel,
            PlayerModel,
            DressUpModel,
            Browser,
            Minimap,
            MessageFrame,
            MovieFrame,
            ScrollingMessageFrame,
            SimpleHTML,
            WorldFrame,
            DropDownToggleButton,
            DropdownButton,
            EventButton,
            EventEditBox,
            Cooldown,
            TaxiRouteFrame,
            ModelFFX,
            TabardModel,
            UiCamera,
            UnitPositionFrame,
            OffScreenFrame,
            Checkout,
            FogOfWarFrame,
            QuestPOIFrame,
            ArchaeologyDigSiteFrame,
            ScenarioPOIFrame,
            UIThemeContainerFrame,
            EventScrollFrame,
            ContainedAlertFrame,
            MapScene,
            Line
        )
    }
}

/// Script include (file attribute is optional for inline scripts).
#[derive(Debug, Deserialize, Clone)]
pub struct ScriptXml {
    #[serde(rename = "@file")]
    pub file: Option<String>,
    #[serde(rename = "$text")]
    pub inline: Option<String>,
}

/// XML include.
#[derive(Debug, Deserialize, Clone)]
pub struct IncludeXml {
    #[serde(rename = "@file")]
    pub file: String,
}

/// Animation group definition.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct AnimationGroupXml {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@parentKey")]
    pub parent_key: Option<String>,
    #[serde(rename = "@inherits")]
    pub inherits: Option<String>,
    #[serde(rename = "@mixin")]
    pub mixin: Option<String>,
    #[serde(rename = "@virtual")]
    pub is_virtual: Option<bool>,
    #[serde(rename = "@setToFinalAlpha")]
    pub set_to_final_alpha: Option<bool>,
    #[serde(rename = "@looping")]
    pub looping: Option<String>,
    #[serde(rename = "$value", default)]
    pub elements: Vec<AnimationElement>,
}

/// Elements that can appear inside an AnimationGroup.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum AnimationElement {
    Animation(AnimationXml),
    Alpha(AnimationXml),
    Translation(AnimationXml),
    LineTranslation(AnimationXml),
    Rotation(AnimationXml),
    Scale(AnimationXml),
    LineScale(AnimationXml),
    Path(AnimationXml),
    FlipBook(AnimationXml),
    VertexColor(AnimationXml),
    TextureCoordTranslation(AnimationXml),
    Scripts(Box<ScriptsXml>),
    KeyValues(KeyValuesXml),
    #[serde(other)]
    Unknown,
}

/// Common animation attributes. Since the simulator doesn't execute animations,
/// all type-specific attributes are optional on a single struct.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct AnimationXml {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@parentKey")]
    pub parent_key: Option<String>,
    #[serde(rename = "@childKey")]
    pub child_key: Option<String>,
    #[serde(rename = "@target")]
    pub target: Option<String>,
    #[serde(rename = "@targetKey")]
    pub target_key: Option<String>,
    #[serde(rename = "@duration")]
    pub duration: Option<f32>,
    #[serde(rename = "@order")]
    pub order: Option<u32>,
    #[serde(rename = "@startDelay")]
    pub start_delay: Option<f32>,
    #[serde(rename = "@endDelay")]
    pub end_delay: Option<f32>,
    #[serde(rename = "@smoothing")]
    pub smoothing: Option<String>,
    // Alpha
    #[serde(rename = "@fromAlpha")]
    pub from_alpha: Option<f32>,
    #[serde(rename = "@toAlpha")]
    pub to_alpha: Option<f32>,
    // Translation
    #[serde(rename = "@offsetX")]
    pub offset_x: Option<f32>,
    #[serde(rename = "@offsetY")]
    pub offset_y: Option<f32>,
    // Scale
    #[serde(rename = "@scaleX")]
    pub scale_x: Option<f32>,
    #[serde(rename = "@scaleY")]
    pub scale_y: Option<f32>,
    #[serde(rename = "@fromScaleX")]
    pub from_scale_x: Option<f32>,
    #[serde(rename = "@fromScaleY")]
    pub from_scale_y: Option<f32>,
    #[serde(rename = "@toScaleX")]
    pub to_scale_x: Option<f32>,
    #[serde(rename = "@toScaleY")]
    pub to_scale_y: Option<f32>,
    // Rotation
    #[serde(rename = "@degrees")]
    pub degrees: Option<f32>,
    #[serde(rename = "@radians")]
    pub radians: Option<f32>,
    // FlipBook
    #[serde(rename = "@flipBookRows")]
    pub flip_book_rows: Option<u32>,
    #[serde(rename = "@flipBookColumns")]
    pub flip_book_columns: Option<u32>,
    #[serde(rename = "@flipBookFrames")]
    pub flip_book_frames: Option<u32>,
    // TextureCoordTranslation
    #[serde(rename = "@offsetU")]
    pub offset_u: Option<f32>,
    #[serde(rename = "@offsetV")]
    pub offset_v: Option<f32>,
    // Path
    #[serde(rename = "@curve")]
    pub curve: Option<String>,
    // Child elements (parsed but not executed)
    #[serde(rename = "Origin")]
    pub origin: Option<OriginXml>,
    #[serde(rename = "ControlPoints")]
    pub control_points: Option<ControlPointsXml>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct OriginXml {
    #[serde(rename = "@point")]
    pub point: Option<String>,
    #[serde(rename = "@x")]
    pub x: Option<f32>,
    #[serde(rename = "@y")]
    pub y: Option<f32>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ControlPointsXml {
    #[serde(rename = "ControlPoint", default)]
    pub points: Vec<ControlPointXml>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ControlPointXml {
    #[serde(rename = "@offsetX")]
    pub offset_x: Option<f32>,
    #[serde(rename = "@offsetY")]
    pub offset_y: Option<f32>,
}

/// Actors container for ModelScene.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct ActorsXml {
    #[serde(rename = "Actor", default)]
    pub actors: Vec<ActorXml>,
}

/// Actor definition for ModelScene.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct ActorXml {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@parentKey")]
    pub parent_key: Option<String>,
    #[serde(rename = "@mixin")]
    pub mixin: Option<String>,
    #[serde(rename = "@virtual")]
    pub is_virtual: Option<bool>,
}

/// Font definition.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct FontXml {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@inherits")]
    pub inherits: Option<String>,
    #[serde(rename = "@virtual")]
    pub is_virtual: Option<bool>,
    #[serde(rename = "@font")]
    pub font: Option<String>,
    #[serde(rename = "@height")]
    pub height: Option<f32>,
    #[serde(rename = "@outline")]
    pub outline: Option<String>,
    #[serde(rename = "@justifyH")]
    pub justify_h: Option<String>,
    #[serde(rename = "@justifyV")]
    pub justify_v: Option<String>,
}

/// FontFamily definition - collection of fonts for different alphabets.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct FontFamilyXml {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@virtual")]
    pub is_virtual: Option<bool>,
    #[serde(rename = "Member", default)]
    pub members: Vec<FontFamilyMemberXml>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct FontFamilyMemberXml {
    #[serde(rename = "@alphabet")]
    pub alphabet: Option<String>,
    #[serde(rename = "Font")]
    pub font: Option<FontXml>,
}
