#![feature(decl_macro, try_trait_v2)]

pub mod commands {
    pub mod attack;
    pub mod block;
    pub mod miracle;
    pub mod receive;
    pub mod rise;
    pub mod vital_trill;
}
pub mod _prelude;
pub mod handler;

use crate::_prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] CoreError),
    #[error("Failed to convert string to reaction type: {str}")]
    FailedToConvertStringToReactionType { str: &'static str },
    #[error("Invalid action tag: {action_tag}")]
    InvalidActionTag { action_tag: FixedString },
}
