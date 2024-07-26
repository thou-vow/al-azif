#![feature(linked_list_cursors)]

pub mod _prelude;
pub mod battle;
pub mod bot;
pub mod constants;
pub mod database;
pub mod effect;
pub mod id;
pub mod mirror;
pub mod player;
pub mod request_reaction;
pub mod response;
pub mod utils {
    pub mod fmt;
    pub mod parse_args;
    pub mod roll;
    pub mod serenity;
}

use crate::_prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ParseArgs(#[from] ParseArgsError),
    #[error(transparent)]
    Database(#[from] DatabaseError),
}