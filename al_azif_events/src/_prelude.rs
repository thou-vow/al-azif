pub use crate::Error as EventError;
pub use al_azif_prefix::_prelude::*;
pub use al_azif_slash::_prelude::*;

pub type Result<T> = std::result::Result<T, EventError>;
