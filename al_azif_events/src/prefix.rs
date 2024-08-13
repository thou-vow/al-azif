use crate::_prelude::*;

pub async fn run(bot: &impl AsBot, ctx: &Context, msg: &Message) -> Result<()> {
    use al_azif_prefix::commands::*;

    let mut args = msg.content[PREFIX.len() ..].split_ascii_whitespace();

    let Some(name) = args.next() else {
        // TODO: when the message has just the prefix
        return Ok(());
    };

    let args = args.collect();

    let execution_result = match name.to_lowercase().as_str() {
        attack::TAG | attack::TAG_PT => attack::run_prefix(bot, msg, args).await.map_err(EventError::Prefix),
        block::TAG | block::TAG_PT => block::run_prefix(bot, msg, args).await.map_err(EventError::Prefix),
        heal::TAG | heal::TAG_PT => heal::run_prefix(bot, msg, args).await.map_err(EventError::Prefix),
        miracle::TAG | miracle::TAG_PT => miracle::run_prefix(bot, msg, args).await.map_err(EventError::Prefix),
        receive::TAG | receive::TAG_PT => receive::run_prefix(bot, msg, args).await.map_err(EventError::Prefix),
        rise::TAG | rise::TAG_PT => rise::run_prefix(bot, msg, args).await.map_err(EventError::Prefix),
        vital_trill::TAG | vital_trill::TAG_PT => vital_trill::run_prefix(bot, msg, args).await.map_err(EventError::Prefix),
        _ => return Err(EventError::InvalidPrefixCommand { name: FixedString::from_str_trunc(name) }),
    };

    let responses = match execution_result {
        Ok(responses) => responses,
        Err(EventError::Prefix(PrefixError::Anticipated(ErrorResponse::Send { blueprints }))) => {
            vec![Response::delete_original(), Response::send_and_delete(blueprints)]
        },
        Err(EventError::Prefix(PrefixError::Anticipated(ErrorResponse::SendLoose { blueprints }))) => {
            vec![Response::delete_original(), Response::send_loose_and_delete(blueprints)]
        },
        Err(err) => return Err(err),
    };

    perform_responses(ctx, msg, responses).await
}

pub async fn perform_responses(ctx: &Context, msg: &Message, responses: Vec<Response>) -> Result<()> {
    let mut delete_original = false;
    let mut msgs_to_delete = Vec::new();

    for response in responses {
        match response {
            Response::DeleteOriginal => delete_original = true,
            Response::EditDefer { .. } => (),
            Response::EditDeferAndDelete { .. } => (),
            Response::Send { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    continue;
                };

                msg.channel_id
                    .send_message(&ctx.http, first_blueprint.create_message().reference_message(msg))
                    .await
                    .map_err(EventError::CouldNotSendMessage)?;

                tokio::time::sleep(RESPONSE_INTERVAL).await;

                for blueprint in blueprints.iter().skip(1) {
                    msg.channel_id
                        .send_message(&ctx.http, blueprint.create_message())
                        .await
                        .map_err(EventError::CouldNotSendMessage)?;

                    tokio::time::sleep(RESPONSE_INTERVAL).await;
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

                tokio::time::sleep(RESPONSE_INTERVAL).await;

                for blueprint in blueprints.iter().skip(1) {
                    msgs_to_delete.push(
                        msg.channel_id
                            .send_message(&ctx.http, blueprint.create_message())
                            .await
                            .map_err(EventError::CouldNotSendMessage)?,
                    );

                    tokio::time::sleep(RESPONSE_INTERVAL).await;
                }
            },
            Response::SendEphemeral { .. } => (),
            Response::SendLoose { blueprints } => {
                for blueprint in blueprints {
                    msg.channel_id
                        .send_message(&ctx.http, blueprint.create_message())
                        .await
                        .map_err(EventError::CouldNotSendMessage)?;

                    tokio::time::sleep(RESPONSE_INTERVAL).await;
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

                    tokio::time::sleep(RESPONSE_INTERVAL).await;
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
