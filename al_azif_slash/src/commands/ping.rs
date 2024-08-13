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

    Ok(vec![Response::edit_defer(ResponseBlueprint::with_content(f!("Latência atual: {elapsed}ms")))])
}
