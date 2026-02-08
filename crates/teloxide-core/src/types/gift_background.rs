use serde::{Deserialize, Serialize};

use crate::types::Rgb;

/// This object describes the background of a gift.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct GiftBackground {
    /// Center color of the background in RGB format
    pub center_color: Rgb,

    /// Edge color of the background in RGB format
    pub edge_color: Rgb,

    /// Text color of the background in RGB format
    pub text_color: Rgb,
}
