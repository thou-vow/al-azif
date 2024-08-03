pub mod commands {
    pub mod attack;
    pub mod block;
    pub mod heal;
    pub mod miracle;
    pub mod receive;
    pub mod rise;
    pub mod vital_trill;
}
pub mod _prelude;
pub mod handler;
pub mod setting;

use crate::_prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("An expected error")]
    Expected(Blueprints),
    #[error(transparent)]
    Core(#[from] CoreError),
    #[error("Failed to convert string to reaction type: {str}")]
    FailedToConvertStringToReactionType { str: &'static str },
    #[error("Invalid action tag: {action_tag}")]
    InvalidActionTag { action_tag: FixedString<u8> },
    #[error(
        "Could not infer user, as the reactive moment target tags index is out of bounds, the primary action tag: {primary_action_tag}, \
         the index: {index}"
    )]
    ReactiveMomentTargetTagsIndexOutOfBounds { primary_action_tag: FixedString<u8>, index: usize },
}
