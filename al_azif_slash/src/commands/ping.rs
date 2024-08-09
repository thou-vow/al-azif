use crate::_prelude::*;

pub const TAG: &str = "ping";
pub const DESCRIPTION: &str = "Pong!";
pub const TAG_PT: &str = "ping";
pub const DESCRIPTION_PT: &str = "Pong!";

pub async fn run_slash(ctx: &Context, slash: &CommandInteraction) -> Result<Vec<Response>> {
    let initial_point = Instant::now();

    slash
        .create_response(&ctx.http, CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new()))
        .await
        .map_err(SlashError::CouldNotCreateInteractionResponse)?;

    let elapsed = initial_point.elapsed().as_millis();

    slash
        .edit_response(&ctx.http, EditInteractionResponse::new().content(f!("Latência atual: {elapsed}ms")))
        .await
        .map_err(SlashError::CouldNotEditInteractionResponse)?;

    Ok(vec![])
}
