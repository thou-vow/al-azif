pub use crate::{
    handler,
    setting::{Empty, FBattle, FOptionalTargets, FPrimaryMoment, FReactiveMoment, FTargets, FUser, Setting},
    Error as PrefixError,
};
pub use al_azif_core::_prelude::*;

pub(crate) type Result<T> = std::result::Result<T, PrefixError>;
