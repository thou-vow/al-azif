use crate::_prelude::*;

pub const NAME: &str = "attack";
pub const NAME_PT: &str = "atacar";

pub async fn run_prefix<'a>(bot: &impl AsBot, msg: &Message, args: &[&'a str]) -> Result<Responses<'a>> {
    let (battle_m, attacker_m, target_m) = {
        let Ok(battle_m) = Mirror::<Battle>::get(bot, msg.channel_id.to_string()).await else {
            return Ok(response::simple_send_and_delete_with_original(lang_diff!(bot,
                en: "No battle is currently happening in this channel.",
                pt: "Não há uma batalha acontecendo neste canal."
            )));
        };
        let battle = battle_m.read().await;

        let Moment::None = battle.current_moment else {
            return Ok(response::simple_send_and_delete_with_original(lang_diff!(bot,
                en: "You can't use this command right now.",
                pt: "Você não pode usar este comando agora."
            )));
        };

        let Some(target_tag) = args.first().copied() else {
            return Ok(response::simple_send_and_delete_with_original(lang_diff!(bot,
                en: "You need to specify the target.",
                pt: "Você precisa especificar o alvo."
            )));
        };

        if target_tag == battle.current_turn_owner_tag {
            return Ok(response::simple_send_and_delete_with_original(lang_diff!(bot,
                en: "You can't attack yourself.",
                pt: "Você não pode atacar a si mesmo."
            )));
        }

        let Ok(target_m) = Mirror::<Id>::get(bot, target_tag).await else {
            return Ok(response::simple_send_and_delete_with_original(lang_diff!(bot,
                en: "This target doesn't exist.",
                pt: "Este alvo não existe."
            )));
        };

        if !battle.opponents.contains_key(&FixedString::from_str_trunc(target_tag)) {
            return Ok(response::simple_send_and_delete_with_original(lang_diff!(bot,
                en: "This target is not in the battle.",
                pt: "Este alvo não está na batalha.")));
        }

        let attacker_m = Mirror::<Id>::get(bot, &battle.current_turn_owner_tag).await?;

        mem::drop(battle);

        (battle_m, attacker_m, target_m)
    };

    let blueprints = main_logic(bot, battle_m, attacker_m, target_m).await?;

    Ok(vec![Response::send(blueprints)])
}

async fn main_logic<'a>(
    bot: &impl AsBot,
    battle_m: Mirror<Battle>,
    attacker_m: Mirror<Id>,
    target_m: Mirror<Id>,
) -> Result<Blueprints<'a>> {
    let mut blueprints = Vec::new();

    let mut battle = battle_m.write().await;
    let attacker = attacker_m.read().await;
    let target = target_m.read().await;

    blueprints.push(ResponseBlueprint::new().set_content(lang_diff!(bot,
        en: f!("{STRIKE_EMOJI} | **{}** will attack **{}**.", attacker.name, target.name),
        pt: f!("{STRIKE_EMOJI} | **{}** irá atacar **{}**.", attacker.name, target.name,
    ))));
    blueprints.push(handler::generate_damage_forecast_response(bot, damage_evaluation(&attacker), target.constitution));
    blueprints.push(handler::generate_reaction_request_response(bot, &target.name)?);

    battle.current_moment = Moment::PrimaryAction {
        primary_action_tag: FixedString::from_static_trunc(NAME),
        attacker_tag:       attacker.tag.clone(),
        target_tag:         target.tag.clone(),
    };

    Ok(blueprints)
}

pub fn execute<'a>(bot: &impl AsBot, attacker: &mut Id, target: &mut Id) -> Blueprints<'a> {
    let mut blueprints = Vec::new();
    let mut damage = damage_evaluation(attacker);

    let (modified_damage, more_blueprints) = attacker.effects_when_attacking_with_primary_action(bot, damage, target);
    blueprints.extend(more_blueprints);
    damage = modified_damage;

    blueprints.push(ResponseBlueprint::new().set_content(f!(
        "{STRIKE_EMOJI} | **{}** recebeu **{}** de dano.",
        target.name,
        mark_thousands(damage),
    )));
    blueprints.extend(target.receive_damage(bot, damage));

    blueprints
}

fn damage_evaluation(attacker: &Id) -> i64 { attacker.might + attacker.evaluate_might_bonuses() }
