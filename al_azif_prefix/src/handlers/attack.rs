use crate::_prelude::*;

pub fn generate_forecast_response<'a>(lang: impl AsRef<Lang>, damage_forecast: i64, target_constitution: i64) -> ResponseBlueprint<'a> {
    let content = match (damage_forecast * 100 / (target_constitution * 10), lang.as_ref()) {
        (.. 5, En) => fc!("{LIGHT_EMOJI} | It looks like it will cause light damage."),
        (5 .. 10, En) => fc!("{MEDIUM_EMOJI} | It looks like it will cause moderate damage."),
        (10 .. 20, En) => fc!("{HEAVY_EMOJI} | It looks like it will cause heavy damage."),
        (20 .. , En) => fc!("{SEVERE_EMOJI} | It looks like it will cause *severe* damage."),
        (.. 5, Pt) => fc!("{LIGHT_EMOJI} | Parece que irá causar um dano leve."),
        (5 .. 10, Pt) => fc!("{MEDIUM_EMOJI} | Parece que irá causar um dano moderado."),
        (10 .. 20, Pt) => fc!("{HEAVY_EMOJI} | Parece que irá causar um dano grave."),
        (20 .. , Pt) => fc!("{SEVERE_EMOJI} | Parece que irá causar um dano *severo*."),
    };

    ResponseBlueprint::new().set_content(content)
}

pub fn generate_reaction_request_response<'a>(content: impl Into<Cow<'a, str>>) -> Result<ResponseBlueprint<'a>> {
    let receive_button = CreateButton::new(crate::commands::receive::NAME)
        .emoji(ReactionType::Unicode("⏭".parse().unwrap()))
        .style(ButtonStyle::Danger);

    Ok(ResponseBlueprint::new()
        .set_content(content)
        .add_buttons(vec![receive_button])
    )
}
