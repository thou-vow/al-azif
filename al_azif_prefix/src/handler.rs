use crate::_prelude::*;

pub fn execute_attack<'a>(bot: &impl AsBot, action_tag: &str, attacker: &mut Id, target: &mut Id) -> Result<Blueprints<'a>> {
    use crate::commands::*;
    let blueprints = match action_tag {
        "attack" => attack::execute(bot, attacker, target),
        "vital_trill" => vital_trill::execute(bot, attacker, target),
        _ => return Err(PrefixError::InvalidActionTag { action_tag: FixedString::from_str_trunc(action_tag) }),
    };
    Ok(blueprints)
}

pub fn generate_damage_forecast_response<'a>(bot: &impl AsBot, damage_forecast: i64, target_constitution: i64) -> ResponseBlueprint<'a> {
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

pub fn generate_reaction_request_response<'a>(bot: &impl AsBot, target_name: &str) -> Result<ResponseBlueprint<'a>> {
    let receive_button = CreateButton::new(crate::commands::receive::NAME)
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
