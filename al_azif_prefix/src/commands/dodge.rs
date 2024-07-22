use crate::_prelude::*;

pub const TAG: &str = "dodge";
pub const ALIASES: [&str; 2] = ["dodge", "desviar"];
pub const EVASION_BONUS: i64 = 0;

pub async fn run_command<'a>(bot: &impl AsBot, msg: &Message) -> Result<Responses<'a>> {
    let battle_tag = msg.channel_id.to_string();
    let Ok(battle_m) = Mirror::<Battle>::get(bot, &battle_tag).await else {
        return response::simple_send_and_delete_with_original(
            "Nenhuma batalha ocorrendo neste canal.",
        );
    };
    let mut battle = battle_m.write().await;

    let Moment::AttackPrimary {
        primary_action_tag,
        attacker_tag,
        target_tag,
        ..
    } = &battle.current_moment
    else {
        return response::simple_send_and_delete_with_original(
            "Você não pode usar uma Ação Reativa agora.",
        );
    };

    let target_m = Mirror::<Id>::get(bot, target_tag).await?;
    let attacker_m = Mirror::<Id>::get(bot, attacker_tag).await?;

    let mut blueprints = Vec::new();

    let target = target_m.read().await;

    blueprints.extend(generate_preliminary_responses(&target));

    let security_key = Timestamp::now().unix_timestamp();

    let dispute = Dispute::new(f!("{battle_tag}-{security_key}"), vec!["prefix", "dodge"])
        .add_member(
            DisputeMember::Test(Test::new(target_tag, TestKind::EvasionTest)
                .set_advantage_bonus(EVASION_BONUS)
            )
        )
        .add_member(
            DisputeMember::Test(Test::new(target_tag, TestKind::AccuracyTest)
                .set_advantage_bonus(get_accuracy_bonus_of_attack(primary_action_tag))
            )
        );

    battle.current_moment = Moment::AttackReactive {
        primary_action_tag: primary_action_tag.clone(),
        attacker_tag: attacker_tag.clone(),
        target_tag: target_tag.clone(),
        security_key,
    };

    blueprints.push(dispute.get_response_blueprint(bot).await?);

    Ok(vec![Response::send(blueprints)])
}

pub async fn run_component<'a>(
    bot: &impl AsBot,
    comp: &'a ComponentInteraction,
    args: &[&str],
) -> Result<Responses<'a>> {
    let Ok(battle_m) = Mirror::<Battle>::get(bot, &comp.channel_id.to_string()).await else {
        return response::simple_send_and_delete("Nenhuma batalha ocorrendo neste canal.");
    };
    let mut battle = battle_m.write().await;

    let Moment::AttackReactive {
        primary_action_tag,
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

    let Ok(dispute_m) = Mirror::<Dispute>::get(bot, args[0].parse::<i64>()?).await else {
        Ok(Vec::new())
    };

    let target_m = Mirror::<Id>::get(bot, target_tag).await?;
    let attacker_m = Mirror::<Id>::get(bot, attacker_tag).await?;

    let button_column = args[1].parse()?;
    let (outcome, summary) = match button_column {
        0 => {
            let target = target_m.read().await;
            al_azif_utils::roll::execute_expression(3, target.dexterity, EVASION_BONUS + 3)
        },
        1 => {
            let attacker = attacker_m.read().await;
            al_azif_utils::roll::execute_expression(3, attacker.dexterity, get_accuracy_bonus_of_attack(primary_action_tag) - 4)
        }
        _ => unreachable!("Invalid button column for 'dodge' component interaction (neither 0 or 1): {button_column}")
    };

    let are_all_buttons_disabled = dispute.are_all_other_buttons_disabled(button_column);
    let outcomes = dispute.outcomes();

    let mut responses = vec![Response::update_delayless(
        dispute.create_response_after_button_press(button_column, outcome, summary),
    )];

    if are_all_buttons_disabled {
        let (target_value, attacker_value) = match button_column {
            0 => (outcome, outcomes[1].unwrap()),
            _ => (outcomes[0].unwrap(), outcome),
        };

        let mut blueprints = Vec::new();

        if target_value >= attacker_value {
            let target = target_m.read().await;
            blueprints.push(
                ResponseBlueprint::default()
                    .set_content(f!("✅ | **{}** conseguiu desviar.", target.name)),
            );
        } else {
            let mut attacker = attacker_m.write().await;
            let mut target = target_m.write().await;
            blueprints.push(
                ResponseBlueprint::default()
                    .set_content(f!("❌ | **{}** não conseguiu desviar.", target.name)),
            );
            blueprints.extend(execute_attack(
                primary_action_tag,
                &mut attacker,
                &mut target,
            ));
        }

        battle.current_moment = Moment::None;
        blueprints.extend(advance(bot, &mut battle).await?);
        blueprints.push(battle.generate_turn_screen(bot).await?);

        responses.push(Response::send_loose(blueprints));
    }

    Ok(responses)
}

fn generate_preliminary_responses<'a>(target: &Id) -> Vec<ResponseBlueprint<'a>> {
    vec![ResponseBlueprint::default().set_content(f!("🔄 | **{}** decide desviar.", target.name,))]
}
