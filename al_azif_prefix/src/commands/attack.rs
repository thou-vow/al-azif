use crate::_prelude::*;

pub const NAME: &str = "attack";
pub const NAME_PT: &str = "atacar";

pub async fn run_prefix<'a>(bot: &impl AsBot, msg: &Message, args: &[&'a str]) -> Result<Responses<'a>> {
    let (battle_m, attacker_m, target_m) = {
        let battle_m = validate::battle_exists_in_channel!(bot, msg.channel_id.to_string());
        let battle = battle_m.read().await;

        validate::moment_none_in_battle!(bot, battle);

        let target_tag = validate::target_arg!(bot, args.first().copied());
        let target_m = validate::target_exists!(bot, target_tag);
        validate::target_is_in_the_battle!(bot, battle, target_tag);

        let attacker_m = Mirror::<Id>::get(bot, &battle.current_turn_owner_tag).await?;

        mem::drop(battle);

        (battle_m, attacker_m, target_m)
    };

    let blueprints = run(bot, battle_m, attacker_m, target_m).await?;

    Ok(vec![Response::send(blueprints)])
}

pub async fn run<'a>(
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
    blueprints.push(common::attack::generate_forecast_response(
        bot,
        damage_evaluation(&attacker),
        target.constitution,
    ));
    blueprints.push(common::attack::generate_reaction_request_response(&target.name)?);

    battle.current_moment = Moment::PrimaryAction {
        primary_action_tag: FixedString::from_static_trunc(NAME),
        attacker_tag:       attacker.tag.clone(),
        target_tag:         target.tag.clone(),
    };

    Ok(blueprints)
}

pub fn execute<'a>(attacker: &mut Id, target: &mut Id) -> Blueprints<'a> {
    target.take_damage(damage_evaluation(attacker))
}

fn damage_evaluation(attacker: &Id) -> i64 { attacker.might + attacker.evaluate_might_bonuses() }
