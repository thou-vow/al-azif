use crate::_prelude::*;

pub fn execute_primary_action(bot: &impl AsBot, action_tag: &str, emitter: &mut Id, target: &mut Id) -> Result<Blueprints> {
    use crate::commands::*;
    let mut blueprints = match action_tag {
        "attack" => attack::execute(bot, emitter, target),
        "vital_trill" => vital_trill::execute(bot, emitter, target),
        _ => return Err(PrefixError::InvalidActionTag { action_tag: FixedString::from_str_trunc(action_tag) }),
    };

    blueprints.extend(effect::on_action_end(bot, emitter));
    blueprints.extend(effect::on_action_end(bot, target));

    Ok(blueprints)
}

pub fn generate_damage_forecast_response(bot: &impl AsBot, damage_forecast: i64, target_constitution: i64) -> ResponseBlueprint {
    let content = match damage_forecast * 100 / (target_constitution * 10) {
        .. 5 => lang_diff!(bot, en: fc!("{LIGHT_EMOJI} | It looks like it will cause light damage."),
                                pt: fc!("{LIGHT_EMOJI} | Parece que irá causar um dano leve.")),
        5 .. 10 => lang_diff!(bot, en: fc!("{MODERATE_EMOJI} | It looks like it will cause  moderate damage."),
                                   pt: fc!("{MODERATE_EMOJI} | Parece que irá causar um dano moderado.")),
        10 .. 20 => lang_diff!(bot, en: fc!("{HEAVY_EMOJI} | It looks like it will cause heavy damage."),
                                    pt: fc!("{HEAVY_EMOJI} | Parece que irá causar um dano grave.")),
        20 .. => lang_diff!(bot, en: fc!("{SEVERE_EMOJI} | It looks like it will cause *severe* damage."),
                                 pt: fc!("{SEVERE_EMOJI} | Parece que irá causar um dano *severo*.")),
    };

    ResponseBlueprint::new().set_content(content)
}

pub fn generate_reaction_request_response(bot: &impl AsBot, target_name: &str) -> Result<ResponseBlueprint> {
    let receive_button = CreateButton::new(fc!("{} .", crate::commands::receive::TAG))
        .emoji(ReactionType::Unicode(
            crate::commands::receive::EMOJI
                .parse()
                .map_err(|_| PrefixError::FailedToConvertStringToReactionType { str: crate::commands::receive::EMOJI })?,
        ))
        .style(ButtonStyle::Danger);

    Ok(ResponseBlueprint::new()
        .set_content(lang_diff!(bot,
            en: f!("⏳ | **{target_name}**, is your time to react."),
            pt: f!("⏳ | **{target_name}**, é a vez de sua reação.")))
        .add_buttons(vec![receive_button]))
}
