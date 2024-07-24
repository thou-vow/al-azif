use crate::_prelude::*;

pub const NAME: &str = "id";
pub const DESCRIPTION: &str = "About Id";
pub const NAME_LOCALIZED: &str = "id";
pub const DESCRIPTION_LOCALIZED: &str = "Sobre Id";

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new(NAME)
        .description(DESCRIPTION)
        .name_localized("pt-BR", NAME_LOCALIZED)
        .description_localized("pt-BR", DESCRIPTION_LOCALIZED)
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, distribute::NAME, distribute::DESCRIPTION)
                .name_localized("pt-BR", distribute::NAME_LOCALIZED)
                .description_localized("pt-BR", distribute::DESCRIPTION_LOCALIZED)
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "id", "The Id to distribute")
                        .description_localized("pt-BR", "O Id para distribuir")
                        .required(true),
                ),
        )
}

pub async fn run_component<'a>(
    bot: &impl AsBot,
    _comp: &ComponentInteraction,
    args: &[&str],
) -> Result<Responses<'a>> {
    match args[0] {
        "distribute" => distribute::run_component(bot, &args[1 ..]).await,
        invalid => unreachable!("Invalid component branch for 'id' components: {invalid}"),
    }
}

pub mod distribute {
    use super::*;

    pub const NAME: &str = "distribute";
    pub const DESCRIPTION: &str = "Distribute points to the attributes";
    pub const NAME_LOCALIZED: &str = "distribuir";
    pub const DESCRIPTION_LOCALIZED: &str = "Distribuir pontos para os atributos";

