#![feature(decl_macro, try_trait_v2)]

pub mod commands {
    pub mod attack;
    pub mod block;
    pub mod receive;
    pub mod rise;
    pub mod vital_trill;
}
pub mod common {
    pub mod attack;
}
pub mod _prelude;
pub mod handler;
pub mod validate;

use crate::_prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] CoreError),
    #[error("Invalid action tag: {action_tag}")]
    InvalidActionTag { action_tag: FixedString },
}
