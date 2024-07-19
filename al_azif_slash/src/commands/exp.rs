use crate::prelude::*;

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("exp")
        .description("About experience")
        .description_localized("pt-BR", "Sobre experiência")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "bestow",
                "Grant experience to the specified Ids",
            )
            .name_localized("pt-BR", "conceder")
            .description_localized("pt-BR", "Conceder experiência para os Ids especificados")
            .add_sub_option(
                CreateCommandOption::new(CommandOptionType::String, "ids", "The Ids to bestow")
                    .name_localized("pt-BR", "ids")
                    .description_localized("pt-BR", "Os Ids para conceder")
                    .required(true),
            )
            .add_sub_option(
                CreateCommandOption::new(CommandOptionType::Integer, "value", "The amount to add")
                    .name_localized("pt-BR", "valor")
                    .description_localized("pt-BR", "Quantia a acrescentar")
                    .required(true),
            ),
        )
}

pub async fn run_command<'a>(
    bot: &impl AsBot,
    _slash: &CommandInteraction,
    args: &[ResolvedOption<'_>],
) -> Result<Models<'a>> {
    let ResolvedValue::SubCommand(inner_args) = &args[0].value else {
        unreachable!("The first argument of the 'exp' command must be a subcommand!");
    };

    match args[0].name {
        "bestow" => bestow::run_command(bot, inner_args).await,
        invalid => unreachable!("Invalid command branch for 'exp' command: {invalid}"),
    }
}

mod bestow {
    use super::*;

    pub async fn run_command<'a>(
        bot: &impl AsBot,
        args: &[ResolvedOption<'_>],
    ) -> Result<Vec<ResponseModel<'a>>> {
        let mut blueprints = Vec::new();

        let ResolvedValue::String(id_tags) = args[0].value else {
            unreachable!("The 'ids' argument of the 'exp bestow' command must be a string!");
        };

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
            let content = if invalid_id_tags.len() > 1 {
                let concat_tags = join_with_and(
                    invalid_id_tags
                        .iter()
                        .map(|tag| f!("`{tag}`"))
                        .collect::<Vec<String>>(),
                );
                f!("Os Ids {concat_tags } não foram encontrados.")
            } else {
                f!(
                    "O Id `{}` não foi encontrado.",
                    invalid_id_tags.first().unwrap()
                )
            };

            blueprints.push(ResponseBlueprint::default().assign_content(content));
        }

        if id_ms.is_empty() {
            blueprints.push(
                ResponseBlueprint::default()
                    .assign_content("Você precisa de pelo menos um Id para conceder experiência."),
            );
            return Ok(vec![ResponseModel::send(blueprints)]);
        }

        let ResolvedValue::Integer(value) = args[1].value else {
            unreachable!("The 'value' argument of the 'exp bestow' command must be an integer!");
        };

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
                let mut content = f!(
                    "**{}** obteve **{}** de experiência e subiu de nível [**{}** ➜ **{}**]",
                    id.name,
                    mark_thousands(value),
                    mark_thousands(previous_lvl),
                    mark_thousands(id.lvl)
                );
                content.push_str(&f!(
                    "Pode distribuir **{}** pontos nos atributos.",
                    mark_thousands(id.points_to_distribute)
                ));
                content.push_str(&f!(
                    "Falta **{}** de experiência para o próximo nível.",
                    mark_thousands(xp_to_next_level(id.lvl) - id.xp)
                ));

                blueprints.push(ResponseBlueprint::default().assign_content(content));
            } else {
                blueprints.push(ResponseBlueprint::default().assign_content(f!(
                    "**{}** obteve **{}** de experiência. Falta **{}** para o próximo nível.",
                    id.name,
                    mark_thousands(value),
                    mark_thousands(xp_to_next_level(id.lvl) - id.xp)
                )));
            }
        }

        Ok(vec![ResponseModel::send(blueprints)])
    }
}