    pub async fn run<'a>(bot: &impl AsBot, args: &[ResolvedOption<'_>]) -> Result<Responses<'a>> {
        let ResolvedValue::String(id_tag) = args[0].value else {
            unreachable!("The 'id' argument of the 'id distribute' command must be a string!");
        };
        let Ok(id_m) = Mirror::<Id>::get(bot, id_tag).await else {
            return response::simple_send_and_delete("Informe um Id válido.");
        };

        let id = id_m.read().await;
        let new_embed = generate_embed(&id).await?;
        let new_components = generate_attribute_components(&id).await?;

        Ok(vec![Response::send(vec![ResponseBlueprint::new()
            .add_embed(new_embed)
            .set_components(new_components)])])
    }

    pub async fn run_component<'a>(bot: &impl AsBot, args: &[&str]) -> Result<Responses<'a>> {
        let id_m = Mirror::<Id>::get(bot, args[0]).await?;

        let components;
        match args[1] {
            "invest_in" => {
                invest(&mut *id_m.write().await, args[2], args[3].parse()?).await?;
                components = generate_incrementor_components(args[0], args[2]).await?;
            },
            "goto_incrementors" => {
                components = generate_incrementor_components(args[0], args[2]).await?;
            },
            "goto_attributes" => {
                components = generate_attribute_components(&*id_m.read().await).await?;
            },
            invalid => unreachable!("Invalid operation on 'id distribute' component interaction: {invalid}"),
        }

        let embed = generate_embed(&*id_m.read().await).await?;

        Ok(vec![Response::update(ResponseBlueprint::new().set_embeds(vec![embed]).set_components(components))])
    }

    async fn invest(id: &mut Id, attribute_str: &str, selected_value: i64) -> Result<()> {
        let spent = min(selected_value, id.points_to_distribute);

        match attribute_str {
            "con" => id.constitution += spent,
            "spr" => id.spirit += spent,
            "mgt" => id.might += spent,
            "mov" => id.movement += spent,
            "dex" => id.dexterity += spent,
            "cog" => id.cognition += spent,
            "cha" => id.charisma += spent,
            invalid => {
                unreachable!("Invalid invested attribute on 'id distribute' component interaction: {invalid}")
            },
        }

        id.points_to_distribute -= spent;

        Ok(())
    }

    async fn generate_embed<'a>(id: &Id) -> Result<CreateEmbed<'a>> {
        let mut attributes_field = String::new();
        attributes_field += &f!("{CON_EMOJI} `{CON_SHORT}` {}\n", mark_thousands(id.constitution));
        attributes_field += &f!("{SPR_EMOJI} `{SPR_SHORT}` {}\n", mark_thousands(id.spirit));
        attributes_field += &f!("{MGT_EMOJI} `{MGT_SHORT}` {}\n", mark_thousands(id.might));
        attributes_field += &f!("{MOV_EMOJI} `{MOV_SHORT}` {}\n", mark_thousands(id.movement));
        attributes_field += &f!("{DEX_EMOJI} `{DEX_SHORT}` {}\n", mark_thousands(id.dexterity));
        attributes_field += &f!("{COG_EMOJI} `{COG_SHORT}` {}\n", mark_thousands(id.cognition));
        attributes_field += &f!("{CHA_EMOJI} `{CHA_SHORT}` {}\n", mark_thousands(id.charisma));

        let new_embed = CreateEmbed::new()
            .title(f!("{} 🎊 [{}]", id.name, mark_thousands(id.lvl)))
            .color(id.color.unwrap_or(0x36393e))
            .description(f!("{} / {}", mark_thousands(id.xp), mark_thousands(xp_to_next_level(id.lvl))))
            .field("", attributes_field, false)
            .footer(CreateEmbedFooter::new(f!(
                "{} pontos para distribuir.",
                mark_thousands(id.points_to_distribute)
            )));

        Ok(new_embed)
    }

    async fn generate_attribute_components<'a>(id: &Id) -> Result<Vec<CreateActionRow<'a>>> {
        let row_1 = CreateActionRow::Buttons(vec![
            CreateButton::new(f!("slash id distribute {} goto_incrementors con", id.tag))
                .emoji(ReactionType::Unicode(CON_EMOJI.parse()?)),
            CreateButton::new(f!("slash id distribute {} goto_incrementors spr", id.tag))
                .emoji(ReactionType::Unicode(SPR_EMOJI.parse()?)),
            CreateButton::new(f!("slash id distribute {} goto_incrementors mgt", id.tag))
                .emoji(ReactionType::Unicode(MGT_EMOJI.parse()?)),
            CreateButton::new(f!("slash id distribute {} goto_incrementors mov", id.tag))
                .emoji(ReactionType::Unicode(MOV_EMOJI.parse()?)),
        ]);

        let row_2 = CreateActionRow::Buttons(vec![
            CreateButton::new(f!("slash id distribute {} goto_incrementors dex", id.tag))
                .emoji(ReactionType::Unicode(DEX_EMOJI.parse()?)),
            CreateButton::new(f!("slash id distribute {} goto_incrementors cog", id.tag))
                .emoji(ReactionType::Unicode(COG_EMOJI.parse()?)),
            CreateButton::new(f!("slash id distribute {} goto_incrementors cha", id.tag))
                .emoji(ReactionType::Unicode(CHA_EMOJI.parse()?)),
        ]);

        Ok(vec![row_1, row_2])
    }

    async fn generate_incrementor_components<'a>(
        id_tag: &str,
        attribute_str: &str,
    ) -> Result<Vec<CreateActionRow<'a>>> {
        let custom_button_id_lead = f!("#slash id distribute {id_tag} invest_in {attribute_str} ");

        let button_1 = CreateButton::new(f!("{custom_button_id_lead}1")).label("+1");
        let button_4 = CreateButton::new(f!("{custom_button_id_lead}4")).label("+4");
        let button_10 = CreateButton::new(f!("{custom_button_id_lead}10")).label("+10");
        let button_100000 = CreateButton::new(f!("{custom_button_id_lead}100000")).label("+100000");

        let button_go_back = CreateButton::new(f!("#slash id distribute {id_tag} goto_attributes"))
            .emoji(ReactionType::Unicode(GO_BACK_EMOJI.parse()?));

        let row_1 = CreateActionRow::Buttons(vec![button_1, button_4, button_10, button_100000, button_go_back]);

        Ok(vec![row_1])
    }
}
