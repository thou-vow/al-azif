use crate::_prelude::*;

pub const NAME: &str = "exp";
pub const DESCRIPTION: &str = "About experience";
pub const NAME_LOCALIZED: &str = "exp";
pub const DESCRIPTION_LOCALIZED: &str = "Sobre experiência";

pub enum SubCommand<'a> {
    Bestow { args: &'a [ResolvedOption<'a>] },
}
impl<'a> SubCommand<'a> {
    pub fn from_args(args: &'a [ResolvedOption<'a>]) -> Option<Self> {
        use ResolvedOption as RO;
        use ResolvedValue as RV;

        match args {
            [RO {
                name: bestow::NAME,
                value: RV::SubCommand(sub_args),
                ..
            }, ..] => Some(Self::Bestow { args: sub_args }),
            _ => None,
        }
    }
    pub fn all() -> [Self; 1] {
        [Self::Bestow { args: &[] }]
    }
    pub fn all_localized_order() -> [Self; 1] {
        let mut all = SubCommand::all();
        all.sort_by(|a, b| a.get_name_localized().cmp(b.get_name_localized()));
        all
    }
}
impl<'a> SubCommand<'a> {
    pub fn get_name_localized(&self) -> &'static str {
        match self {
            Self::Bestow { .. } => bestow::NAME_LOCALIZED,
        }
    }
    pub fn get_description_localized(&self) -> &'static str {
        match self {
            Self::Bestow { .. } => bestow::DESCRIPTION_LOCALIZED,
        }
    }
    pub async fn run(
        &self,
        bot: &impl AsBot,
    ) -> Result<Responses<'a>> {
        match self {
            Self::Bestow { args } => bestow::run(bot, args).await,
        }
    }
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new(NAME)
        .description(DESCRIPTION)
        .name_localized("pt-BR", NAME_LOCALIZED)
        .description_localized("pt-BR", "Sobre experiência")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                bestow::NAME,
                bestow::DESCRIPTION,
            )
            .name_localized("pt-BR", bestow::NAME_LOCALIZED)
            .description_localized("pt-BR", bestow::DESCRIPTION_LOCALIZED)
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

mod bestow {
    use super::*;

    pub const NAME: &str = "bestow";
    pub const DESCRIPTION: &str = "Grant experience to the specified Ids";
    pub const NAME_LOCALIZED: &str = "conceder";
    pub const DESCRIPTION_LOCALIZED: &str = "Conceder experiência para os Ids especificados";

    pub async fn run<'a>(
        bot: &impl AsBot,
        args: &[ResolvedOption<'_>],
    ) -> Result<Vec<Response<'a>>> {
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
            let new_content = if invalid_id_tags.len() > 1 {
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

            return response::simple_send_and_delete(new_content);
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
                let mut new_content = f!(
                    "**{}** obteve **{}** de experiência e subiu de nível [**{}** ➜ **{}**]",
                    id.name,
                    mark_thousands(value),
                    mark_thousands(previous_lvl),
                    mark_thousands(id.lvl)
                );
                new_content.push_str(&f!(
                    "Pode distribuir **{}** pontos nos atributos.",
                    mark_thousands(id.points_to_distribute)
                ));
                new_content.push_str(&f!(
                    "Falta **{}** de experiência para o próximo nível.",
                    mark_thousands(xp_to_next_level(id.lvl) - id.xp)
                ));

                blueprints.push(ResponseBlueprint::default().set_content(new_content));
            } else {
                blueprints.push(ResponseBlueprint::default().set_content(f!(
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
