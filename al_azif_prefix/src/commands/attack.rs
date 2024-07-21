use crate::_prelude::*;

pub const TAG: &str = "attack";
pub const ALIASES: [&str; 2] = ["attack", "atacar"];
pub const ACCURACY_BONUS: i64 = 0;

pub async fn run_command<'a>(
    bot: &impl AsBot,
    msg: &Message,
    args: &[&str],
) -> Result<Responses<'a>> {
    let Ok(battle_m) = Mirror::<Battle>::get(bot, msg.channel_id.to_string()).await else {
        return response::simple_send_and_delete_with_original(
            "Nenhuma batalha ocorrendo neste canal.",
        );
    };
    let mut battle = battle_m.write().await;

    let Moment::None = battle.current_moment else {
        return response::simple_send_and_delete_with_original("Você não pode usar agora.");
    };

    let Some(target_tag) = args.first() else {
        return response::simple_send_and_delete_with_original("O argumento 'alvo' é obrigatório.");
    };

    if !battle
        .opponents
        .contains_key(&FixedString::from_str_trunc(target_tag))
    {
        return response::simple_send_and_delete_with_original("O alvo não está na batalha.");
    }

    let Ok(target_m) = Mirror::<Id>::get(bot, target_tag).await else {
        return response::simple_send_and_delete_with_original("O alvo não existe.");
    };

    if *target_tag == battle.current_turn_owner_tag {
        return response::simple_send_and_delete_with_original(
            "O alvo não pode ser o próprio usuário.",
        );
    }

    let mut blueprints = Vec::new();

    let attacker_m = Mirror::<Id>::get(bot, &battle.current_turn_owner_tag).await?;

    let mut attacker = attacker_m.write().await;
    let target = target_m.read().await;

    blueprints.extend(generate_preliminary_responses(&attacker, &target));
    blueprints.extend(generate_forecast_responses(&mut attacker, &target));

    let security_key = Timestamp::now().unix_timestamp();

    blueprints.push(request_reaction::create(
        f!("⏳ | **{}**, é a vez de sua reação.", target.name),
        security_key,
    )?);

    battle.current_moment = Moment::AttackPrimary {
        primary_action_tag: FixedString::from_static_trunc(TAG),
        attacker_tag: attacker.tag.clone(),
        target_tag: FixedString::from_str_trunc(target_tag),
        security_key,
    };

    Ok(vec![Response::send(blueprints)])
}

fn generate_preliminary_responses<'a>(user: &Id, target: &Id) -> Blueprints<'a> {
    vec![ResponseBlueprint::default().set_content(f!(
        "{STRIKE_EMOJI} | **{}** irá atacar **{}**.",
        user.name,
        target.name,
    ))]
}

fn generate_forecast_responses<'a>(attacker: &mut Id, target: &Id) -> Blueprints<'a> {
    let attacker_damage_evaluation = attacker.might + attacker.evaluate_might_bonuses();

    let content = match attacker_damage_evaluation * 100 / (target.constitution * 10) {
        0..=5 => fc!("{LIGHT_EMOJI} | Parece que irá causar um dano leve."),
        6..=10 => fc!("{MEDIUM_EMOJI} | Parece que irá causar um dano moderado."),
        11..=20 => fc!("{HEAVY_EMOJI} | Parece que irá causar um dano grave."),
        _ => fc!("{SEVERE_EMOJI} | Parece que irá causar um dano *severo*."),
    };

    vec![ResponseBlueprint::default().set_content(content)]
}

pub fn execute<'a>(attacker: &mut Id, target: &mut Id) -> Blueprints<'a> {
    let attacker_damage_evaluation = attacker.might + attacker.evaluate_might_bonuses();
    target.take_damage(attacker_damage_evaluation)
}
