use crate::_prelude::*;

pub const NAME: &str = "ping";
pub const DESCRIPTION: &str = "Pong!";
pub const NAME_LOCALIZED: &str = "ping";
pub const DESCRIPTION_LOCALIZED: &str = "Pong!";

pub async fn run<'a>(
    ctx: &Context,
    slash: &CommandInteraction,
) -> Result<Vec<Response<'a>>> {
    let initial_point = Instant::now();

    slash
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new()),
        )
        .await?;

    let elapsed = initial_point.elapsed().as_millis();

    slash
        .edit_response(
            &ctx.http,
            EditInteractionResponse::new().content(f!("Latência atual: {elapsed}ms")),
        )
        .await?;

    Ok(vec![])
}
