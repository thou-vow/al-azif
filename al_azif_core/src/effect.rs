use crate::_prelude::*;

#[repr(u64)]
#[derive(Deserialize, Serialize)]
pub enum Effect {
    Block = 0,
    Rise { might_bonus: i64, turn_duration: i64 } = 1,
}
