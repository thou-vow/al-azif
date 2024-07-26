use crate::_prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse component arg '{arg}' into {into_type}")]
    CouldNotParseComponentArg { arg: String, into_type: &'static str },
}

#[macro_export]
macro_rules! parse_comp_arg {
    ($arg:expr, $t:ty) => {
        $arg.parse::<$t>().map_err(|_| {
            CoreError::ParseArgs(ParseArgsError::CouldNotParseComponentArg {
                arg:       $arg.to_string(),
                into_type: stringify!($t),
            })
        })
    };
}
