use crate::_prelude::*;
use al_azif_prefix::commands::*;

pub async fn run_command(bot: &impl AsBot, ctx: &Context, msg: &Message) -> Result<()> {
    let mut args = msg.content[PREFIX.len()..].split_ascii_whitespace();

    let cmd_name = args.next().unwrap().to_lowercase();

    let execution_result = match cmd_name.as_str() {
        name if attack::ALIASES.contains(&name) => {
            attack::run_command(bot, msg, &args.collect::<Vec<&str>>()).await
        }
        name if block::ALIASES.contains(&name) => block::run_command(bot, msg).await,
        name if rise::ALIASES.contains(&name) => rise::run_command(bot, msg).await,
        _ => return Ok(()),
    };

    let responses = execution_result?;

    perform_response_responses(ctx, msg, responses).await
}

pub async fn perform_response_responses<'a>(
    ctx: &Context,
    msg: &Message,
    responses: Vec<Response<'a>>,
) -> Result<()> {
    let mut delete_original = false;
    let mut msgs_to_delete = Vec::new();

    for response in responses {
        match response {
            Response::DeleteOriginal => delete_original = true,
            Response::Send { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    continue;
                };

                msg.channel_id
                    .send_message(
                        &ctx.http,
                        first_blueprint.create_message().reference_message(msg),
                    )
                    .await?;

                for blueprint in blueprints.iter().skip(1) {
                    msg.channel_id
                        .send_message(&ctx.http, blueprint.create_message())
                        .await?;
                }
            }
            Response::SendAndDelete { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    continue;
                };

                msgs_to_delete.push(
                    msg.channel_id
                        .send_message(
                            &ctx.http,
                            first_blueprint.create_message().reference_message(msg),
                        )
                        .await?,
                );

                for blueprint in blueprints.iter().skip(1) {
                    msgs_to_delete.push(
                        msg.channel_id
                            .send_message(&ctx.http, blueprint.create_message())
                            .await?,
                    );
                }
            }
            Response::SendEphemeral { .. } => (),
            Response::SendLoose { blueprints } => {
                for blueprint in blueprints {
                    msg.channel_id
                        .send_message(&ctx.http, blueprint.create_message())
                        .await?;
                }
            }
            Response::SendLooseAndDelete { blueprints } => {
                for blueprint in blueprints {
                    msgs_to_delete.push(
                        msg.channel_id
                            .send_message(&ctx.http, blueprint.create_message())
                            .await?,
                    );
                }
            }
            Response::Update { .. } => (),
            Response::UpdateDelayless { .. } => (),
        }
    }

    tokio::time::sleep(RESPONSE_TIMEOUT).await;

    for msg_to_delete in msgs_to_delete {
        msg_to_delete.delete(&ctx.http, None).await?;

        tokio::time::sleep(RESPONSE_INTERVAL).await;
    }

    if delete_original {
        msg.delete(&ctx.http, None).await?;
    }

    Ok(())
}
