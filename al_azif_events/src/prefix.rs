use crate::_prelude::*;

pub async fn run(bot: &impl AsBot, ctx: &Context, msg: &Message) -> Result<()> {
    use al_azif_prefix::commands::*;

    let mut args = msg.content[PREFIX.len() ..].split_ascii_whitespace();

    let Some(name) = args.next() else {
        // TODO: when the message has just the prefix
        return Ok(());
    };

    let execution_result = match name.to_lowercase().as_str() {
        attack::NAME | attack::NAME_PT => {
            attack::run(bot, msg, &args.collect::<Vec<&str>>()).await.map_err(EventError::Prefix)
        },
        block::NAME | block::NAME_PT => block::run(bot, msg).await.map_err(EventError::Prefix),
        /*receive::NAME | receive::NAME_PT => receive::run(bot, msg).await.map_err(EventError::Prefix),*/
        rise::NAME | rise::NAME_PT => rise::run(bot, msg).await.map_err(EventError::Prefix),
        _ => return Err(EventError::InvalidPrefixCommand { name: FixedString::from_str_trunc(name) }),
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
                    .send_message(&ctx.http, first_blueprint.create_message().reference_message(msg))
                    .await
                    .map_err(EventError::CouldNotSendMessage)?;

                for blueprint in blueprints.iter().skip(1) {
                    msg.channel_id
                        .send_message(&ctx.http, blueprint.create_message())
                        .await
                        .map_err(EventError::CouldNotSendMessage)?;
                }
            },
            Response::SendAndDelete { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    continue;
                };

                msgs_to_delete.push(
                    msg.channel_id
                        .send_message(&ctx.http, first_blueprint.create_message().reference_message(msg))
                        .await
                        .map_err(EventError::CouldNotSendMessage)?,
                );

                for blueprint in blueprints.iter().skip(1) {
                    msgs_to_delete.push(
                        msg.channel_id
                            .send_message(&ctx.http, blueprint.create_message())
                            .await
                            .map_err(EventError::CouldNotSendMessage)?,
                    );
                }
            },
            Response::SendEphemeral { .. } => (),
            Response::SendLoose { blueprints } => {
                for blueprint in blueprints {
                    msg.channel_id
                        .send_message(&ctx.http, blueprint.create_message())
                        .await
                        .map_err(EventError::CouldNotSendMessage)?;
                }
            },
            Response::SendLooseAndDelete { blueprints } => {
                for blueprint in blueprints {
                    msgs_to_delete.push(
                        msg.channel_id
                            .send_message(&ctx.http, blueprint.create_message())
                            .await
                            .map_err(EventError::CouldNotSendMessage)?,
                    );
                }
            },
            Response::Update { .. } => (),
            Response::UpdateDelayless { .. } => (),
        }
    }

    tokio::time::sleep(RESPONSE_TIMEOUT).await;

    for msg_to_delete in msgs_to_delete {
        msg_to_delete.delete(&ctx.http, None).await.map_err(EventError::CouldNotDeleteMessage)?;

        tokio::time::sleep(RESPONSE_INTERVAL).await;
    }

    if delete_original {
        msg.delete(&ctx.http, None).await.map_err(EventError::CouldNotDeleteMessage)?;
    }

    Ok(())
}
