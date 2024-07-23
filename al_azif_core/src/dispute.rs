use crate::_prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Dispute {
    pub tag: FixedString,
    pub button_lead_args: Vec<FixedString>,
    pub title: FixedString,
    pub tests: Vec<Test>,
}
impl Dispute {
    pub fn new(tag: impl AsRef<str>, button_lead_args: Vec<impl AsRef<str>>) -> Self {
        Self {
            tag: FixedString::from_str_trunc(tag.as_ref()),
            title: FixedString::default(),
            button_lead_args: button_lead_args.iter().map(|arg| FixedString::from_str_trunc(arg.as_ref())).collect(),
            tests: Vec::new(),
        }
    }
    pub fn add_test(mut self, test: Test) -> Self {
        self.tests.push(test);
        self
    }
    pub fn set_title(mut self, title: impl AsRef<str>) -> Self {
        self.title = FixedString::from_str_trunc(title.as_ref());
        self
    }
}
impl Dispute {
    pub async fn evaluate_test(&mut self, bot: &impl AsBot, index: usize) -> Result<()> {
        if let Some(test) = self.tests.get_mut(index) {
            test.get_roll_expression(bot).await?;
        }

        Ok(())
    }
}
impl Dispute {
    pub fn are_all_evaluated(&self) -> bool {
        self.tests
            .iter()
            .all(|test| test.evaluation.is_some())
    }
    pub async fn generate_screen<'a>(
        &self,
        bot: &impl AsBot,
    ) -> Result<ResponseBlueprint<'a>> {
        let mut new_buttons = Vec::new();
        let mut new_description = String::new();

        let joined_lead_args = self.button_lead_args.join(" ");

        for (i, test) in self.tests.iter().enumerate() {
                let button_style = match i % 3 {
                    0 => ButtonStyle::Primary,
                    1 => ButtonStyle::Danger,
                    2 => ButtonStyle::Success,
                    _ => unreachable!(),
                };

                let (new_section, new_button) = test
                    .new_section_and_button(
                        bot,
                        f!("{joined_lead_args} {} {i}", self.tag),
                        button_style,
                    )
                    .await?;

                new_description.push_str(&new_section);
                new_buttons.push(new_button);
        }

        let new_embed = CreateEmbed::new()
            .author(CreateEmbedAuthor::new(self.title.clone())
                .icon_url("https://media.discordapp.net/attachments/1161050052538675200/1264433422344917086/dice.png?ex=669ddae3&is=669c8963&hm=67ac368580845b5828f46f56bc0337365d616712e093620e9b68a9ced24e3e63&=&format=webp&quality=lossless&width=412&height=473")
            )
            .description(new_description);

        Ok(ResponseBlueprint::default()
            .add_embed(new_embed)
            .add_buttons(new_buttons))
    }
}

#[derive(Deserialize, Serialize)]
pub struct Test {
    pub test_kind: TestKind,
    pub dice_bonus: i64,
    pub side_bonus: i64,
    pub advantage_bonus: i64,
    pub evaluation: Option<(i64, RollSummary)>,
    pub button_style: ButtonStyle,
}
impl Test {
    pub fn new(test_kind: TestKind) -> Self {
        Self {
            test_kind,
            dice_bonus: 0,
            side_bonus: 0,
            advantage_bonus: 0,
            evaluation: None,
            button_style: ButtonStyle::Primary,
        }
    }
    pub fn set_dice_bonus(mut self, dice_bonus: i64) -> Self {
        self.dice_bonus = dice_bonus;
        self
    }
    pub fn set_side_bonus(mut self, side_bonus: i64) -> Self {
        self.side_bonus = side_bonus;
        self
    }
    pub fn set_advantage_bonus(mut self, advantage_bonus: i64) -> Self {
        self.advantage_bonus = advantage_bonus;
        self
    }
    pub fn set_button_style(mut self, button_style: ButtonStyle) -> Self {
        self.button_style = button_style;
        self
    }
}
impl Test {
    pub async fn get_roll_expression<'a>(&self, bot: &impl AsBot, id_tag: impl AsRef<str>) -> Result<RollExpression> {
        let id_m = Mirror::<Id>::get(bot, id_tag.as_ref()).await?;
        let id = id_m.read().await;

        let mut roll_expression = match self.test_kind {
            TestKind::AccuracyTest => RollExpression::new(1, id.dexterity, 0),
            TestKind::EvasionTest => RollExpression::new(1, id.dexterity, 0),
        };
        roll_expression.dices += self.dice_bonus;
        roll_expression.sides += self.side_bonus;
        roll_expression.advantage += self.advantage_bonus;

        Ok(roll_expression)
    }
    pub async fn new_section_and_button<'a>(
        &self,
        bot: &impl AsBot,
        id_tag: impl AsRef<str>,
        button_custom_id: impl Into<Cow<'a, str>>,
    ) -> Result<(String, CreateButton<'a>)> {
        let id_tag = id_tag.as_ref();

        let mut new_section = f!("### {}\n> ", self.test_kind);
        let mut new_button = CreateButton::new(button_custom_id).style(self.button_style);

        let id_m = Mirror::<Id>::get(bot, id_tag).await?;
        let id = id_m.read().await;

        if let Some(emoji) = &id.emoji {
            new_section += emoji;
            new_button = new_button.emoji(ReactionType::Unicode(emoji.parse()?));
        } else {
            match self.button_style {
                ButtonStyle::Primary => {
                    new_section += "🔵";
                    new_button = new_button.emoji(ReactionType::Unicode("🔵".parse()?));
                }
                ButtonStyle::Danger => {
                    new_section += "🔴";
                    new_button = new_button.emoji(ReactionType::Unicode("🔴".parse()?));
                }
                ButtonStyle::Success => {
                    new_section += "🟢";
                    new_button = new_button.emoji(ReactionType::Unicode("🟢".parse()?));
                }
                _ => unreachable!(),
            };
        }
        new_section += &f!("**{}**", id.name);

        mem::drop(id);

        let roll_expression = self.get_roll_expression(bot, id_tag).await?;

        new_section += &f!(
            " `{}`\n> {}d{} 🎉 {}\n",
            id_tag,
            roll_expression.dices,
            roll_expression.sides,
            roll_expression.advantage
        );

        if let Some((outcome, roll_summary)) = &self.evaluation {
            new_section += &f!(
                "> # {}\n{}\n",
                outcome,
                roll_summary.ansi_code_block_in_block_quote()
            );
        } else {
            new_section += "> ```Aguardando interação...```\n";
        }

        if self.evaluation.is_some() {
            new_button = new_button.disabled(true);
        }

        Ok((new_section, new_button))
    }
}

#[derive(Deserialize, Serialize)]
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
