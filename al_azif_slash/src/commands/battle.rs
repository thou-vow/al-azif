use crate::_prelude::*;

pub const TAG: &str = "battle";
pub const DESCRIPTION: &str = "About battle";
pub const TAG_PT: &str = "batalha";
pub const DESCRIPTION_PT: &str = "Sobre batalha";

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new(TAG)
        .description(DESCRIPTION)
        .name_localized("pt-BR", TAG_PT)
        .description_localized("pt-BR", DESCRIPTION_PT)
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, end::TAG, end::DESCRIPTION)
                .name_localized("pt-BR", end::TAG_PT)
                .description_localized("pt-BR", end::DESCRIPTION_PT),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, join::TAG, join::DESCRIPTION)
                .name_localized("pt-BR", join::TAG_PT)
                .description_localized("pt-BR", join::DESCRIPTION_PT)
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "ids", "The Ids to join")
                        .description_localized("pt-BR", "Os Ids para ingressar")
                        .required(true),
                ),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, start::TAG, start::DESCRIPTION)
                .name_localized("pt-BR", start::TAG_PT)
                .description_localized("pt-BR", start::DESCRIPTION_PT)
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "ids", "The Ids to join")
                        .description_localized("pt-BR", "Os Ids para ingressar")
                        .required(true),
                ),
        )
}

pub mod end {
    use super::*;

    pub const TAG: &str = "end";
    pub const DESCRIPTION: &str = "End the battle";
    pub const TAG_PT: &str = "terminar";
    pub const DESCRIPTION_PT: &str = "Encerrar a batalha";

    pub async fn run_slash(bot: &impl AsBot, slash: &CommandInteraction) -> Result<Responses> {
        let battle_tag = slash.channel_id.to_string().into_boxed_str();
        let Ok(battle_m) = Mirror::<Battle>::get(bot, &battle_tag).await else {
            return Err(SlashError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(bot,
                en: "No battle is currently happening in this channel.",
                pt: "Não há uma batalha acontecendo neste canal."
            ))]));
        };

        let battle = battle_m.read().await;
        for opponent in battle.opponents.iter() {
            let id_m = Mirror::<Id>::get(bot, &opponent.tag).await?;
            let mut id = id_m.write().await;
            id.current_battle_tag = None;
        }

        Mirror::<Battle>::cut(bot, &battle_tag).await?;

        Ok(response::simple_send(lang_diff!(bot,
            en: "Battle ended.",
            pt: "Batalha encerrada."
        )))
    }
}

pub mod join {
    use super::*;

    pub const TAG: &str = "join";
    pub const DESCRIPTION: &str = "Join Ids to the battle";
    pub const TAG_PT: &str = "participar";
    pub const DESCRIPTION_PT: &str = "Integrar Ids na batalha";

    pub async fn run_slash(bot: &impl AsBot, slash: &CommandInteraction, id_tags: &str) -> Result<Responses> {
        let battle_tag = slash.channel_id.to_string();
        let Ok(battle_m) = Mirror::<Battle>::get(bot, &battle_tag).await else {
            return Err(SlashError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(bot,
                en: "No battle is currently happening in this channel.",
                pt: "Não há uma batalha acontecendo neste canal."
            ))]));
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
            if id_m.read().await.current_battle_tag.is_some() {
                already_in_battle_id_tags.push(id_tag);
                continue;
            }
            id_ms.push(id_m);
        }

        if !invalid_id_tags.is_empty() {
            let new_content = if invalid_id_tags.len() > 1 {
                let concat_tags = join_with_and(&invalid_id_tags.iter().map(|tag| f!("`{tag}`")).collect::<Vec<String>>());
                lang_diff!(bot,
                    en: f!("The Ids {concat_tags} were not found."),
                    pt: f!("Os Ids {concat_tags} não foram encontrados.")
                )
            } else {
                lang_diff!(bot,
                    en: f!("The Id `{}` was not found.", invalid_id_tags.first().unwrap()),
                    pt: f!("O Id `{}` não foi encontrado.", invalid_id_tags.first().unwrap()
                ))
            };

            return Err(SlashError::Expected(vec![ResponseBlueprint::with_content(new_content)]));
        }

        if !already_in_battle_id_tags.is_empty() {
            let new_content = if already_in_battle_id_tags.len() > 1 {
                let concat_tags = join_with_and(&already_in_battle_id_tags.iter().map(|tag| f!("`{tag}`")).collect::<Vec<String>>());
                lang_diff!(bot,
                    en: f!("The Ids {concat_tags} are already in a battle."),
                    pt: f!("Os Ids {concat_tags } já estão em batalha.")
                )
            } else {
                lang_diff!(bot,
                    en: f!("The Id `{}` is already in a battle.", already_in_battle_id_tags.first().unwrap()),
                    pt: f!("O Id `{}` já está em batalha.", already_in_battle_id_tags.first().unwrap())
                )
            };

            return Err(SlashError::Expected(vec![ResponseBlueprint::with_content(new_content)]));
        }

        let mut joined_id_names = Vec::new();
        let mut battle = battle_m.write().await;
        for id_m in id_ms {
            let mut id = id_m.write().await;
            id.join_battle(&mut battle).await;
            joined_id_names.push(id.name.clone());
        }

        let content = if joined_id_names.len() > 1 {
            let concat_names = joined_id_names.iter().map(|name| f!("**{name}**")).collect::<Vec<String>>().join(", ");
            lang_diff!(bot,
                en: f!("⚔️ | {concat_names} joined the battle."),
                pt: f!("⚔️ | {concat_names} entraram na batalha.")
            )
        } else {
            lang_diff!(bot,
                en: f!("⚔️ | **{}** joined the battle.", joined_id_names.first().unwrap()),
                pt: f!("⚔️ | **{}** entrou na batalha.", joined_id_names.first().unwrap())
            )
        };

        blueprints.push(ResponseBlueprint::new().set_content(content));

        Ok(vec![Response::send(blueprints)])
    }
}

