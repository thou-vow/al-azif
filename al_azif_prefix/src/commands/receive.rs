use crate::_prelude::*;

pub const NAME: &str = "receive";
pub const NAME_PT: &str = "receber";

pub async fn run_component<'a>(bot: &impl AsBot, comp: &ComponentInteraction) -> Result<Responses<'a>> {
    let Ok(battle_m) = Mirror::<Battle>::get(bot, comp.channel_id.to_string()).await else {
        // No battle is currently happening in this channel.
        return Ok(Vec::new());
    };

    let mut battle = battle_m.write().await;

    let Moment::PrimaryAction { primary_action_tag, attacker_tag, target_tag } = &battle.current_moment else {
        // Attack didn't start yet (receive action is not available)
        return Ok(Vec::new());
    };

    let mut blueprints = Vec::new();

    let target_m = Mirror::<Id>::get(bot, &target_tag).await?;
    let mut target = target_m.write().await;
    let attacker_m = Mirror::<Id>::get(bot, &attacker_tag).await?;
    let mut attacker = attacker_m.write().await;

    blueprints.extend(handler::execute_attack(primary_action_tag, &mut attacker, &mut target)?);

    mem::drop(target);
    mem::drop(attacker);

    battle.current_moment = Moment::None;
    blueprints.extend(advance(bot, &mut battle).await.map_err(PrefixError::Core)?);
    blueprints.push(battle.generate_turn_screen(bot).await.map_err(PrefixError::Core)?);

    Ok(vec![Response::send_loose(blueprints)])
}
