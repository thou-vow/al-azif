use crate::_prelude::*;

pub const NAME: &str = "battle";
pub const DESCRIPTION: &str = "About battle";
pub const NAME_PT: &str = "batalha";
pub const DESCRIPTION_PT: &str = "Sobre batalha";

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new(NAME)
        .description(DESCRIPTION)
        .name_localized("pt-BR", NAME_PT)
        .description_localized("pt-BR", DESCRIPTION_PT)
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, end::NAME, end::DESCRIPTION)
                .name_localized("pt-BR", end::NAME_PT)
                .description_localized("pt-BR", end::DESCRIPTION_PT),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, join::NAME, join::DESCRIPTION)
                .name_localized("pt-BR", join::NAME_PT)
                .description_localized("pt-BR", join::DESCRIPTION_PT)
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "ids", "The Ids to join")
                        .description_localized("pt-BR", "Os Ids para ingressar")
                        .required(true),
                ),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, start::NAME, start::DESCRIPTION)
                .name_localized("pt-BR", start::NAME_PT)
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

    pub const NAME: &str = "end";
    pub const DESCRIPTION: &str = "End the battle";
    pub const NAME_PT: &str = "terminar";
    pub const DESCRIPTION_PT: &str = "Encerrar a batalha";

    pub async fn run_slash<'a>(bot: &impl AsBot, slash: &CommandInteraction) -> Result<Responses<'a>> {
        let battle_tag = slash.channel_id.to_string().into_boxed_str();
        let Ok(battle_m) = Mirror::<Battle>::get(bot, &battle_tag).await else {
            return Ok(response::simple_send_and_delete("Não há uma batalha acontecendo neste canal."));
        };

        let battle = battle_m.read().await;
        for id_tag in battle.opponents.keys() {
            let id_m = Mirror::<Id>::get(bot, id_tag).await?;
            let mut id = id_m.write().await;
            id.current_battle_tag = None;
        }

        Mirror::<Battle>::cut(bot, &battle_tag).await?;

        Ok(response::simple_send("Batalha finalizada."))
    }
}

pub mod join {
    use super::*;

    pub const NAME: &str = "join";
    pub const DESCRIPTION: &str = "Join Ids to the battle";
    pub const NAME_PT: &str = "participar";
    pub const DESCRIPTION_PT: &str = "Integrar Ids na batalha";

    pub async fn run_slash<'a>(bot: &impl AsBot, slash: &CommandInteraction, id_tags: &str) -> Result<Responses<'a>> {
        let battle_tag = slash.channel_id.to_string().into_boxed_str();
        let Ok(battle_m) = Mirror::<Battle>::get(bot, &battle_tag).await else {
            return Ok(response::simple_send("Não há uma batalha neste canal."));
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
                let concat_tags =
                    join_with_and(invalid_id_tags.iter().map(|tag| f!("`{tag}`")).collect::<Vec<String>>());
                f!("Os Ids {concat_tags } não foram encontrados.")
            } else {
                f!("O Id `{}` não foi encontrado.", invalid_id_tags.first().unwrap())
            };

            return Ok(response::simple_send_and_delete(new_content));
        }

        if !already_in_battle_id_tags.is_empty() {
            let new_content = if already_in_battle_id_tags.len() > 1 {
                let concat_tags = join_with_and(
                    already_in_battle_id_tags.iter().map(|tag| f!("`{tag}`")).collect::<Vec<String>>(),
                );
                f!("Os Ids {concat_tags } já estão em batalha.")
            } else {
                f!("O Id `{}` já está em batalha.", already_in_battle_id_tags.first().unwrap())
            };

            return Ok(response::simple_send_and_delete(new_content));
        }

        let mut joined_id_names = Vec::new();
        let mut battle = battle_m.write().await;
        for id_m in id_ms {
            let mut id = id_m.write().await;
            id.join_battle(&mut battle).await;
            joined_id_names.push(id.name.clone());
        }

        let content = if joined_id_names.len() > 1 {
            let concat_names =
                joined_id_names.iter().map(|name| f!("**{name}**")).collect::<Vec<String>>().join(", ");
            f!("⚔️ | {concat_names} entraram na batalha.")
        } else {
            f!("⚔️ | **{}** entrou na batalha.", joined_id_names.first().unwrap())
        };

        blueprints.push(ResponseBlueprint::new().set_content(content));

        Ok(vec![Response::send(blueprints)])
    }
}

pub mod start {
    use super::*;

    pub const NAME: &str = "start";
    pub const DESCRIPTION: &str = "Start a battle";
    pub const NAME_PT: &str = "iniciar";
    pub const DESCRIPTION_PT: &str = "Iniciar uma batalha";

    pub async fn run_slash<'a>(
        bot: &impl AsBot,
        slash: &CommandInteraction,
        id_tags: &str,
    ) -> Result<Vec<Response<'a>>> {
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
                let concat_tags =
                    join_with_and(invalid_id_tags.iter().map(|tag| f!("`{tag}`")).collect::<Vec<_>>());
                f!("Os Ids {concat_tags } não foram encontrados.")
            } else {
                f!("O Id `{}` não foi encontrado.", invalid_id_tags.first().unwrap())
            };

            return Ok(response::simple_send_and_delete(new_content));
        }

        if !already_in_battle_id_tags.is_empty() {
            let new_content = if already_in_battle_id_tags.len() > 1 {
                let concat_tags = join_with_and(
                    already_in_battle_id_tags.iter().map(|tag| f!("`{tag}`")).collect::<Vec<String>>(),
                );
                f!("Os Ids {concat_tags } já estão em batalha.")
            } else {
                f!("O Id `{}` já está em batalha.", already_in_battle_id_tags.first().unwrap())
            };

            return Ok(response::simple_send_and_delete(new_content));
        }

        if id_ms.len() < 2 {
            return Ok(response::simple_send_and_delete("Precisa de pelo menos 2 Ids para iniciar uma batalha."));
        }

        let mut blueprints = Vec::new();

        let mut battle = Battle::new(battle_tag);
        for id_m in id_ms {
            id_m.write().await.join_battle(&mut battle).await;
        }

        blueprints.extend(advance(bot, &mut battle).await?);
        blueprints.push(battle.generate_turn_screen(bot).await?);
        Mirror::<Battle>::set_and_get(bot, battle).await?;

        Ok(vec![Response::send(blueprints)])
    }
}
