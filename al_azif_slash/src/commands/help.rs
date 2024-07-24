use crate::_prelude::*;

pub const NAME: &str = "help";
pub const DESCRIPTION: &str = "Shows available commands";
pub const NAME_LOCALIZED: &str = "ajuda";
pub const DESCRIPTION_LOCALIZED: &str = "Mostra os comandos disponíveis";

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new(NAME)
        .description(DESCRIPTION)
        .name_localized("pt-BR", NAME_LOCALIZED)
        .description_localized("pt-BR", DESCRIPTION_LOCALIZED)
}

pub async fn run<'a>() -> Result<Vec<Response<'a>>> {
    Ok(vec![Response::send(vec![ResponseBlueprint::new().add_embed(embed_1())])])
}

fn embed_1() -> CreateEmbed<'static> {
    use crate::commands::*;

    CreateEmbed::new()
        .title("Lista de Comandos")
        .colour(Colour::from_rgb(0, 255, 255))
        .field("", fc!("**/{NAME_LOCALIZED}**: {DESCRIPTION_LOCALIZED}"), true)
        .field(
            "",
            fc!(
                "**/{} {}**: {}\n**/{} {}**: {}\n**/{} {}**: {}",
                battle::NAME_LOCALIZED,
                battle::start::NAME_LOCALIZED,
                battle::start::DESCRIPTION_LOCALIZED,
                battle::NAME_LOCALIZED,
                battle::join::NAME_LOCALIZED,
                battle::join::DESCRIPTION_LOCALIZED,
                battle::NAME_LOCALIZED,
                battle::end::NAME_LOCALIZED,
                battle::end::DESCRIPTION_LOCALIZED,
            ),
            true,
        )
        .field(
            "",
            fc!(
                "**/{} {}**: {}",
                exp::NAME_LOCALIZED,
                exp::bestow::NAME_LOCALIZED,
                exp::bestow::DESCRIPTION_LOCALIZED
            ),
            true,
        )
        .field(
            "",
            fc!(
                "**/{} {}**: {}",
                id::NAME_LOCALIZED,
                id::distribute::NAME_LOCALIZED,
                id::distribute::DESCRIPTION_LOCALIZED
            ),
            true,
        )
        .field("", fc!("**/{}**: {}", ping::NAME_LOCALIZED, ping::DESCRIPTION_LOCALIZED), true)
}
