use crate::_prelude::*;

pub const NAME: &str = "receive";
pub const NAME_PT: &str = "receber";
pub const EMOJI: &str = "⏭";

pub async fn run_prefix(bot: &impl AsBot, msg: &Message, args: VecDeque<&str>) -> Result<Responses> {
    let setting = Setting::new(bot, args)
        .fetch_battle(msg.channel_id.to_string())
        .await?
        .require_reactive_moment()
        .await?
        .fetch_user()
        .await?;

    Ok(vec![Response::send(run(setting).await?)])
}

pub async fn run_component(bot: &impl AsBot, comp: &ComponentInteraction, args: VecDeque<&str>) -> Result<Responses> {
    let setting = Setting::new(bot, args)
        .fetch_battle(comp.channel_id.to_string())
        .await?
        .require_reactive_moment()
        .await?
        .fetch_user()
        .await?;

    Ok(vec![Response::send_loose(run(setting).await?)])
}

async fn run(setting: Setting<'_, impl AsBot, FBattle, FReactiveMoment, FUser, Empty, Empty>) -> Result<Blueprints> {
    let mut blueprints = Vec::new();

    let emitter_m = Mirror::<Id>::get(setting.bot, setting.get_primary_moment_owner_tag()).await?;
    let mut battle = setting.get_battle_mirror().write().await;
    let mut user = setting.get_user_mirror().write().await;
    let mut emitter = emitter_m.write().await;

    blueprints.extend(handler::execute_primary_action(setting.bot, setting.get_primary_action_tag(), &mut emitter, &mut user)?);

    user.unwrite();
    emitter.unwrite();

    blueprints.extend(battle.advance(setting.bot).await?);
    blueprints.push(battle.generate_turn_screen(setting.bot).await?);

    Ok(blueprints)
}
