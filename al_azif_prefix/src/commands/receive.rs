use crate::_prelude::*;

pub const NAME: &str = "receive";
pub const NAME_PT: &str = "receber";
pub const EMOJI: &str = "⏭";

pub async fn run_prefix<'a>(bot: &impl AsBot, msg: &Message) -> Result<Responses<'a>> {
    let (battle_m, primary_action_tag, attacker_m, target_m) = {
        let Ok(battle_m) = Mirror::<Battle>::get(bot, msg.channel_id.to_string()).await else {
            return Ok(response::simple_send_and_delete_with_original(lang_diff!(bot,
                en: "No battle is currently happening in this channel.",
                pt: "Não há uma batalha acontecendo neste canal."
            )));
        };
        let battle = battle_m.read().await;

        let Moment::PrimaryAction { primary_action_tag, attacker_tag, target_tag } = &battle.current_moment else {
            return Ok(response::simple_send_and_delete_with_original(lang_diff!(bot,
                en: "You can't use this command right now.",
                pt: "Você não pode usar este comando agora."
            )));
        };
        let primary_action_tag = primary_action_tag.to_owned();

        let attacker_m = Mirror::<Id>::get(bot, attacker_tag).await?;
        let target_m = Mirror::<Id>::get(bot, target_tag).await?;

        mem::drop(battle);

        (battle_m, primary_action_tag, attacker_m, target_m)
    };

    let mut blueprints = main_logic(bot, battle_m.clone(), primary_action_tag, attacker_m, target_m).await?;

    let mut battle = battle_m.write().await;

    blueprints.extend(battle.advance(bot).await?);
    blueprints.push(battle.generate_turn_screen(bot).await?);

    Ok(vec![Response::send(blueprints)])
}

pub async fn run_component<'a>(bot: &impl AsBot, comp: &ComponentInteraction) -> Result<Responses<'a>> {
    let (battle_m, primary_action_tag, attacker_m, target_m) = {
        let Ok(battle_m) = Mirror::<Battle>::get(bot, comp.channel_id.to_string()).await else {
            return Ok(response::simple_send_ephemeral(lang_diff!(bot,
                en: "No battle is currently happening in this channel.",
                pt: "Não há uma batalha acontecendo neste canal."
            )));
        };
        let battle = battle_m.read().await;

        let Moment::PrimaryAction { primary_action_tag, attacker_tag, target_tag } = &battle.current_moment else {
            return Ok(response::simple_send_ephemeral(lang_diff!(bot,
                en: "You can't use this interaction right now.",
                pt: "Você não pode usar esta interação agora."
            )));
        };
        let primary_action_tag = primary_action_tag.to_owned();

        let attacker_m = Mirror::<Id>::get(bot, attacker_tag).await?;
        let target_m = Mirror::<Id>::get(bot, target_tag).await?;

        mem::drop(battle);

        (battle_m, primary_action_tag, attacker_m, target_m)
    };

    let mut blueprints = main_logic(bot, battle_m.clone(), primary_action_tag, attacker_m, target_m).await?;

    let mut battle = battle_m.write().await;

    blueprints.extend(battle.advance(bot).await.map_err(PrefixError::Core)?);
    blueprints.push(battle.generate_turn_screen(bot).await.map_err(PrefixError::Core)?);

    Ok(vec![Response::send_loose(blueprints)])
}

async fn main_logic<'a>(
    bot: &impl AsBot,
    battle_m: Mirror<Battle>,
    primary_action_tag: FixedString,
    attacker_m: Mirror<Id>,
    target_m: Mirror<Id>,
) -> Result<Blueprints<'a>> {
    let mut blueprints = Vec::new();

    let mut battle = battle_m.write().await;
    let mut target = target_m.write().await;
    let mut attacker = attacker_m.write().await;

    blueprints.extend(handler::execute_attack(bot, &primary_action_tag, &mut attacker, &mut target)?);

    battle.current_moment = Moment::None;

    Ok(blueprints)
}
