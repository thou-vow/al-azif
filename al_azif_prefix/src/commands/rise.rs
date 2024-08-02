use crate::_prelude::*;

pub const NAME: &str = "rise";
pub const NAME_PT: &str = "elevar";

pub async fn run_prefix(bot: &impl AsBot, msg: &Message, args: VecDeque<&str>) -> Result<Responses> {
    let setting = Setting::new(bot, args)
        .fetch_battle(msg.channel_id.to_string())
        .await?
        .require_primary_moment()
        .await?
        .fetch_user()
        .await?;

    let mut blueprints = Vec::new();

    let mut battle = setting.get_battle_mirror().write().await;
    let mut user = setting.get_user_mirror().write().await;

    let might_bonus = user.might / 2;
    blueprints.extend(user.acquire_effect(bot, RiseEffect { might_bonus, turn_duration: 10 }));

    user.unwrite();

    blueprints.extend(battle.advance(bot).await?);
    blueprints.push(battle.generate_turn_screen(bot).await?);

    Ok(vec![Response::send(blueprints)])
}
