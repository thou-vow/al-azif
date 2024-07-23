use crate::_prelude::*;

pub async fn run_component<'a>(
    bot: &impl AsBot,
    comp: &ComponentInteraction,
    button_security_key: i64,
) -> Result<Responses<'a>> {
    let Ok(battle_m) = Mirror::<Battle>::get(bot, comp.channel_id.to_string()).await else {
        // No battle is currently happening in this channel.
        return Ok(Vec::new());
    };

    let mut battle = battle_m.write().await;

    let Moment::PrimaryAction {
        primary_action_tag,
        attacker_tag,
        target_tag,
        security_key,
    } = &battle.current_moment
    else {
        // Attack didn't start yet (receive action is not available)
        return Ok(Vec::new());
    };

    if button_security_key != *security_key {
        // The moment to press it already passed
        return Ok(Vec::new());
    }

    let mut responses = vec![Response::update_delayless(
        request_reaction::disable_button(&comp.message, 0),
    )];

    let mut blueprints = Vec::new();

    let target_m = Mirror::<Id>::get(bot, &target_tag).await?;
    let mut target = target_m.write().await;
    let attacker_m = Mirror::<Id>::get(bot, &attacker_tag).await?;
    let mut attacker = attacker_m.write().await;

    blueprints.extend(al_azif_prefix::utils::execute_attack(
        primary_action_tag,
        &mut attacker,
        &mut target,
    ));

    mem::drop(target);
    mem::drop(attacker);

    battle.current_moment = Moment::None;
    blueprints.extend(advance(bot, &mut battle).await?);
    blueprints.push(battle.generate_turn_screen(bot).await?);

    responses.push(Response::send_loose(blueprints));

    Ok(responses)
}