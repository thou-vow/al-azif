use crate::prelude::*;

pub const TAG: &str = "dodge";
pub const ALIASES: [&str; 2] = ["dodge", "desviar"];
pub const EVASION_BONUS: i64 = 0;

pub async fn run_command<'a>(bot: &impl AsBot, msg: &Message) -> Result<Vec<ResponseModel<'a>>> {
    let Ok(battle_m) = Mirror::<Battle>::get(bot, msg.channel_id.to_string()).await else {
        return response::simple_send("Nenhuma batalha ocorrendo neste canal.");
    };
    let mut battle = battle_m.write().await;

    let Moment::AttackPrimary {
        action_tag,
        attacker_tag,
        target_tag,
        ..
    } = &battle.current_moment
    else {
        return response::simple_send("Você não pode usar uma Ação Reativa agora.");
    };

    let target_m = Mirror::<Id>::get(bot, target_tag).await?;
    let attacker_m = Mirror::<Id>::get(bot, attacker_tag).await?;

    let mut blueprints = Vec::new();

    let target = target_m.read().await;

    blueprints.extend(generate_preliminary_responses(&target));

    let attacker = attacker_m.read().await;

    let embed = CreateEmbed::new()
        .title("Desvio")
        .field(
            "Aguardando interação...",
            f!(
                "**{}**: d*{}* 🎉 **{EVASION_BONUS}**",
                target.name,
                target.dexterity
            ),
            true,
        )
        .field(
            "Aguardando interação...",
            f!(
                "**{}**: d*{}* 🎉 **{}**",
                attacker.name,
                attacker.dexterity,
                get_accuracy_bonus_of_attack(action_tag)
            ),
            true,
        );

    let security_key = Timestamp::now().unix_timestamp();

    battle.current_moment = Moment::AttackReactive {
        action_tag: action_tag.clone(),
        attacker_tag: attacker_tag.clone(),
        target_tag: target_tag.clone(),
        security_key,
    };

    let button_row = CreateActionRow::Buttons(vec![
        CreateButton::new(f!("prefix dodge {} 0", security_key))
            .emoji(ReactionType::Unicode("🎲".parse()?)),
        CreateButton::new(f!("prefix dodge {} 1", security_key))
            .emoji(ReactionType::Unicode("🎲".parse()?))
            .style(ButtonStyle::Danger),
    ]);

    blueprints.push(
        ResponseBlueprint::default()
            .assign_embeds(vec![embed])
            .assign_components(vec![button_row]),
    );

    Ok(vec![ResponseModel::send(blueprints)])
}

pub async fn run_component<'a>(
    bot: &impl AsBot,
    comp: &'a ComponentInteraction,
    args: &[&str],
) -> Result<Models<'a>> {
    let battle_m = Mirror::<Battle>::get(bot, &comp.channel_id.to_string()).await?;
    let mut battle = battle_m.write().await;

    let Moment::AttackReactive {
        action_tag,
        attacker_tag,
        target_tag,
        security_key,
    } = &battle.current_moment
    else {
        return Ok(Vec::new());
    };

    if *security_key != args[0].parse::<i64>()? {
        return Ok(Vec::new());
    }

    let original_roll_event = OriginalRollEvent::from_message(&comp.message);

    let target_m = Mirror::<Id>::get(bot, target_tag).await?;
    let attacker_m = Mirror::<Id>::get(bot, attacker_tag).await?;

    let button_column = args[1].parse()?;
    let (outcome, summary) = match button_column {
        0 => {
            let target = target_m.read().await;
            
                al_azif_utils::math::roll::execute_expression(1, target.dexterity, EVASION_BONUS)
        },
        1 => {
            let attacker = attacker_m.read().await;
                al_azif_utils::math::roll::execute_expression(1, attacker.dexterity, get_accuracy_bonus_of_attack(action_tag))
        
        }
        _ => unreachable!("Invalid button column for 'dodge' component interaction (neither 0 or 1): {button_column}")
    };

    let are_all_buttons_disabled =
        original_roll_event.are_all_other_buttons_disabled(button_column);
    let original_outcomes = original_roll_event.outcomes();

    let mut models = vec![ResponseModel::update(
        original_roll_event.after_button_press_blueprint(button_column, outcome, summary),
    )];

    if are_all_buttons_disabled {
        let (target_value, attacker_value) = match button_column {
            0 => (outcome, original_outcomes[1].unwrap()),
            _ => (original_outcomes[0].unwrap(), outcome),
        };

        let mut blueprints = Vec::new();

        if target_value >= attacker_value {
            let target = target_m.read().await;
            blueprints.push(
                ResponseBlueprint::default()
                    .assign_content(f!("✅ | **{}** conseguiu desviar.", target.name)),
            );
        } else {
            let attacker = attacker_m.read().await;
            let mut target = target_m.write().await;
            blueprints.push(
                ResponseBlueprint::default()
                    .assign_content(f!("❌ | **{}** não conseguiu desviar.", target.name)),
            );
            blueprints.extend(execute_action(action_tag, &attacker, &mut target)?);
        }

        battle.current_moment = Moment::None;
        blueprints.extend(advance(bot, &mut battle).await?);
        blueprints.push(battle.generate_turn_screen(bot).await?);

        models.push(ResponseModel::send_loose(blueprints));
    }

    Ok(models)
}

fn generate_preliminary_responses<'a>(target: &Id) -> Vec<ResponseBlueprint<'a>> {
    vec![ResponseBlueprint::default()
        .assign_content(f!("🔄 | **{}** decide desviar.", target.name,))]
}