pub mod start {
    use super::*;

    pub const TAG: &str = "start";
    pub const DESCRIPTION: &str = "Start a battle";
    pub const TAG_PT: &str = "iniciar";
    pub const DESCRIPTION_PT: &str = "Iniciar uma batalha";

    pub async fn run_slash(bot: &impl AsBot, slash: &CommandInteraction, id_tags: &str) -> Result<Vec<Response>> {
        let battle_tag = slash.channel_id.to_string();
        if Mirror::<Battle>::get(bot, &battle_tag).await.is_ok() {
            return Ok(response::simple_send("Já está ocorrendo uma batalha neste canal."));
        }

        let mut invalid_id_tags = Vec::new();
        let mut already_in_battle_id_tags = Vec::new();
        let mut id_ms = Vec::new();
        for id_tag in id_tags.split_ascii_whitespace() {
            let Ok(id_m) = Mirror::<Id>::get(bot, id_tag).await else {
                invalid_id_tags.push(id_tag);
                continue;
            };
            if id_m.read().await.current_battle_tag.is_some() {
                already_in_battle_id_tags.push(id_tag);
                continue;
            }
            id_ms.push(id_m);
        }

        if !invalid_id_tags.is_empty() {
            let new_content = if invalid_id_tags.len() > 1 {
                let concat_tags = join_with_and(&invalid_id_tags.iter().map(|tag| f!("`{tag}`")).collect::<Vec<String>>());
                lang_diff!(bot,
                    en: f!("The Ids {concat_tags} were not found."),
                    pt: f!("Os Ids {concat_tags} não foram encontrados.")
                )
            } else {
                lang_diff!(bot,
                    en: f!("The Id `{}` was not found.", invalid_id_tags.first().unwrap()),
                    pt: f!("O Id `{}` não foi encontrado.", invalid_id_tags.first().unwrap()
                ))
            };

            return Err(SlashError::Expected(vec![ResponseBlueprint::with_content(new_content)]));
        }

        if !already_in_battle_id_tags.is_empty() {
            let new_content = if already_in_battle_id_tags.len() > 1 {
                let concat_tags = join_with_and(&already_in_battle_id_tags.iter().map(|tag| f!("`{tag}`")).collect::<Vec<String>>());
                lang_diff!(bot,
                    en: f!("The Ids {concat_tags} are already in a battle."),
                    pt: f!("Os Ids {concat_tags } já estão em batalha.")
                )
            } else {
                lang_diff!(bot,
                    en: f!("The Id `{}` is already in a battle.", already_in_battle_id_tags.first().unwrap()),
                    pt: f!("O Id `{}` já está em batalha.", already_in_battle_id_tags.first().unwrap())
                )
            };

            return Err(SlashError::Expected(vec![ResponseBlueprint::with_content(new_content)]));
        }

        if id_ms.len() < 2 {
            return Err(SlashError::Expected(vec![ResponseBlueprint::with_content(lang_diff!(bot,
                en: "You need at least 2 Ids to start a battle.",
                pt: "Precisa de pelo menos 2 Ids para iniciar uma batalha."
            ))]));
        }

        let mut blueprints = Vec::new();

        let mut battle = Battle::new(battle_tag);
        for id_m in id_ms {
            id_m.write().await.join_battle(&mut battle).await;
        }

        blueprints.extend(battle.advance(bot).await?);
        blueprints.push(battle.generate_turn_screen(bot).await?);
        Mirror::<Battle>::set_and_get(bot, battle).await?;

        Ok(vec![Response::send(blueprints)])
    }
}
