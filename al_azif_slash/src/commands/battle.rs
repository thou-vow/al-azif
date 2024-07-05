use crate::prelude::*;

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("battle").description("About battle")
        .name_localized("pt-BR", "batalha")
        .description_localized("pt-BR", "Sobre batalha")
        .add_option(CreateCommandOption::new(CommandOptionType::SubCommand, "end", "End the battle within this channel")
            .name_localized("pt-BR", "terminar")
            .description_localized("pt-BR", "Encerrar a batalha deste canal")
        )
        .add_option(CreateCommandOption::new(CommandOptionType::SubCommand, "join", "Join Ids to the battle")
            .name_localized("pt-BR", "participar")
            .description_localized("pt-BR", "Integrar Ids na batalha")
            .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "ids", "The Ids to join")
                .description_localized("pt-BR", "Os Ids para ingressar")
                .required(true)
            )
        )
        .add_option(CreateCommandOption::new(CommandOptionType::SubCommand, "start", "Start a battle within this channel")
            .name_localized("pt-BR", "iniciar")
            .description_localized("pt-BR", "Iniciar uma batalha neste canal")
            .add_sub_option(CreateCommandOption::new(CommandOptionType::String, "ids", "The Ids to join")
                .description_localized("pt-BR", "Os Ids para ingressar")
                .required(true)
            )
        )
}

pub async fn run_command(bot: &impl AsBot, slash: &CommandInteraction, args: &[ResolvedOption<'_>]) -> Result<Vec<ResponseModel>> {
    let ResolvedValue::SubCommand(inner_args) = &args[0].value else {
        unreachable!("The first argument of the 'id' command must be a subcommand!");
    };

    match args[0].name {
        "end" => end::run_command(bot, slash).await,
        "join" => join::run_command(bot, slash, inner_args).await,
        "start" => start::run_command(bot, slash, inner_args).await,
        invalid => unreachable!("Invalid command branch for 'id' command: {invalid}")
    }
}

mod end {
    use super::*;

    pub async fn run_command(bot: &impl AsBot, slash: &CommandInteraction) -> Result<Vec<ResponseModel>> {
        let battle_tag = slash.channel_id.to_string().into_boxed_str();
        let Ok(battle_m) = Mirror::<Battle>::get(bot, &battle_tag).await else {
            return simple_send_response("Não há uma batalha neste canal.", false);
        };

        let battle = battle_m.read().await;
        for id_tag in battle.opponents.keys() {
            let id_m = Mirror::<Id>::get(bot, id_tag).await?;
            let mut id = id_m.write().await;
            id.current_battle = None;
        }

        Mirror::<Battle>::cut(bot, &battle_tag).await?;

        simple_send_response("Batalha finalizada.", false)
    }
}

mod join {
    use super::*;

    pub async fn run_command(bot: &impl AsBot, slash: &CommandInteraction, args: &[ResolvedOption<'_>]) -> Result<Vec<ResponseModel>> {
        let battle_tag = slash.channel_id.to_string().into_boxed_str();
        let Ok(battle_m) = Mirror::<Battle>::get(bot, &battle_tag).await else {
            return simple_send_response("Não há uma batalha neste canal.", false);
        };

        let ResolvedValue::String(id_tags) = args[0].value else {
            unreachable!("The 'ids' argument of the 'battle start' command must be a string!");
        };

        let mut blueprints = Vec::new();
        
        let mut invalid_id_tags = Vec::new();
        let mut already_in_battle_id_tags = Vec::new();
        let mut id_ms = Vec::new();
        for id_tag in id_tags.split_ascii_whitespace() {
            let Ok(id_m) = Mirror::<Id>::get(bot, id_tag).await else {
                invalid_id_tags.push(id_tag);
                continue;
            };
            if id_m.read().await.current_battle.is_some() {
                already_in_battle_id_tags.push(id_tag);
                continue;
            }
            id_ms.push(id_m);
        }

        if !invalid_id_tags.is_empty() {
            blueprints.push(ResponseBlueprint::default().content(f!(
                "Id não encontrado: {}",
                invalid_id_tags.iter().map(|tag| f!("`{tag}`")).collect::<Vec<String>>().join(", ")
            )));
        }
        if !already_in_battle_id_tags.is_empty() {
            blueprints.push(ResponseBlueprint::default().content(f!(
                "Id já está em uma batalha: {}",
                already_in_battle_id_tags.iter().map(|tag| f!("`{tag}`")).collect::<Vec<String>>().join(", ")
            )));
        }

        if id_ms.is_empty() {
            blueprints.push(ResponseBlueprint::default().content("Nenhum Id entrou na batalha."));
            return Ok(vec![ResponseModel::send(blueprints)]);
        }

        let mut battle = battle_m.write().await;
        for id_m in id_ms {
            id_m.write().await.join_battle(&mut battle).await;
        }

        blueprints.push(battle.generate_turn_screen(bot).await?);

        Ok(vec![ResponseModel::send(blueprints)])
    }
}

mod start {
    use super::*;

    pub async fn run_command(bot: &impl AsBot, slash: &CommandInteraction, args: &[ResolvedOption<'_>]) -> Result<Vec<ResponseModel>> {
        let battle_tag = slash.channel_id.to_string().into_boxed_str();
        if Mirror::<Battle>::get(bot, &battle_tag).await.is_ok() {
            return simple_send_response("Já está ocorrendo uma batalha neste canal.", false);
        }

        let ResolvedValue::String(id_tags) = args[0].value else {
            unreachable!("The 'ids' argument of the 'battle start' command must be a string!");
        };

        let mut blueprints = Vec::new();
        
        let mut invalid_id_tags = Vec::new();
        let mut already_in_battle_id_tags = Vec::new();
        let mut id_ms = Vec::new();
        for id_tag in id_tags.split_ascii_whitespace() {
            let Ok(id_m) = Mirror::<Id>::get(bot, id_tag).await else {
                invalid_id_tags.push(id_tag);
                continue;
            };
            if id_m.read().await.current_battle.is_some() {
                already_in_battle_id_tags.push(id_tag);
                continue;
            }
            id_ms.push(id_m);
        }

        if !invalid_id_tags.is_empty() {
            blueprints.push(ResponseBlueprint::default().content(f!(
                "Id não encontrado: {}",
                invalid_id_tags.iter().map(|tag| f!("`{tag}`")).collect::<Vec<String>>().join(", ")
            )));
        }
        if !already_in_battle_id_tags.is_empty() {
            blueprints.push(ResponseBlueprint::default().content(f!(
                "Id já está em uma batalha: {}",
                already_in_battle_id_tags.iter().map(|tag| f!("`{tag}`")).collect::<Vec<String>>().join(", ")
            )));
        }

        if id_ms.len() < 2 {
            blueprints.push(ResponseBlueprint::default().content("Você precisa de pelo menos 2 Ids para iniciar uma batalha."));
            return Ok(vec![ResponseModel::send(blueprints)]);
        }

        let mut battle = Battle::new(&battle_tag);
        for id_m in id_ms {
            id_m.write().await.join_battle(&mut battle).await;
        }

        
        blueprints.extend(advance(bot, &mut battle).await?);
        blueprints.push(battle.generate_turn_screen(bot).await?);
        Mirror::<Battle>::set_and_get(bot, battle).await?;

        Ok(vec![ResponseModel::send(blueprints)])
    }
}