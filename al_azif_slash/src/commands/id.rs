use crate::_prelude::*;

pub const NAME: &str = "id";
pub const DESCRIPTION: &str = "About Id";
pub const NAME_PT: &str = "id";
pub const DESCRIPTION_PT: &str = "Sobre Id";

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new(NAME)
        .description(DESCRIPTION)
        .name_localized("pt-BR", NAME_PT)
        .description_localized("pt-BR", DESCRIPTION_PT)
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, distribute::NAME, distribute::DESCRIPTION)
                .name_localized("pt-BR", distribute::NAME_PT)
                .description_localized("pt-BR", distribute::DESCRIPTION_PT)
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "id", "The Id to distribute")
                        .description_localized("pt-BR", "O Id para distribuir")
                        .required(true),
                ),
        )
}

pub mod distribute {
    use super::*;

    pub const NAME: &str = "distribute";
    pub const DESCRIPTION: &str = "Distribute points to the attributes";
    pub const NAME_PT: &str = "distribuir";
    pub const DESCRIPTION_PT: &str = "Distribuir pontos para os atributos";

    pub async fn run_slash(bot: &impl AsBot, id_tag: &str) -> Result<Responses> {
        let Ok(id_m) = Mirror::<Id>::get(bot, id_tag).await else {
            return Err(SlashError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(bot,
                en: f!("Id `{id_tag}` not found."),
                pt: f!("Id `{id_tag}` não encontrado.")
            ))]));
        };

        let new_embed = generate_embed(bot, &*id_m.read().await)?;
        let new_components = generate_attribute_components(id_tag)?;

        Ok(vec![Response::send(vec![ResponseBlueprint::new().add_embed(new_embed).set_components(new_components)])])
    }

    pub mod goto_attributes {
        use super::*;

        pub async fn run_component(bot: &impl AsBot, id_tag: &str) -> Result<Responses> {
            let id_m = Mirror::<Id>::get(bot, id_tag).await?;

            let new_embed = generate_embed(bot, &*id_m.read().await)?;
            let new_components = generate_attribute_components(id_tag)?;

            Ok(vec![Response::update(ResponseBlueprint::new().set_embeds(vec![new_embed]).set_components(new_components))])
        }
    }

    pub mod goto_incrementors {
        use super::*;

        pub async fn run_component(bot: &impl AsBot, id_tag: &str, attribute_str: &str) -> Result<Responses> {
            let id_m = Mirror::<Id>::get(bot, id_tag).await?;

            let new_embed = generate_embed(bot, &*id_m.read().await)?;
            let new_components = generate_incrementor_components(id_tag, attribute_str)?;

            Ok(vec![Response::update(ResponseBlueprint::new().set_embeds(vec![new_embed]).set_components(new_components))])
        }
    }

    pub mod invest_in {
        use super::*;

        pub async fn run_component(bot: &impl AsBot, id_tag: &str, attribute_str: &str, selected_value: i64) -> Result<Responses> {
            let id_m = Mirror::<Id>::get(bot, id_tag).await?;

            let mut id = id_m.write().await;
            invest(&mut id, attribute_str, selected_value)?;
            let id = id.downgrade()?;
            let new_embed = generate_embed(bot, &id)?;

            let new_components = generate_incrementor_components(id_tag, attribute_str)?;

            Ok(vec![Response::update(ResponseBlueprint::new().set_embeds(vec![new_embed]).set_components(new_components))])
        }
    }

    fn invest(id: &mut Id, attribute_str: &str, selected_value: i64) -> Result<()> {
        let spent = min(selected_value, id.points_to_distribute);

        match attribute_str {
            "con" => id.constitution += spent,
            "spr" => id.spirit += spent,
            "mgt" => id.might += spent,
            "mov" => id.movement += spent,
            "dex" => id.dexterity += spent,
            "cog" => id.cognition += spent,
            "cha" => id.charisma += spent,
            _ => return Err(SlashError::InvalidInvestedAttribute { attribute_str: FixedString::from_str_trunc(attribute_str) }),
        }

        id.points_to_distribute -= spent;

        Ok(())
    }

    fn generate_embed(bot: &impl AsBot, id: &Id) -> Result<CreateEmbed<'static>> {
        let mut attributes_field = String::new();
        attributes_field += &f!("{CON_EMOJI} `{}` {}\n", lang_diff!(bot, en: CON_SHORT, pt: CON_SHORT_PT), mark_thousands(id.constitution));
        attributes_field += &f!("{SPR_EMOJI} `{}` {}\n", lang_diff!(bot, en: SPR_SHORT, pt: SPR_SHORT_PT), mark_thousands(id.spirit));
        attributes_field += &f!("{MGT_EMOJI} `{}` {}\n", lang_diff!(bot, en: MGT_SHORT, pt: MGT_SHORT_PT), mark_thousands(id.might));
        attributes_field += &f!("{MOV_EMOJI} `{}` {}\n", lang_diff!(bot, en: MOV_SHORT, pt: MOV_SHORT_PT), mark_thousands(id.movement));
        attributes_field += &f!("{DEX_EMOJI} `{}` {}\n", lang_diff!(bot, en: DEX_SHORT, pt: DEX_SHORT_PT), mark_thousands(id.dexterity));
        attributes_field += &f!("{COG_EMOJI} `{}` {}\n", lang_diff!(bot, en: COG_SHORT, pt: COG_SHORT_PT), mark_thousands(id.cognition));
        attributes_field += &f!("{CHA_EMOJI} `{}` {}\n", lang_diff!(bot, en: CHA_SHORT, pt: CHA_SHORT_PT), mark_thousands(id.charisma));

        let new_embed = CreateEmbed::new()
            .title(f!("{} 🎊 [{}]", id.name, mark_thousands(id.lvl)))
            .color(id.color.unwrap_or(0x36393e))
            .description(f!("{} / {}", mark_thousands(id.xp), mark_thousands(xp_to_next_level(id.lvl))))
            .field("", attributes_field, false)
            .footer(CreateEmbedFooter::new(lang_diff!(bot,
                en: f!("{} points to distribute.", mark_thousands(id.points_to_distribute)),
                pt: f!("{} pontos para distribuir.", mark_thousands(id.points_to_distribute)
            ))));

        Ok(new_embed)
    }

    fn generate_attribute_components(id_tag: &str) -> Result<Vec<CreateActionRow<'static>>> {
        let row_1 = CreateActionRow::Buttons(vec![
            CreateButton::new(f!("#slash id distribute goto_incrementors {id_tag} con")).emoji(ReactionType::Unicode(
                CON_EMOJI.parse().map_err(|_| SlashError::FailedToConvertStringToReactionType { str: CON_EMOJI })?,
            )),
            CreateButton::new(f!("#slash id distribute goto_incrementors {id_tag} spr")).emoji(ReactionType::Unicode(
                SPR_EMOJI.parse().map_err(|_| SlashError::FailedToConvertStringToReactionType { str: SPR_EMOJI })?,
            )),
            CreateButton::new(f!("#slash id distribute goto_incrementors {id_tag} mgt")).emoji(ReactionType::Unicode(
                MGT_EMOJI.parse().map_err(|_| SlashError::FailedToConvertStringToReactionType { str: MGT_EMOJI })?,
            )),
            CreateButton::new(f!("#slash id distribute goto_incrementors {id_tag} mov")).emoji(ReactionType::Unicode(
                MOV_EMOJI.parse().map_err(|_| SlashError::FailedToConvertStringToReactionType { str: MOV_EMOJI })?,
            )),
        ]);

        let row_2 = CreateActionRow::Buttons(vec![
            CreateButton::new(f!("#slash id distribute goto_incrementors {id_tag} dex")).emoji(ReactionType::Unicode(
                DEX_EMOJI.parse().map_err(|_| SlashError::FailedToConvertStringToReactionType { str: DEX_EMOJI })?,
            )),
            CreateButton::new(f!("#slash id distribute goto_incrementors {id_tag} cog")).emoji(ReactionType::Unicode(
                COG_EMOJI.parse().map_err(|_| SlashError::FailedToConvertStringToReactionType { str: COG_EMOJI })?,
            )),
            CreateButton::new(f!("#slash id distribute goto_incrementors {id_tag} cha")).emoji(ReactionType::Unicode(
                CHA_EMOJI.parse().map_err(|_| SlashError::FailedToConvertStringToReactionType { str: CHA_EMOJI })?,
            )),
        ]);

        Ok(vec![row_1, row_2])
    }

    fn generate_incrementor_components(id_tag: &str, attribute_str: &str) -> Result<Vec<CreateActionRow<'static>>> {
        let custom_button_id_lead = f!("#slash id distribute invest_in {id_tag} {attribute_str} ");

        let new_button_1 = CreateButton::new(f!("{custom_button_id_lead}1")).label("+1");
        let new_button_4 = CreateButton::new(f!("{custom_button_id_lead}4")).label("+4");
        let new_button_10 = CreateButton::new(f!("{custom_button_id_lead}10")).label("+10");
        let new_button_100000 = CreateButton::new(f!("{custom_button_id_lead}100000")).label("+100000");

        let new_button_go_back = CreateButton::new(f!("#slash id distribute goto_attributes {id_tag}")).emoji(ReactionType::Unicode(
            GO_BACK_EMOJI.parse().map_err(|_| SlashError::FailedToConvertStringToReactionType { str: GO_BACK_EMOJI })?,
        ));

        let row_1 = CreateActionRow::Buttons(vec![new_button_1, new_button_4, new_button_10, new_button_100000, new_button_go_back]);

        Ok(vec![row_1])
    }
}
