use crate::_prelude::*;

pub const NAME: &str = "exp";
pub const DESCRIPTION: &str = "About experience";
pub const NAME_PT: &str = "exp";
pub const DESCRIPTION_PT: &str = "Sobre experiência";

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new(NAME)
        .description(DESCRIPTION)
        .name_localized("pt-BR", NAME_PT)
        .description_localized("pt-BR", "Sobre experiência")
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, bestow::NAME, bestow::DESCRIPTION)
                .name_localized("pt-BR", bestow::NAME_PT)
                .description_localized("pt-BR", bestow::DESCRIPTION_PT)
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::String, "ids", "The Ids to bestow")
                        .name_localized("pt-BR", "ids")
                        .description_localized("pt-BR", "Os Ids para conceder")
                        .required(true),
                )
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::Integer, "amount", "The amount to add")
                        .name_localized("pt-BR", "quantia")
                        .description_localized("pt-BR", "Quantia a acrescentar")
                        .required(true),
                ),
        )
}

pub mod bestow {
    use super::*;

    pub const NAME: &str = "bestow";
    pub const DESCRIPTION: &str = "Grant experience to the specified Ids";
    pub const NAME_PT: &str = "conceder";
    pub const DESCRIPTION_PT: &str = "Conceder experiência para os Ids especificados";

    pub async fn run_slash(bot: &impl AsBot, id_tags: &str, value: i64) -> Result<Responses> {
        let mut blueprints = Vec::new();

        let mut id_ms = Vec::new();
        let mut invalid_id_tags = Vec::new();

        for id_tag in id_tags.split_ascii_whitespace() {
            let Ok(id_m) = Mirror::<Id>::get(bot, id_tag).await else {
                invalid_id_tags.push(id_tag);
                continue;
            };
            id_ms.push(id_m);
        }

        if !invalid_id_tags.is_empty() {
            let new_content = if invalid_id_tags.len() > 1 {
                let concat_tags = join_with_and(invalid_id_tags.iter().map(|tag| f!("`{tag}`")).collect::<Vec<String>>());
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

        for id_m in id_ms {
            let mut id = id_m.write().await;
            id.xp += value;

            let previous_lvl = id.lvl;
            let mut needed_xp: i64 = xp_to_next_level(id.lvl);
            while id.xp >= needed_xp {
                id.lvl += 1;
                id.xp -= needed_xp;
                id.points_to_distribute += 10;
                needed_xp = xp_to_next_level(id.lvl);
            }

            if previous_lvl != id.lvl {
                let mut new_content = f!(
                    "**{}** obteve **{}** de experiência e subiu de nível [**{}** ➜ **{}**]",
                    id.name,
                    mark_thousands(value),
                    mark_thousands(previous_lvl),
                    mark_thousands(id.lvl)
                );
                new_content.push_str(&f!("Pode distribuir **{}** pontos nos atributos.", mark_thousands(id.points_to_distribute)));
                new_content
                    .push_str(&f!("Falta **{}** de experiência para o próximo nível.", mark_thousands(xp_to_next_level(id.lvl) - id.xp)));

                blueprints.push(ResponseBlueprint::new().set_content(new_content));
            } else {
                blueprints.push(ResponseBlueprint::new().set_content(f!(
                    "**{}** obteve **{}** de experiência. Falta **{}** para o próximo nível.",
                    id.name,
                    mark_thousands(value),
                    mark_thousands(xp_to_next_level(id.lvl) - id.xp)
                )));
            }
        }

        Ok(vec![Response::send(blueprints)])
    }
}
