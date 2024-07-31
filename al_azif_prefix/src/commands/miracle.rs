use crate::_prelude::*;

pub const NAME: &str = "miracle";
pub const NAME_PT: &str = "milagre";

pub async fn run_prefix<'a>(bot: &impl AsBot, msg: &Message) -> Result<Responses<'a>> {
    let (battle_m, user_m) = {
        let Ok(battle_m) = Mirror::<Battle>::get(bot, msg.channel_id.to_string()).await else {
            return Ok(response::simple_send_and_delete_with_original(lang_diff!(bot,
                en: "No battle is currently happening in this channel.",
                pt: "Não há uma batalha acontecendo neste canal."
            )));
        };
        let battle = battle_m.read().await;

        let Moment::None = battle.current_moment else {
            return Ok(response::simple_send_and_delete_with_original(lang_diff!(bot,
                en: "You can't use this command right now.",
                pt: "Você não pode usar este comando agora."
            )));
        };

        let user_m = Mirror::<Id>::get(bot, &battle.current_turn_owner_tag).await?;

        mem::drop(battle);

        (battle_m, user_m)
    };

    let mut blueprints = main_logic(bot, battle_m.clone(), user_m).await?;

    let mut battle = battle_m.write().await;

    blueprints.extend(battle.advance(bot).await?);
    blueprints.push(battle.generate_turn_screen(bot).await?);

    Ok(vec![Response::send(blueprints)])
}

async fn main_logic<'a>(bot: &impl AsBot, battle_m: Mirror<Battle>, user_m: Mirror<Id>) -> Result<Blueprints<'a>> {
    let mut blueprints = Vec::new();

    let mut _battle = battle_m.write().await;
    let mut user = user_m.write().await;

    blueprints.extend(user.acquire_effect(bot, MiracleEffect));

    Ok(blueprints)
}
