use crate::prelude::*;

pub const ALIASES: [&str; 2] = ["dodge", "desviar"];

pub async fn run_command(bot: &impl AsBot, msg: &Message) -> ResponseResult {
    let Ok(battle_m) = Mirror::<Battle>::get(bot, &msg.channel_id.to_string()).await else {
        return simple_response("Nenhuma batalha ocorrendo neste canal.", ResponseMode::Delete);
    };
    
    panic!();
}