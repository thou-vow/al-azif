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
    use crate::commands::*;

    fn add_line(cmd: &SlashCommand, mut current_section: Vec<String>)  -> Vec<String> {
        match cmd {
            SlashCommand::Battle(sub) => {
                current_section.push(f!("**/{} {}**: {}", cmd.get_name_localized(), sub.get_name_localized(), sub.get_description_localized()))
            },
            SlashCommand::Exp(sub) => {
                current_section.push(f!("**/{} {}**: {}", cmd.get_name_localized(), sub.get_name_localized(), sub.get_description_localized()))
            }
            SlashCommand::Help => current_section.push(f!("**/{}**: {}", cmd.get_name_localized(), cmd.get_description_localized())),
            SlashCommand::Id(sub) => {
                current_section.push(f!("**/{} {}**: {}", cmd.get_name_localized(), sub.get_name_localized(), sub.get_description_localized()))
            },
            SlashCommand::Ping => current_section.push(f!("**/{}**: {}", cmd.get_name_localized(), cmd.get_description_localized())),
        }

        current_section
    }

    let all_cmds = SlashCommand::all_localized_order();

    let first_cmd= all_cmds.first().unwrap();
    let mut current_section = Vec::new();
    current_section = add_line(first_cmd, current_section);

    let mut prev_cmd_name = first_cmd.get_name();
    let mut all_sections = Vec::new();

    for cmd in all_cmds.into_iter().skip(1) {
        if cmd.get_name() != prev_cmd_name {
            all_sections.push(current_section);
            current_section = Vec::new();
            prev_cmd_name = cmd.get_name();
        }
        current_section = add_line(&cmd, current_section);
    }
    all_sections.push(current_section);


    let mut new_embed = CreateEmbed::new()
        .title("Lista de Comandos")
        .colour(Colour::from_rgb(0, 255, 255));

    for section in all_sections {
        let joined = section.join("\n");
        new_embed = new_embed.field("", joined, true);
    }

    Ok(vec![Response::send(vec![
        ResponseBlueprint::default().add_embed(new_embed)
    ])])
}
