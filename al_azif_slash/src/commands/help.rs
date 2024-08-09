use crate::_prelude::*;

pub const TAG: &str = "help";
pub const DESCRIPTION: &str = "Shows available commands";
pub const TAG_PT: &str = "ajuda";
pub const DESCRIPTION_PT: &str = "Mostra os comandos disponíveis";

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new(TAG)
        .description(DESCRIPTION)
        .name_localized("pt-BR", TAG_PT)
        .description_localized("pt-BR", DESCRIPTION_PT)
}

pub async fn run_slash() -> Result<Vec<Response>> { Ok(vec![Response::send(vec![ResponseBlueprint::new().add_embed(embed_1())])]) }

fn embed_1() -> CreateEmbed<'static> {
    use crate::commands::*;

    CreateEmbed::new()
        .title("Lista de Comandos")
        .colour(Colour::from_rgb(0, 255, 255))
        .field("", fc!("**/{TAG_PT}**: {DESCRIPTION_PT}"), true)
        .field(
            "",
            fc!(
                "**/{} {}**: {}\n**/{} {}**: {}\n**/{} {}**: {}",
                battle::TAG_PT,
                battle::start::TAG_PT,
                battle::start::DESCRIPTION_PT,
                battle::TAG_PT,
                battle::join::TAG_PT,
                battle::join::DESCRIPTION_PT,
                battle::TAG_PT,
                battle::end::TAG_PT,
                battle::end::DESCRIPTION_PT,
            ),
            true,
        )
        .field("", fc!("**/{} {}**: {}", exp::TAG_PT, exp::bestow::TAG_PT, exp::bestow::DESCRIPTION_PT), true)
        .field("", fc!("**/{} {}**: {}", id::TAG_PT, id::distribute::TAG_PT, id::distribute::DESCRIPTION_PT), true)
        .field("", fc!("**/{}**: {}", ping::TAG_PT, ping::DESCRIPTION_PT), true)
}
