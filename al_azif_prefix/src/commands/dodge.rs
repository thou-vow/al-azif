use crate::prelude::*;

pub const TAG: &str = "dodge";
pub const ALIASES: [&str; 2] = ["dodge", "desviar"];
pub const EVASION_BONUS: i64 = 0;

pub async fn run_command(bot: &impl AsBot, msg: &Message) -> Result<Vec<ResponseModel>> {
    let Ok(battle_m) = Mirror::<Battle>::get(bot, &msg.channel_id.to_string()).await else {
        return response::simple_send("Nenhuma batalha ocorrendo neste canal.");
    };
    let mut battle = battle_m.write().await;

    let Moment::AttackPrimary {
        action_tag,
        user_tag: attacker_tag,
        target_tag: user_tag,
        ..
    } = &battle.current_moment else {
        return response::simple_send("Você não pode usar uma Ação Reativa agora.");
    };

    let user_m = Mirror::<Id>::get(bot, &user_tag).await?;
    let attacker_m = Mirror::<Id>::get(bot, &attacker_tag).await?;

    let mut blueprints = Vec::new();

    let user = user_m.read().await;

    blueprints.extend(generate_preliminary_responses(&user));

    let attacker = attacker_m.read().await;

    let embed = CreateEmbed::new()
        .title("Desvio")
        .field("Aguardando interação...", f!("**{}**: d*{}* 🎉 **{EVASION_BONUS}**", user.name, user.dexterity), true)
        .field("Aguardando interação...", 
            f!("**{}**: d*{}* 🎉 **{}**", attacker.name, attacker.dexterity, get_accuracy_bonus_of_attack(action_tag)),
            true
        );

    let security_key = Timestamp::now().unix_timestamp();

    battle.current_moment = Moment::AttackReactive { 
        action_tag: action_tag.clone(),
        user_tag: attacker_tag.clone(),
        target_tag: user_tag.clone(),
        security_key
    };

    let button_row = CreateActionRow::Buttons(vec![
        CreateButton::new(f!("prefix dodge {} user", security_key)).emoji(ReactionType::Unicode("🎲".parse()?)), 
        CreateButton::new(f!("prefix dodge {} attacker", security_key)).emoji(ReactionType::Unicode("🎲".parse()?)).style(ButtonStyle::Danger),
    ]);

    blueprints.push(ResponseBlueprint::default().embeds(vec![embed]).components(vec![button_row]));

    Ok(vec![ResponseModel::send(blueprints)])
}

pub async fn run_component(bot: &impl AsBot, comp: &ComponentInteraction, args: &[&str]) -> Result<Vec<ResponseModel>> {
    let battle_m = Mirror::<Battle>::get(bot, &comp.channel_id.to_string()).await?;
    let mut battle = battle_m.write().await;

    let Moment::AttackReactive { 
        action_tag,
        user_tag: attacker_tag,
        target_tag: user_tag,
        security_key
    } = &battle.current_moment else {
        return Ok(Vec::new());
    };

    if *security_key != args[0].parse::<i64>()? {
        return Ok(Vec::new());
    }

    let Message { embeds, components, .. } = &*comp.message;

    let original_embed = embeds.first().unwrap();
    let mut embed = CreateEmbed::new();

    let original_buttons = components
        .first().unwrap()
        .components.iter()
        .filter_map(|component| {
            if let ActionRowComponent::Button(button) = component {
                Some(button)
            } else {
                None
            }
        })
        .collect::<Vec<&Button>>();

    let mut buttons = original_buttons
        .iter()
        .map(ToOwned::to_owned)
        .map(Clone::clone)
        .map(CreateButton::from)
        .collect::<Vec<CreateButton>>();

    embed = embed.title(original_embed.title.as_ref().unwrap().clone());

    let mut models = Vec::new();

    let user_m = Mirror::<Id>::get(bot, &user_tag).await?;
    let attacker_m = Mirror::<Id>::get(bot, &attacker_tag).await?;

    let (user_value, attacker_value) = match args[1] {
        "user" => {
            let user = user_m.read().await;

            let (outcome, summary)
                = math::roll::execute_roll_expression(1, user.dexterity, EVASION_BONUS);

            buttons[0] = buttons[0].clone().label(outcome.to_string()).disabled(true);

            embed = embed
                .field(f!("{outcome}"),
                    f!("{}\n{summary}", original_embed.fields[0].value),
                    true
                )
                .field(original_embed.fields[1].name.clone(), 
                    original_embed.fields[1].value.clone(), 
                    true
                );

            models.push(ResponseModel::update(
                ResponseBlueprint::default().embeds(vec![embed]).components(vec![CreateActionRow::Buttons(buttons)])
            ));

            if !original_buttons[1].disabled {
                return Ok(models);
            }

            (outcome, original_buttons[1].label.as_ref().unwrap().parse()?)
        },
        "attacker" => {
            let attacker = attacker_m.read().await;

            let (outcome, summary)
                = math::roll::execute_roll_expression(1, attacker.dexterity, get_accuracy_bonus_of_attack(action_tag));

            buttons[1] = buttons[1].clone().label(outcome.to_string()).disabled(true);

            embed = embed
                .field(original_embed.fields[0].name.clone(),
                    original_embed.fields[0].value.clone(),
                    true
                )
                .field(f!("{outcome}"),
                    f!("{}\n{summary}", original_embed.fields[1].value),
                    true
                );

            models.push(ResponseModel::update(
                ResponseBlueprint::default().embeds(vec![embed]).components(vec![CreateActionRow::Buttons(buttons)])
            ));

            if !original_buttons[0].disabled {
                return Ok(models);
            }

            (original_buttons[0].label.as_ref().unwrap().parse()?, outcome)
        },
        _ => unreachable!("Invalid character for 'dodge' component interaction (neither 'user' or 'attacker'): {}", args[0]),
    };

    let mut blueprints = Vec::new();

    if user_value >= attacker_value {
        let user = user_m.read().await;
        blueprints.push(ResponseBlueprint::default().content(f!("✅ | **{}** conseguiu desviar.", user.name)));
    } else {
        let attacker = attacker_m.read().await;
        let mut user = user_m.write().await;
        blueprints.push(ResponseBlueprint::default().content(f!("❌ | **{}** não conseguiu desviar.", user.name)));
        blueprints.extend(execute_action(action_tag, &attacker, &mut user)?);
    }

    battle.current_moment = Moment::None;
    blueprints.extend(advance(bot, &mut battle).await?);
    blueprints.push(battle.generate_turn_screen(bot).await?);

    models.push(ResponseModel::send_loose(blueprints));

    Ok(models)
}

fn generate_preliminary_responses(user: &Id) -> Vec<ResponseBlueprint> {
    vec![ResponseBlueprint::default().content(f!(
        "🔄 | **{}** decide desviar.",
        user.name,
    ))]
}
