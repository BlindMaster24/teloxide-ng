use serde::{Deserialize, Serialize};

use crate::types::{CustomEmojiId, Rgb};

/// This object contains information about the color scheme for a user's name,
/// message replies and link previews based on a unique gift.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct UniqueGiftColors {
    /// Custom emoji identifier of the unique gift's model
    pub model_custom_emoji_id: CustomEmojiId,

    /// Custom emoji identifier of the unique gift's symbol
    pub symbol_custom_emoji_id: CustomEmojiId,

    /// Main color used in light themes; RGB format
    pub light_theme_main_color: Rgb,

    /// List of 1-3 additional colors used in light themes; RGB format
    pub light_theme_other_colors: Vec<Rgb>,

    /// Main color used in dark themes; RGB format
    pub dark_theme_main_color: Rgb,

    /// List of 1-3 additional colors used in dark themes; RGB format
    pub dark_theme_other_colors: Vec<Rgb>,
}
