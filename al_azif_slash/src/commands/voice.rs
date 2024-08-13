use crate::_prelude::*;

pub const TAG: &str = "voice";
pub const DESCRIPTION: &str = "Voice commands";
pub const TAG_PT: &str = "voz";
pub const DESCRIPTION_PT: &str = "Comandos de voz";

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new(TAG)
        .description(DESCRIPTION)
        .name_localized("pt-BR", TAG_PT)
        .description_localized("pt-BR", DESCRIPTION_PT)
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommandGroup, play::TAG, play::DESCRIPTION)
                .name_localized("pt-BR", play::TAG_PT)
                .description_localized("pt-BR", play::DESCRIPTION_PT)
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::SubCommand, play::youtube::TAG, play::youtube::DESCRIPTION)
                        .name_localized("pt-BR", play::youtube::TAG_PT)
                        .description_localized("pt-BR", play::youtube::DESCRIPTION_PT)
                        .add_sub_option(
                            CreateCommandOption::new(CommandOptionType::String, "url", "The Youtube URL to play")
                                .description_localized("pt-BR", "A URL do Youtube para tocar")
                                .required(true),
                        ),
                ),
        )
}

pub mod play {
    use super::*;

    pub const TAG: &str = "play";
    pub const DESCRIPTION: &str = "Play a song";
    pub const TAG_PT: &str = "reproduzir";
    pub const DESCRIPTION_PT: &str = "Tocar uma música";

    pub mod youtube {
        use super::*;

        pub const TAG: &str = "youtube";
        pub const DESCRIPTION: &str = "Play a song from YouTube";
        pub const TAG_PT: &str = "youtube";
        pub const DESCRIPTION_PT: &str = "Tocar uma música do YouTube";

        pub async fn run_slash(bot: &impl AsBot, ctx: &Context, slash: &CommandInteraction, url: &str) -> Result<Responses> {
            slash
                .create_response(&ctx.http, CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new()))
                .await
                .map_err(SlashError::CouldNotCreateInteractionResponse)?;

            if voice::join_main_voice_channel(bot).await.is_err() {
                return Err(SlashError::Anticipated(ErrorResponse::send(vec![ResponseBlueprint::with_content(
                    lang_diff!(bot, en: "Could not join the main voice channel.", pt: "Não foi possível entrar no canal de voz principal."),
                )])));
            }

            match voice::play_youtube_video(bot, url).await {
                Ok(()) => Ok(vec![Response::edit_defer(ResponseBlueprint::with_content(lang_diff!(bot,
                    en: "Playing...", pt: "Tocando..."
                )))]),
                Err(CoreError::Voice(VoiceError::YoutubeVideoNotFound)) => {
                    Err(SlashError::Anticipated(ErrorResponse::edit_defer(ResponseBlueprint::with_content(lang_diff!(bot,
                        en: "The Youtube video was not found.", pt: "O vídeo do Youtube não foi encontrado."
                    )))))
                },
                Err(why) => Err(SlashError::Core(why)),
            }
        }
    }
}
