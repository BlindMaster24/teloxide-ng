use serde::{Deserialize, Serialize};

use crate::types::Audio;

/// This object represents a user's profile audio files.
///
/// [The official docs](https://core.telegram.org/bots/api#userprofileaudios).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct UserProfileAudios {
    /// Total number of profile audio files the target user has.
    pub total_count: u32,

    /// Requested profile audio files.
    pub audios: Vec<Audio>,
}
