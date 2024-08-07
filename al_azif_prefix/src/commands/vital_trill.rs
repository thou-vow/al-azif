use crate::_prelude::*;

pub const NAME: &str = "vital_trill";
pub const NAME_PT: &str = "trinado_vital";

pub async fn run_prefix(bot: &impl AsBot, msg: &Message, args: VecDeque<&str>) -> Result<Responses> {
    let setting = Setting::new(bot, args)
        .fetch_battle(msg.channel_id.to_string())
        .await?
        .require_primary_moment()
        .await?
        .fetch_user()
        .await?
        .fetch_targets([lang_diff!(bot,
            en: "You need to specify the target.",
            pt: "Você precisa especificar o alvo."
        )])
        .await?
        .unallow_self_target::<0>(lang_diff!(bot,
            en: "You can't attack yourself.",
            pt: "Você não pode atacar a si mesmo."
        ))?;

    let mut blueprints = Vec::new();

    let mut battle = setting.get_battle_mirror().write().await;
    let user = setting.get_user_mirror().read().await;
    let target = setting.get_target_ms()[0].read().await;

    blueprints.push(ResponseBlueprint::new().set_content(lang_diff!(bot,
        en: f!("{STRIKE_EMOJI} | **{}** will attack **{}**.", user.name, target.name),
        pt: f!("{STRIKE_EMOJI} | **{}** irá atacar **{}**.", user.name, target.name,
    ))));

    battle.current_moment = Moment::Reactive(ReactiveMoment {
        primary_moment_owner_tag: user.tag.clone(),
        primary_action_tag:       FixedString::from_static_trunc(NAME),
        target_tags:              vec![target.tag.clone()],
        target_index:             0,
    });

    blueprints.push(handler::generate_damage_forecast_response(bot, damage_evaluation(&user), target.constitution));
    blueprints.push(handler::generate_reaction_request_response(bot, &target.name)?);

    Ok(vec![Response::send(blueprints)])
}

pub fn execute(bot: &impl AsBot, emitter: &mut Id, target: &mut Id) -> Blueprints {
    let mut blueprints = Vec::new();
    let mut damage = damage_evaluation(emitter);

    let (modified_damage, more_blueprints) = effect::at_primary_action_attack(bot, emitter, target, damage);
    blueprints.extend(more_blueprints);
    damage = modified_damage;

    blueprints.push(ResponseBlueprint::new().set_content(f!(
        "{STRIKE_EMOJI} | **{}** recebeu **{}** de dano.",
        target.name,
        mark_thousands(damage),
    )));
    blueprints.extend(target.receive_damage(bot, damage));
    blueprints.extend(target.acquire_effect(bot, BleedEffect { damage_over_turn: 10, turn_duration: 2 }));

    blueprints
}

fn damage_evaluation(emitter: &Id) -> i64 { emitter.evaluate_total_might() }
