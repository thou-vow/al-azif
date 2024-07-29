pub use crate::{common, handler, validate, Error as PrefixError};
pub use al_azif_core::_prelude::*;

pub(crate) type Result<T> = std::result::Result<T, PrefixError>;
