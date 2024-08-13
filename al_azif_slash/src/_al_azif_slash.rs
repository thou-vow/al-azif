pub mod commands {
    pub mod battle;
    pub mod exp;
    pub mod help;
    pub mod id;
    pub mod ping;
    pub mod voice;
}
pub mod _prelude;

use crate::_prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("An expected error")]
    Anticipated(ErrorResponse),
    #[error(transparent)]
    Core(#[from] CoreError),
    #[error("Could not create response, why: {0}")]
    CouldNotCreateInteractionResponse(SerenityError),
    #[error("Failed to convert string to reaction type: {str}")]
    FailedToConvertStringToReactionType { str: &'static str },
    #[error("Invalid invested attribute: {attribute_str}")]
    InvalidInvestedAttribute { attribute_str: FixedString },
}
