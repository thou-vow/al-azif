use crate::_prelude::*;

pub const NAME: &str = "help";
pub const DESCRIPTION: &str = "Shows available commands";
pub const NAME_PT: &str = "ajuda";
pub const DESCRIPTION_PT: &str = "Mostra os comandos disponíveis";

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new(NAME)
        .description(DESCRIPTION)
        .name_localized("pt-BR", NAME_PT)
        .description_localized("pt-BR", DESCRIPTION_PT)
}

pub async fn run_slash<'a>() -> Result<Vec<Response<'a>>> { Ok(vec![Response::send(vec![ResponseBlueprint::new().add_embed(embed_1())])]) }

fn embed_1() -> CreateEmbed<'static> {
    use crate::commands::*;

    CreateEmbed::new()
        .title("Lista de Comandos")
        .colour(Colour::from_rgb(0, 255, 255))
        .field("", fc!("**/{NAME_PT}**: {DESCRIPTION_PT}"), true)
        .field(
            "",
            fc!(
                "**/{} {}**: {}\n**/{} {}**: {}\n**/{} {}**: {}",
                battle::NAME_PT,
                battle::start::NAME_PT,
                battle::start::DESCRIPTION_PT,
                battle::NAME_PT,
                battle::join::NAME_PT,
                battle::join::DESCRIPTION_PT,
                battle::NAME_PT,
                battle::end::NAME_PT,
                battle::end::DESCRIPTION_PT,
            ),
            true,
        )
        .field("", fc!("**/{} {}**: {}", exp::NAME_PT, exp::bestow::NAME_PT, exp::bestow::DESCRIPTION_PT), true)
        .field("", fc!("**/{} {}**: {}", id::NAME_PT, id::distribute::NAME_PT, id::distribute::DESCRIPTION_PT), true)
        .field("", fc!("**/{}**: {}", ping::NAME_PT, ping::DESCRIPTION_PT), true)
}
