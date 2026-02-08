use serde::{Deserialize, Serialize};

/// This object describes the rating of a user based on their Telegram Star
/// spendings.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct UserRating {
    /// Current level of the user
    pub level: i32,

    /// Numerical value of the user's rating
    pub rating: i32,

    /// The rating value required to get the current level
    pub current_level_rating: i32,

    /// The rating value required to get to the next level
    pub next_level_rating: Option<i32>,
}
