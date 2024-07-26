pub use crate::{utils::execute_attack, Error as PrefixError};
pub use al_azif_core::_prelude::*;

pub type Result<T> = std::result::Result<T, PrefixError>;