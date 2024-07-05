use crate::prelude::*;

pub const TAG: &str = "attack";
pub const ALIASES: [&str; 2] = ["attack", "atacar"];
pub const NAME: &str = "Ataque";
pub const ACCURACY_BONUS: i64 = 0;

pub async fn run_command(bot: &impl AsBot, msg: &Message, args: &[&str]) -> Result<Vec<ResponseModel>> {
    let Ok(battle_m) = Mirror::<Battle>::get(bot, &msg.channel_id.to_string()).await else {
        return simple_send_response("Nenhuma batalha ocorrendo neste canal.", false);
    };

    let mut battle = battle_m.write().await;

    let Moment::None = battle.current_moment else {
        return simple_send_response("Você não pode usar agora.", false);
    };

    let Some(target_tag) = args.first() else {
        return simple_send_response("O argumento 'alvo' é obrigatório.", false);
    };

    let Ok(target_m) = Mirror::<Id>::get(bot, target_tag).await else {
        return simple_send_response("O alvo não existe.", false);
    };

    if *target_tag == battle.current_turn_owner_tag {
        return simple_send_response("O alvo não pode ser o próprio usuário.", false);
    }

    let user_m = Mirror::<Id>::get(bot, &battle.current_turn_owner_tag).await?;

    let mut blueprints = Vec::new();
    
    let user = user_m.read().await;
    let target = target_m.read().await;

    blueprints.extend(generate_preliminary_responses(&user, &target));
    blueprints.extend(generate_forecast_responses(&user, &target));
    blueprints.push(ResponseBlueprint::default().content(f!("{}, é a vez de sua reação.", target.name)));

    battle.current_moment = Moment::Attacking {
        action_tag: FixedString::from_static_trunc(TAG),
        user_tag: user.tag.clone(),
        target_tag: FixedString::from_str_trunc(&target_tag)
    };
    battle.action_counter += 1;

    Ok(vec![ResponseModel::send(blueprints)])
}

fn generate_preliminary_responses(user: &Id, target: &Id) -> Vec<ResponseBlueprint> {
    vec![ResponseBlueprint::default().content(f!(
        "{} está prestes a atacar {}.",
        user.name,
        target.name,
    ))]
}

fn generate_forecast_responses(user: &Id, target: &Id) -> Vec<ResponseBlueprint> {
    let evaluated_damage = target.evaluate_damage_to_receive(user.might);

    let content = match evaluated_damage * 100 / target.hp {
        0..=5 => "Parece que irá causar um dano leve.",
        6..=10 => "Parece que irá causar um dano moderado.",
        11..=20 => "Parece que irá causar um dano grave.",
        _ => "Parece que irá causar um dano **severo**.",
    };

    vec![ResponseBlueprint::default().content(content)]
}

pub fn execute(user: &Id, target: &mut Id) -> Result<Vec<ResponseBlueprint>> {
    let previous_hp = target.hp;
    target.receive_damage(user.might);

    Ok(vec![ResponseBlueprint::default().content(f!(
        "{STRIKE_EMOJI} | {} recebeu {} de dano. [{HP_SHORT}: {} → {}]",
        target.name,
        mark_thousands(user.might),
        mark_thousands(previous_hp),
        mark_thousands(target.hp),
    ))])
}