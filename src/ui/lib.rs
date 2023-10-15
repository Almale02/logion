use bevy::{prelude::Bundle, sprite::Anchor};
use bevy_text::*;

#[derive(Bundle, Clone, Debug, Default)]
pub struct SpritedTextBundle {
    /// Contains the text.
    pub text: Text,
    /// How the text is positioned relative to its transform.
    pub text_anchor: Anchor,
    /// The maximum width and height of the text.
    pub text_2d_bounds: Text2dBounds,
    /// Contains the size of the text and its glyph's position and scale data. Generated via [`TextPipeline::queue_text`]
    pub text_layout_info: TextLayoutInfo,
}
