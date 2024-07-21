use crate::_prelude::*;

pub fn create<'a>(
    content: impl Into<Cow<'a, str>>,
    security_key: i64,
) -> Result<ResponseBlueprint<'a>> {
    let receive_button = CreateButton::new(f!("unclassified receive {security_key}"))
        .emoji(ReactionType::Unicode("⏭".parse()?))
        .style(ButtonStyle::Danger);

    Ok(ResponseBlueprint::default()
        .set_content(content)
        .set_components(vec![CreateActionRow::Buttons(vec![receive_button])]))
}

pub fn disable_button<'a>(message: &Message, button_column: usize) -> ResponseBlueprint<'a> {
    let buttons = message
        .components
        .first()
        .unwrap()
        .components
        .iter()
        .filter_map(|component| {
            if let ActionRowComponent::Button(button) = component {
                Some(button)
            } else {
                None
            }
        })
        .collect::<Vec<&Button>>();

    let new_buttons = buttons
            .iter()
            .take(button_column)
            .map(|original_button| al_azif_utils::serenity::copy_button(original_button))
            .chain(iter::once(
                al_azif_utils::serenity::copy_button(buttons.get(button_column)
                    .unwrap_or_else(|| unreachable!("Missing triggered button of index {button_column} on request reaction disable button")))
                    .disabled(true),
            ))
            .chain(
                buttons
                    .iter()
                    .skip(button_column + 1)
                    .map(|original_button| al_azif_utils::serenity::copy_button(original_button)),
            )
            .collect::<Vec<_>>();

    ResponseBlueprint::default()
        .set_content(message.content.clone())
        .set_components(vec![CreateActionRow::Buttons(new_buttons)])
}
