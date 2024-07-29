#![feature(decl_macro, linked_list_cursors, try_trait_v2)]

pub mod _prelude;
pub mod battle;
pub mod bot;
pub mod constants;
pub mod database;
pub mod effect;
pub mod id;
pub mod mirror;
pub mod player;
pub mod response;
pub mod utils {
    pub mod fmt;
    pub mod roll;
    pub mod serenity;
}

use crate::_prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] DatabaseError),
}
