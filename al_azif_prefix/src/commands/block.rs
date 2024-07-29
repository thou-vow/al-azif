use crate::_prelude::*;

pub const NAME: &str = "block";
pub const NAME_PT: &str = "bloquear";

pub async fn run_prefix<'a>(bot: &impl AsBot, msg: &Message) -> Result<Responses<'a>> {
    let (battle_m, primary_action_tag, attacker_m, target_m) = {
        let battle_m = validate::battle_exists_in_channel!(bot, msg.channel_id.to_string());
        let battle = battle_m.read().await;

        let (primary_action_tag, attacker_tag, target_tag) = validate::moment_primary_in_battle!(bot, &battle);

        let primary_action_tag = primary_action_tag.to_owned();
        let attacker_m = Mirror::<Id>::get(bot, attacker_tag).await?;
        let target_m = Mirror::<Id>::get(bot, target_tag).await?;

        mem::drop(battle);

        (battle_m, primary_action_tag, attacker_m, target_m)
    };
    let mut battle = battle_m.write().await;
    let mut target = target_m.write().await;
    let mut attacker = attacker_m.write().await;

    let mut blueprints = Vec::new();
    blueprints.push(ResponseBlueprint::new().set_content(lang_diff!(bot,
        en: f!("🛡 | **{}** decided to defend.", target.name),
        pt: f!("🛡 | **{}** decidiu se defender.", target.name)
    )));
    blueprints.extend(target.acquire_effect(Effect::Block));
    blueprints.extend(handler::execute_attack(&primary_action_tag, &mut attacker, &mut target)?);

    mem::drop((target, attacker));

    battle.current_moment = Moment::None;
    blueprints.extend(advance(bot, &mut battle).await?);
    blueprints.push(battle.generate_turn_screen(bot).await?);

    Ok(vec![Response::send(blueprints)])
}
