use crate::prelude::*;

pub const ALIASES: [&str; 2] = ["atacar", "attack"];

pub async fn run_command(bot: &impl AsBot, msg: &Message, args: &[&str]) -> ResponseResult {
    let Ok(battle_m) = Mirror::<Battle>::get(bot, &msg.channel_id.to_string()).await else {
        return simple_response("Nenhuma batalha ocorrendo neste canal.", ResponseMode::Delete);
    };

    let Some(target_tag) = args.first() else {
        return simple_response("O argumento 'alvo' é obrigatório.", ResponseMode::Delete);
    };
    let Ok(target_m) = Mirror::<Id>::get(bot, target_tag).await else {
        return simple_response("O alvo não existe.", ResponseMode::Delete);
    };

    let mut battle = battle_m.write().await;
    if *target_tag == battle.state.current_turn_owner_tag.as_ref() {
        return simple_response("O alvo não pode ser você.", ResponseMode::Delete);
    }

    let user_m = Mirror::<Id>::get(bot, &battle.state.current_turn_owner_tag).await?;

    let mut blueprints = Vec::new();

    blueprints.push(attack(&*user_m.read().await, &mut *target_m.write().await));
    blueprints.extend(battle.advance(bot).await?);
    blueprints.push(battle.generate_turn_screen(bot).await?);

    Ok((blueprints, ResponseMode::Normal))
}

fn attack(user: &Id, target: &mut Id) -> ResponseBlueprint {
    let damage = max(user.attributes.might, 0);
    let previous_hp = target.hp;

    target.take_damage(damage);

    ResponseBlueprint {
        content: Some(f!("{STRIKE_EMOJI} | {} recebeu {} de dano. [{HP_SHORT}: {} → {}]",
            target.ego.name,
            mark_thousands(damage),
            mark_thousands(previous_hp),
            mark_thousands(target.hp)
        ).into()),
        ..Default::default()
    }
}