use crate::prelude::*;

pub struct OriginalRollEvent<'a> {
    pub embed: &'a Embed,
    pub buttons: Vec<&'a Button>,
}
impl<'a> OriginalRollEvent<'a> {
    pub fn from_message(msg: &'a Message) -> Self {
        let original_embed = msg.embeds.first().unwrap();

        let original_buttons = msg
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

        Self {
            embed: original_embed,
            buttons: original_buttons,
        }
    }

    pub fn are_all_other_buttons_disabled(&self, button_column: usize) -> bool {
        self.buttons
            .iter()
            .take(button_column)
            .all(|button| button.disabled)
            && self
                .buttons
                .iter()
                .skip(button_column + 1)
                .all(|button| button.disabled)
    }

    pub fn outcomes(&self) -> Vec<Option<i64>> {
        self.buttons
            .iter()
            .map(|button| {
                button
                    .label
                    .as_ref()
                    .and_then(|label| label.parse::<i64>().ok())
            })
            .collect::<Vec<_>>()
    }

    pub fn after_button_press_blueprint(
        self,
        button_column: usize,
        outcome: i64,
        summary: String,
    ) -> ResponseBlueprint<'a> {
        let mut embed = CreateEmbed::new().title(self.embed.title.clone().unwrap());

        for field in self.embed.fields.iter().take(button_column) {
            embed = embed.field(field.name.clone(), field.value.clone(), field.inline);
        }

        let original_corresponding_field =
            self.embed.fields.get(button_column).unwrap_or_else(|| {
                unreachable!("Missing field of index {button_column} on roll event button press")
            });
        embed = embed.field(
            f!("{outcome}"),
            f!("{}\n{summary}", original_corresponding_field.value),
            original_corresponding_field.inline,
        );

        for field in self.embed.fields.iter().skip(button_column + 1) {
            embed = embed.field(field.name.clone(), field.value.clone(), field.inline);
        }

        let buttons = self
            .buttons
            .iter()
            .take(button_column)
            .map(|button| al_azif_utils::serenity::copy_button(button))
            .chain(iter::once(
                al_azif_utils::serenity::copy_button(
                    self.buttons.get(button_column).unwrap_or_else(|| {
                        unreachable!(
                            "Missing button of index {button_column} on roll event button press"
                        )
                    }),
                )
                .disabled(true)
                .label(outcome.to_string()),
            ))
            .chain(
                self.buttons
                    .iter()
                    .skip(button_column + 1)
                    .map(|button| al_azif_utils::serenity::copy_button(button)),
            )
            .collect::<Vec<_>>();

        ResponseBlueprint::default()
            .assign_embeds(vec![embed])
            .assign_components(vec![CreateActionRow::Buttons(buttons)])
    }
}
