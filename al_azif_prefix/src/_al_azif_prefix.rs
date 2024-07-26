pub mod commands {
    pub mod attack;
    pub mod block;
    pub mod receive;
    pub mod rise;
}
pub mod handlers {
    pub mod attack;
}
pub mod _prelude;
pub mod utils;

use crate::_prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] CoreError),
}
