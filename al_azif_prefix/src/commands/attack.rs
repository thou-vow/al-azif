use crate::prelude::*;

pub const TAG: &str = "attack";
pub const ALIASES: [&str; 2] = ["attack", "atacar"];
pub const NAME: &str = "Ataque";
pub const ACCURACY_BONUS: i64 = 0;

pub async fn run_command<'a>(bot: &impl AsBot, msg: &Message, args: &[&str]) -> Result<Models<'a>> {
    let Ok(battle_m) = Mirror::<Battle>::get(bot, msg.channel_id.to_string()).await else {
        return response::simple_send("Nenhuma batalha ocorrendo neste canal.");
    };

    let mut battle = battle_m.write().await;

    let Moment::None = battle.current_moment else {
        return response::simple_send("Você não pode usar agora.");
    };

    let Some(target_tag) = args.first() else {
        return response::simple_send("O argumento 'alvo' é obrigatório.");
    };

    if !battle
        .opponents
        .contains_key(&FixedString::from_str_trunc(target_tag))
    {
        return response::simple_send("O alvo não está na batalha.");
    }

    let Ok(target_m) = Mirror::<Id>::get(bot, target_tag).await else {
        return response::simple_send("O alvo não existe.");
    };

    if *target_tag == battle.current_turn_owner_tag {
        return response::simple_send("O alvo não pode ser o próprio usuário.");
    }

    let attacker_m = Mirror::<Id>::get(bot, &battle.current_turn_owner_tag).await?;

    let mut blueprints = Vec::new();

    let attacker = attacker_m.read().await;
    let target = target_m.read().await;

    blueprints.extend(generate_preliminary_responses(&attacker, &target));
    blueprints.extend(generate_forecast_responses(&attacker, &target));

    let security_key = Timestamp::now().unix_timestamp();

    blueprints.push(event::request_reaction::create(
        f!("⏳ | **{}**, é a vez de sua reação.", target.name),
        security_key,
    )?);

    battle.current_moment = Moment::AttackPrimary {
        action_tag: FixedString::from_static_trunc(TAG),
        attacker_tag: attacker.tag.clone(),
        target_tag: FixedString::from_str_trunc(target_tag),
        security_key,
    };

    Ok(vec![ResponseModel::send(blueprints)])
}

fn generate_preliminary_responses<'a>(user: &Id, target: &Id) -> Blueprints<'a> {
    vec![ResponseBlueprint::default().assign_content(f!(
        "{STRIKE_EMOJI} | **{}** irá atacar **{}**.",
        user.name,
        target.name,
    ))]
}

fn generate_forecast_responses<'a>(user: &Id, target: &Id) -> Blueprints<'a> {
    let evaluated_damage = target.evaluate_damage_to_receive(user.might);

    let content = match evaluated_damage * 100 / (target.constitution * 10) {
        0..=5 => fc!("{LIGHT_EMOJI} | Parece que irá causar um dano leve."),
        6..=10 => fc!("{MEDIUM_EMOJI} | Parece que irá causar um dano moderado."),
        11..=20 => fc!("{HEAVY_EMOJI} | Parece que irá causar um dano grave."),
        _ => fc!("{SEVERE_EMOJI} | Parece que irá causar um dano *severo*."),
    };

    vec![ResponseBlueprint::default().assign_content(content)]
}

pub fn execute<'a>(user: &Id, target: &mut Id) -> Result<Blueprints<'a>> {
    let previous_hp = target.hp;
    target.receive_damage(user.might);

    Ok(vec![ResponseBlueprint::default().assign_content(f!(
        "{STRIKE_EMOJI} | **{}** recebeu **{}** de dano. [{HP_SHORT}: **{}** → **{}**]",
        target.name,
        mark_thousands(user.might),
        mark_thousands(previous_hp),
        mark_thousands(target.hp),
    ))])
}
