use crate::_prelude::*;

pub enum ComponentArgs<'a> {
    Slash { args: Vec<&'a str> },
    Unclassified { args: Vec<&'a str> },
}