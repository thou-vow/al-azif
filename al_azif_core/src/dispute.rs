use crate::_prelude::*;

pub enum TestKind {
    AccuracyTest,
    EvasionTest,
}
impl Display for TestKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TestKind::AccuracyTest => write!(f, "Teste de Precisão"),
            TestKind::EvasionTest => write!(f, "Teste de Evasão"),
        }
    }
}

pub struct CreateDispute<'a> {
    pub custom_id_part: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub first_member: Option<CreateDisputeMember<'a>>,
    pub second_member: Option<CreateDisputeMember<'a>>,
    pub third_member: Option<CreateDisputeMember<'a>>,
    pub security_key: i64,
}

impl<'a> CreateDispute<'a> {
    pub fn new(
        custom_id_part: impl Into<Cow<'a, str>>,
        title: impl Into<Cow<'a, str>>,
        security_key: i64,
    ) -> Self {
        Self {
            custom_id_part: custom_id_part.into(),
            title: title.into(),
            first_member: None,
            second_member: None,
            third_member: None,
            security_key,
        }
    }
    pub fn add_member(mut self, member: CreateDisputeMember<'a>) -> Self {
        if self.first_member.is_none() {
            self.first_member = Some(member);
        } else if self.second_member.is_none() {
            self.second_member = Some(member);
        } else if self.third_member.is_none() {
            self.third_member = Some(member);
        }

        self
    }
}
impl<'a> CreateDispute<'a> {
    pub fn create(self) -> ResponseBlueprint<'a> {
        let mut new_buttons = Vec::new();
        let mut new_description = String::new();

        if let Some(first_member) = self.first_member {
            new_description += &f!(
                "### {}\n> 🟦 **{}** `{}`\n> {}d{} 🎉 {}\n> ```Aguardando interação...```",
                first_member.test_kind,
                first_member.id_name,
                first_member.id_tag,
                first_member.dices,
                first_member.sides,
                first_member.advantage
            );

            new_buttons.push(
                CreateButton::new(f!("{} {} 0", self.custom_id_part, self.security_key))
                    .emoji(ReactionType::Unicode("🎲".parse().unwrap())),
            )
        }

        if let Some(second_member) = self.second_member {
            new_description += &f!(
                "\n### {}\n> 🟥 **{}** `{}`\n> {}d{} 🎉 {}\n> ```Aguardando interação...```",
                second_member.test_kind,
                second_member.id_name,
                second_member.id_tag,
                second_member.dices,
                second_member.sides,
                second_member.advantage
            );

            new_buttons.push(
                CreateButton::new(f!("{} {} 1", self.custom_id_part, self.security_key))
                    .emoji(ReactionType::Unicode("🎲".parse().unwrap()))
                    .style(ButtonStyle::Danger),
            )
        }

        if let Some(third_member) = self.third_member {
            new_description += &f!(
                "\n### {}\n> 🟩 **{}** `{}`\n> {}d{} 🎉 {}\n> ```Aguardando interação...```",
                third_member.test_kind,
                third_member.id_name,
                third_member.id_tag,
                third_member.dices,
                third_member.sides,
                third_member.advantage
            );

            new_buttons.push(
                CreateButton::new(f!("{} {} 2", self.custom_id_part, self.security_key))
                    .emoji(ReactionType::Unicode("🎲".parse().unwrap()))
                    .style(ButtonStyle::Success),
            )
        }

        let new_embed = CreateEmbed::new()
            .author(CreateEmbedAuthor::new(self.title)
                .icon_url("https://media.discordapp.net/attachments/1161050052538675200/1264433422344917086/dice.png?ex=669ddae3&is=669c8963&hm=67ac368580845b5828f46f56bc0337365d616712e093620e9b68a9ced24e3e63&=&format=webp&quality=lossless&width=412&height=473")
            )
            .description(new_description);

        ResponseBlueprint::default()
            .add_embed(new_embed)
            .set_components(vec![CreateActionRow::Buttons(new_buttons)])
    }
}

pub struct CreateDisputeMember<'a> {
    pub test_kind: TestKind,
    pub id_tag: Cow<'a, str>,
    pub id_name: Cow<'a, str>,
    pub dices: i64,
    pub sides: i64,
    pub advantage: i64,
}
impl<'a> CreateDisputeMember<'a> {
    pub fn new(
        test_kind: TestKind,
        id_tag: impl Into<Cow<'a, str>>,
        id_name: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            test_kind,
            id_tag: id_tag.into(),
            id_name: id_name.into(),
            dices: 1,
            sides: 20,
            advantage: 0,
        }
    }
    pub fn set_dices(mut self, dices: i64) -> Self {
        self.dices = dices;
        self
    }
    pub fn set_sides(mut self, sides: i64) -> Self {
        self.sides = sides;
        self
    }
    pub fn set_advantage(mut self, advantage: i64) -> Self {
        self.advantage = advantage;
        self
    }
}

pub struct Dispute<'a> {
    pub embed: &'a Embed,
    pub buttons: Vec<&'a Button>,
}
impl<'a> Dispute<'a> {
    pub fn from_message(msg: &'a Message) -> Self {
        let embed = msg.embeds.first().unwrap();

        let buttons = msg
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

        Self { embed, buttons }
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
    pub fn create_response_after_button_press(
        &self,
        button_column: usize,
        outcome: i64,
        summary: RollSummary,
    ) -> ResponseBlueprint<'a> {
        let embed_author = self.embed.author.as_ref().unwrap();

        let mut description_sections: Vec<_> = self
            .embed
            .description
            .as_ref()
            .unwrap()
            .split("\n###")
            .map(|section| section.to_string())
            .collect();

        let mut corresponding_section_lines = description_sections
            .get_mut(button_column)
            .unwrap()
            .split('\n')
            .map(|line| line.to_string())
            .collect::<Vec<_>>();

        let new_line = f!(
            "> # {outcome}\n{}",
            summary.ansi_code_block_in_block_quote()
        );

        *corresponding_section_lines.last_mut().unwrap() = new_line;
        description_sections[button_column] = corresponding_section_lines.join("\n");

        let new_description = description_sections.join("\n###");

        let new_embed = CreateEmbed::new()
            .author(
                CreateEmbedAuthor::new(embed_author.name.clone())
                    .icon_url(embed_author.icon_url.as_ref().unwrap().clone()),
            )
            .description(new_description);

        let new_buttons = self
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
            .set_embeds(vec![new_embed])
            .set_components(vec![CreateActionRow::Buttons(new_buttons)])
    }
}
