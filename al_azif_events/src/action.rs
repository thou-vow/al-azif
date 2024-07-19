use crate::prelude::*;

pub async fn receive<'a>(
    bot: &impl AsBot,
    comp: &ComponentInteraction,
    button_security_key: i64,
) -> Result<Models<'a>> {
    let Ok(battle_m) = Mirror::<Battle>::get(bot, comp.channel_id.to_string()).await else {
        // No battle is currently happening in this channel.
        return Ok(Vec::new());
    };

    let mut battle = battle_m.write().await;

    let Moment::AttackPrimary {
        action_tag,
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

    let mut models = vec![ResponseModel::update(
        event::request_reaction::disable_button(&comp.message, 0),
    )];

    let mut blueprints = Vec::new();

    let target_m = Mirror::<Id>::get(bot, &target_tag).await?;
    let attacker_m = Mirror::<Id>::get(bot, &attacker_tag).await?;

    blueprints.extend(al_azif_prefix::utils::execute_action(
        action_tag,
        &*attacker_m.read().await,
        &mut *target_m.write().await,
    )?);

    battle.current_moment = Moment::None;
    blueprints.extend(advance(bot, &mut battle).await?);
    blueprints.push(battle.generate_turn_screen(bot).await?);

    models.push(ResponseModel::send_loose(blueprints));

    Ok(models)
}
/* pub async fn receive<'a>(bot: &impl AsBot, comp: &ComponentInteraction, security_key: i64) -> Result<Models<'a>> {
    let Ok(battle_m) = Mirror::<Battle>::get(bot, &comp.channel_id.to_string()).await else {
        return Ok(Vec::new());
    };

    let mut battle = battle_m.write().await;

    let Moment::AttackPrimary {
        action_tag,
        user_tag: attacker_tag,
        target_tag: user_tag,
        security_key: moment_security_key,
    } = &battle.current_moment else {
        return Ok(Vec::new());
    };

    if *moment_security_key != security_key {
        return Ok(Vec::new());
    }

    let mut buttons = comp
        .message
        .components
        .first()
        .unwrap()
        .components
        .iter()
        .filter_map(|action_row_comp| {
            if let ActionRowComponent::Button(button) = action_row_comp {
                Some(button)
            } else {
                None
            }
        })
        .map(ToOwned::to_owned)
        .map(CreateButton::from)
        .collect::<Vec<CreateButton>>();

    buttons[0] = buttons[0].clone().disabled(true);

    let mut models = vec![ResponseModel::update(
        ResponseBlueprint::default()
            .content(comp.message.content.clone())
            .embeds(
                comp.message
                    .embeds
                    .iter()
                    .map(ToOwned::to_owned)
                    .map(CreateEmbed::from)
                    .collect::<Vec<CreateEmbed>>(),
            )
            .components(vec![CreateActionRow::Buttons(buttons)]),
    )];

    let mut blueprints = Vec::new();

    let user_m = Mirror::<Id>::get(bot, &user_tag).await?;
    let attacker_m = Mirror::<Id>::get(bot, &attacker_tag).await?;

    let attacker = attacker_m.read().await;
    let mut user = user_m.write().await;

    blueprints.extend(al_azif_prefix::utils::execute_action(action_tag, &attacker, &mut user)?);

    battle.current_moment = Moment::None;
    blueprints.extend(advance(bot, &mut battle).await?);
    blueprints.push(battle.generate_turn_screen(bot).await?);

    models.push(ResponseModel::send_loose(blueprints));

    Ok(models)
} */
