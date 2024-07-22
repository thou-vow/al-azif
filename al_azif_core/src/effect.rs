use crate::_prelude::*;

#[derive(Deserialize, Serialize)]
pub enum Effect {
    Block,
    Rise {
        might_bonus: i64,
        turn_duration: i64,
    },
}
