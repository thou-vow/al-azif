use crate::prelude::*;

pub const TAG: &str = "attack";
pub const ALIASES: [&str; 2] = ["attack", "atacar"];
pub const NAME: &str = "Ataque";
pub const ACCURACY_BONUS: i64 = 0;

pub async fn run_command(bot: &impl AsBot, msg: &Message, args: &[&str]) -> Result<Vec<ResponseModel>> {
    let Ok(battle_m) = Mirror::<Battle>::get(bot, &msg.channel_id.to_string()).await else {
        return response::simple_send("Nenhuma batalha ocorrendo neste canal.");
    };

    let mut battle = battle_m.write().await;

    let Moment::None = battle.current_moment else {
        return response::simple_send("Você não pode usar agora.");
    };

    let Some(target_tag) = args.first() else {
        return response::simple_send("O argumento 'alvo' é obrigatório.");
    };

    if !battle.opponents.contains_key(&FixedString::from_str_trunc(target_tag)) {
        return response::simple_send("O alvo não está na batalha.");
    }

    let Ok(target_m) = Mirror::<Id>::get(bot, target_tag).await else {
        return response::simple_send("O alvo não existe.");
    };

    if *target_tag == battle.current_turn_owner_tag {
        return response::simple_send("O alvo não pode ser o próprio usuário.");
    }

    let user_m = Mirror::<Id>::get(bot, &battle.current_turn_owner_tag).await?;

    let mut blueprints = Vec::new();
    
    let user = user_m.read().await;
    let target = target_m.read().await;

    blueprints.extend(generate_preliminary_responses(&user, &target));
    blueprints.extend(generate_forecast_responses(&user, &target));
    blueprints.push(ResponseBlueprint::default().content(f!("⏳ | **{}**, é a vez de sua reação.", target.name)));

    battle.current_moment = Moment::AttackAct {
        action_tag: FixedString::from_static_trunc(TAG),
        user_tag: user.tag.clone(),
        target_tag: FixedString::from_str_trunc(&target_tag)
    };
    battle.action_counter += 1;

    Ok(vec![ResponseModel::send(blueprints)])
}

fn generate_preliminary_responses(user: &Id, target: &Id) -> Vec<ResponseBlueprint> {
    vec![ResponseBlueprint::default().content(f!(
        "{STRIKE_EMOJI} | **{}** irá atacar **{}**.",
        user.name,
        target.name,
    ))]
}

fn generate_forecast_responses(user: &Id, target: &Id) -> Vec<ResponseBlueprint> {
    let evaluated_damage = target.evaluate_damage_to_receive(user.might);

    println!("{evaluated_damage} evaluated damage, {} max hp, {} formula result",
        target.constitution * 10, evaluated_damage * 100 / (target.constitution * 10)
    );

    let content = match evaluated_damage * 100 / (target.constitution * 10) {
        0..=5 => "🔎 | Parece que irá causar um dano leve.",
        6..=10 => "🔎 | Parece que irá causar um dano moderado.",
        11..=20 => "🔎 | Parece que irá causar um dano grave.",
        _ => "🔎 | Parece que irá causar um dano **severo**.",
    };

    vec![ResponseBlueprint::default().content(content)]
}

pub fn execute(user: &Id, target: &mut Id) -> Result<Vec<ResponseBlueprint>> {
    let previous_hp = target.hp;
    target.receive_damage(user.might);

    Ok(vec![ResponseBlueprint::default().content(f!(
        "{STRIKE_EMOJI} | **{}** recebeu **{}** de dano. [{HP_SHORT}: **{}** → **{}**]",
        target.name,
        mark_thousands(user.might),
        mark_thousands(previous_hp),
        mark_thousands(target.hp),
    ))])
}