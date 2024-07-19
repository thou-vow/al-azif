use crate::prelude::*;

pub fn create<'a>(
    content: impl Into<Cow<'a, str>>,
    security_key: i64,
) -> Result<ResponseBlueprint<'a>> {
    let receive_button = CreateButton::new(f!("unclassified receive {security_key}"))
        .emoji(ReactionType::Unicode("⏭".parse()?))
        .style(ButtonStyle::Danger);

    Ok(ResponseBlueprint::default()
        .assign_content(content)
        .assign_components(vec![CreateActionRow::Buttons(vec![receive_button])]))
}

pub fn disable_button<'a>(message: &Message, button_column: usize) -> ResponseBlueprint<'a> {
    let original_buttons = message
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

    let buttons = original_buttons
            .iter()
            .take(button_column)
            .map(|original_button| al_azif_utils::serenity::copy_button(original_button))
            .chain(iter::once(
                al_azif_utils::serenity::copy_button(original_buttons.get(button_column)
                    .unwrap_or_else(|| unreachable!("Missing triggered button of index {button_column} on request reaction disable button")))
                    .disabled(true),
            ))
            .chain(
                original_buttons
                    .iter()
                    .skip(button_column + 1)
                    .map(|original_button| al_azif_utils::serenity::copy_button(original_button)),
            )
            .collect::<Vec<_>>();

    ResponseBlueprint::default()
        .assign_content(message.content.clone())
        .assign_components(vec![CreateActionRow::Buttons(buttons)])
}
