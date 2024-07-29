use crate::_prelude::*;

#[repr(u64)]
#[derive(Deserialize, Serialize)]
pub enum Effect {
    Bleed { damage_over_turn: i64, turn_duration: i64 },
    Block,
    Rise { might_bonus: i64, turn_duration: i64 },
}
