use crate::_prelude::*;

pub const TAG: &str = "block";
pub const TAG_PT: &str = "bloquear";

pub async fn run_prefix(bot: &impl AsBot, msg: &Message, args: VecDeque<&str>) -> Result<Responses> {
    let setting = Setting::new(bot, args)
        .fetch_battle(msg.channel_id.to_string())
        .await?
        .require_reactive_moment()
        .await?
        .fetch_user()
        .await?;

    let mut blueprints = Vec::new();

    let emitter_m = Mirror::<Id>::get(bot, setting.get_primary_moment_owner_tag()).await?;
    let mut battle = setting.get_battle_mirror().write().await;
    let mut user = setting.get_user_mirror().write().await;
    let mut emitter = emitter_m.write().await;

    blueprints.extend(user.acquire_effect(bot, BlockEffect));
    blueprints.extend(handler::execute_primary_action(bot, setting.get_primary_action_tag(), &mut emitter, &mut user)?);

    user.unwrite();
    emitter.unwrite();

    blueprints.extend(battle.advance(bot).await?);
    blueprints.push(battle.generate_turn_screen(bot).await?);

    Ok(vec![Response::send(blueprints)])
}
