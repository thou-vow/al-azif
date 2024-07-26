pub use crate::Error as SlashError;
pub use al_azif_core::_prelude::*;

pub type Result<T> = std::result::Result<T, SlashError>;
