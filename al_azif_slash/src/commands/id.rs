use crate::prelude::*;

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("id").description("About the existing Ids")
        .description_localized("pt-BR", "Sobre os Ids existentes")
        .add_option(CreateCommandOption::new(CommandOptionType::SubCommand, "distribute", "Distribute points to the attributes of the specified Id")
            .name_localized("pt-BR", "distribuir")
            .description_localized("pt-BR", "Distribuir pontos para os atributos do Id especificado")
            .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "id", "The Id to distribute")
                .description_localized("pt-BR", "O Id para distribuir")
                .required(true)
            )
        )
}

pub async fn run_command(bot: &impl AsBot, _slash: &CommandInteraction, args: &[ResolvedOption<'_>]) -> Result<Vec<ResponseModel>> {
    let ResolvedValue::SubCommand(inner_args) = &args[0].value else {
        unreachable!("The first argument of the 'id' command must be a subcommand!");
    };

    match args[0].name {
        "distribute" => distribute::run_command(bot, inner_args).await,
        invalid => unreachable!("Invalid command branch for 'id' command: {invalid}")
    }
}

pub async fn run_component(bot: &impl AsBot, _comp: &ComponentInteraction, args: &[&str]) -> Result<Vec<ResponseModel>> {
    match args[0] {
        "distribute" => distribute::run_component(bot, &args[1..]).await,
        invalid => unreachable!("Invalid component branch for 'id' components: {invalid}")
    }
}

mod distribute {
    use super::*;

    pub async fn run_command(bot: &impl AsBot, args: &[ResolvedOption<'_>]) -> Result<Vec<ResponseModel>> {
        let ResolvedValue::String(id_tag) = args[0].value else {
            unreachable!("The 'id' argument of the 'id distribute' command must be a string!");
        };
        let Ok(id_m) = Mirror::<Id>::get(bot, id_tag).await else {
            return response::simple_send("Informe um Id válido.");
        };
    
        let id = id_m.read().await;
        let embed = generate_embed(&id).await?;
        let components = generate_attribute_components(&id).await?;
        
        Ok(vec![ResponseModel::send(vec![ResponseBlueprint::default().embeds(vec![embed]).components(components)])])
    }

    pub async fn run_component(bot: &impl AsBot, args: &[&str]) -> Result<Vec<ResponseModel>> {
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
            invalid => unreachable!("Invalid operation on 'id distribute' component interaction: {invalid}")
        }
    
        let embed = generate_embed(&*id_m.read().await).await?;
        
        Ok(vec![ResponseModel::update(ResponseBlueprint::default().embeds(vec![embed]).components(components))])
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
            invalid => unreachable!("Invalid invested attribute on 'id distribute' component interaction: {invalid}")
        }
        
        id.points_to_distribute -= spent;

        Ok(())
    }

    async fn generate_embed(id: &Id) -> Result<CreateEmbed<'static>> {
        let mut attributes_field = String::new();
        attributes_field += &f!("{CON_EMOJI} `{CON_SHORT}` {}\n", mark_thousands(id.constitution));
        attributes_field += &f!("{SPR_EMOJI} `{SPR_SHORT}` {}\n", mark_thousands(id.spirit));
        attributes_field += &f!("{MGT_EMOJI} `{MGT_SHORT}` {}\n", mark_thousands(id.might));
        attributes_field += &f!("{MOV_EMOJI} `{MOV_SHORT}` {}\n", mark_thousands(id.movement));
        attributes_field += &f!("{DEX_EMOJI} `{DEX_SHORT}` {}\n", mark_thousands(id.dexterity));
        attributes_field += &f!("{COG_EMOJI} `{COG_SHORT}` {}\n", mark_thousands(id.cognition));
        attributes_field += &f!("{CHA_EMOJI} `{CHA_SHORT}` {}\n", mark_thousands(id.charisma));
    
        let embed = CreateEmbed::new()
            .title(f!("{} 🎊 [{}]", id.name, mark_thousands(id.lvl)))
            .color(id.color.unwrap_or(0x36393e))
            .description(f!("{} / {}", mark_thousands(id.xp), mark_thousands(xp_to_next_level(id.lvl))))
            .field("", attributes_field, false)
            .footer(CreateEmbedFooter::new(f!("{} pontos para distribuir.", mark_thousands(id.points_to_distribute))));
    
        Ok(embed)
    }

    async fn generate_attribute_components(id: &Id) -> Result<Vec<CreateActionRow<'static>>> {
        let row_1 = CreateActionRow::Buttons(vec![
            CreateButton::new(f!("slash id distribute {} goto_incrementors con", id.tag)).emoji(ReactionType::Unicode(CON_EMOJI.parse()?)),
            CreateButton::new(f!("slash id distribute {} goto_incrementors spr", id.tag)).emoji(ReactionType::Unicode(SPR_EMOJI.parse()?)),
            CreateButton::new(f!("slash id distribute {} goto_incrementors mgt", id.tag)).emoji(ReactionType::Unicode(MGT_EMOJI.parse()?)),
            CreateButton::new(f!("slash id distribute {} goto_incrementors mov", id.tag)).emoji(ReactionType::Unicode(MOV_EMOJI.parse()?)),
        ]);
    
        let row_2 = CreateActionRow::Buttons(vec![
            CreateButton::new(f!("slash id distribute {} goto_incrementors dex", id.tag)).emoji(ReactionType::Unicode(DEX_EMOJI.parse()?)),
            CreateButton::new(f!("slash id distribute {} goto_incrementors cog", id.tag)).emoji(ReactionType::Unicode(COG_EMOJI.parse()?)),
            CreateButton::new(f!("slash id distribute {} goto_incrementors cha", id.tag)).emoji(ReactionType::Unicode(CHA_EMOJI.parse()?)),
        ]);
    
        Ok(vec![row_1, row_2])
    }

    async fn generate_incrementor_components(id_tag: &str, attribute_str: &str) -> Result<Vec<CreateActionRow<'static>>> {
        let custom_button_id_prefix = f!("slash id distribute {id_tag} invest_in {attribute_str} ");
    
        let button_1 = CreateButton::new(f!("{custom_button_id_prefix}1"))
            .label("+1");
        let button_4 = CreateButton::new(f!("{custom_button_id_prefix}4"))
            .label("+4");
        let button_10 = CreateButton::new(f!("{custom_button_id_prefix}10"))
            .label("+10");
        let button_100000 = CreateButton::new(f!("{custom_button_id_prefix}100000"))
            .label("+100000");
        let button_go_back = CreateButton::new(f!("slash id distribute {id_tag} goto_attributes"))
            .emoji(ReactionType::Unicode(GO_BACK_EMOJI.parse()?));

        let row_1 = CreateActionRow::Buttons(vec![button_1, button_4, button_10, button_100000, button_go_back]);
        
        Ok(vec![row_1])
    }
}

