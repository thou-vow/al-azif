#![feature(decl_macro)]

pub mod commands {
    pub mod battle;
    pub mod exp;
    pub mod help;
    pub mod id;
    pub mod ping;
}
pub mod _prelude;

use crate::_prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] CoreError),
    #[error("Could not create response, why: {0}")]
    CouldNotCreateInteractionResponse(SerenityError),
    #[error("Could not edit response, why: {0}")]
    CouldNotEditInteractionResponse(SerenityError),
    #[error("Invalid invested attribute: {attribute_str}")]
    InvalidInvestedAttribute { attribute_str: FixedString },
}
