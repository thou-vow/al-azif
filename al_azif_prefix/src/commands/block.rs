use crate::_prelude::*;

pub const NAME: &str = "block";
pub const NAME_PT: &str = "bloquear";

pub async fn run<'a>(bot: &impl AsBot, msg: &Message) -> Result<Responses<'a>> {
    let Ok(battle_m) = Mirror::<Battle>::get(bot, msg.channel_id.to_string()).await else {
        return Ok(response::simple_send_and_delete_with_original("Nenhuma batalha ocorrendo neste canal."));
    };
    let mut battle = battle_m.write().await;

    let Moment::PrimaryAction { primary_action_tag, attacker_tag, target_tag, .. } = &battle.current_moment else {
        return Ok(response::simple_send_and_delete_with_original("Você não pode usar uma Ação Reativa agora."));
    };

    let mut blueprints = Vec::new();

    let target_m = Mirror::<Id>::get(bot, target_tag).await?;
    let mut target = target_m.write().await;

    blueprints.push(ResponseBlueprint::new().set_content(f!("🛡 | **{}** decidiu se defender.", target.name)));

    target.acquire_effect(Effect::Block);

    let attacker_m = Mirror::<Id>::get(bot, attacker_tag).await?;
    let mut attacker = attacker_m.write().await;

    blueprints.extend(execute_attack(primary_action_tag, &mut attacker, &mut target));

    mem::drop(target);
    mem::drop(attacker);

    battle.current_moment = Moment::None;
    blueprints.extend(advance(bot, &mut battle).await?);
    blueprints.push(battle.generate_turn_screen(bot).await?);

    Ok(vec![Response::send(blueprints)])
}
