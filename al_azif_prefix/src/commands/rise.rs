use crate::_prelude::*;

pub const NAME: &str = "rise";
pub const NAME_PT: &str = "elevar";

pub async fn run<'a>(bot: &impl AsBot, msg: &Message) -> Result<Responses<'a>> {
    let Ok(battle_m) = Mirror::<Battle>::get(bot, msg.channel_id.to_string()).await else {
        return Ok(response::simple_send_and_delete_with_original("Nenhuma batalha ocorrendo neste canal."));
    };
    let mut battle = battle_m.write().await;

    let Moment::None = battle.current_moment else {
        return Ok(response::simple_send_and_delete_with_original("Você não pode usar agora."));
    };

    let user_m = Mirror::<Id>::get(bot, &battle.current_turn_owner_tag).await?;

    let mut user = user_m.write().await;

    let might_bonus = user.might / 2;
    user.acquire_effect(Effect::Rise { might_bonus, turn_duration: 1 });

    let mut blueprints = Vec::new();

    blueprints.push(ResponseBlueprint::new().set_content(f!(
        "💪 | **{}** adquiriu o efeito **Subir** por **1** turno. [**{}** {MGT_EMOJI}]",
        user.name,
        mark_thousands_and_show_sign(might_bonus)
    )));

    mem::drop(user);

    blueprints.extend(advance(bot, &mut battle).await?);
    blueprints.push(battle.generate_turn_screen(bot).await?);

    Ok(vec![Response::send(blueprints)])
}