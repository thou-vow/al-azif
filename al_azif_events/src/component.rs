use crate::_prelude::*;

pub async fn run(bot: &impl AsBot, ctx: &Context, comp: &ComponentInteraction) -> Result<()> {
    let mut args = comp.data.custom_id.split(' ');

    let Some(slash_or_name) = args.next() else {
        return Err(EventError::EmptyComponentInteractionCustomId);
    };

    match slash_or_name {
        "#slash" => run_slash(bot, ctx, comp, args.collect::<Vec<&str>>()).await,
        name => run_prefix(bot, ctx, comp, name, args).await,
    }
}

pub async fn run_slash(bot: &impl AsBot, ctx: &Context, comp: &ComponentInteraction, args: Vec<&str>) -> Result<()> {
    use al_azif_slash::commands::*;

    let execution_result = match args.as_slice() {
        ["id", "distribute", "goto_attributes", id_tag] => {
            id::distribute::goto_attributes::run_component(bot, id_tag).await.map_err(EventError::Slash)
        },
        ["id", "distribute", "goto_incrementors", id_tag, attribute_str] => {
            id::distribute::goto_incrementors::run_component(bot, id_tag, attribute_str).await.map_err(EventError::Slash)
        },
        ["id", "distribute", "invest_in", id_tag, attribute_str, selected_value] => {
            id::distribute::invest_in::run_component(bot, id_tag, attribute_str, parse_comp_arg!(selected_value, i64)?)
                .await
                .map_err(EventError::Slash)
        },
        _ => Err(EventError::InvalidSlashComponent { custom_id: comp.data.custom_id.clone() }),
    };

    let responses = execution_result?;

    perform_responses(ctx, comp, responses).await
}

pub async fn run_prefix(bot: &impl AsBot, ctx: &Context, comp: &ComponentInteraction, name: &str, args: Split<'_, char>) -> Result<()> {
    use al_azif_prefix::commands::*;

    let args = args.collect();

    let execution_result = match name {
        receive::NAME => receive::run_component(bot, comp, args).await.map_err(EventError::Prefix),
        _ => Err(EventError::InvalidPrefixComponent { custom_id: comp.data.custom_id.clone() }),
    };

    let responses = match execution_result {
        Ok(responses) => responses,
        Err(EventError::Prefix(PrefixError::Expected(blueprints))) => {
            vec![Response::send_and_delete(blueprints)]
        },
        Err(err) => return Err(err),
    };

    perform_responses(ctx, comp, responses).await
}

pub async fn perform_responses(ctx: &Context, comp: &ComponentInteraction, responses: Responses) -> Result<()> {
    let mut msgs_to_delete = Vec::new();

    for response in responses {
        match response {
            Response::DeleteOriginal => (),
            Response::Send { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    continue;
                };

                comp.create_response(&ctx.http, CreateInteractionResponse::Message(first_blueprint.create_interaction_response_message()))
                    .await
                    .map_err(EventError::CouldNotCreateInteractionResponse)?;

                tokio::time::sleep(RESPONSE_INTERVAL).await;

                for blueprint in blueprints.iter().skip(1) {
                    comp.channel_id
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

                comp.create_response(&ctx.http, CreateInteractionResponse::Message(first_blueprint.create_interaction_response_message()))
                    .await
                    .map_err(EventError::CouldNotCreateInteractionResponse)?;
                msgs_to_delete.push(
                    ctx.http
                        .get_original_interaction_response(&comp.token)
                        .await
                        .map_err(EventError::CouldNotGetOriginalInteractionResponse)?,
                );

                tokio::time::sleep(RESPONSE_INTERVAL).await;

                for blueprint in blueprints.iter().skip(1) {
                    msgs_to_delete.push(
                        comp.channel_id
                            .send_message(&ctx.http, blueprint.create_message())
                            .await
                            .map_err(EventError::CouldNotSendMessage)?,
                    );

                    tokio::time::sleep(RESPONSE_INTERVAL).await;
                }
            },
            Response::SendEphemeral { blueprint } => {
                comp.create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(blueprint.create_interaction_response_message().ephemeral(true)),
                )
                .await
                .map_err(EventError::CouldNotCreateInteractionResponse)?;
            },
            Response::SendLoose { blueprints } => {
                for blueprint in blueprints {
                    comp.channel_id
                        .send_message(&ctx.http, blueprint.create_message())
                        .await
                        .map_err(EventError::CouldNotSendMessage)?;

                    tokio::time::sleep(RESPONSE_INTERVAL).await;
                }
            },
            Response::SendLooseAndDelete { blueprints } => {
                for blueprint in blueprints {
                    msgs_to_delete.push(
                        comp.channel_id
                            .send_message(&ctx.http, blueprint.create_message())
                            .await
                            .map_err(EventError::CouldNotSendMessage)?,
                    );

                    tokio::time::sleep(RESPONSE_INTERVAL).await;
                }
            },
            Response::Update { blueprint } => {
                comp.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(blueprint.create_interaction_response_message()))
                    .await
                    .map_err(EventError::CouldNotCreateInteractionResponse)?;

                tokio::time::sleep(RESPONSE_INTERVAL).await;
            },
            Response::UpdateDelayless { blueprint } => {
                comp.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(blueprint.create_interaction_response_message()))
                    .await
                    .map_err(EventError::CouldNotCreateInteractionResponse)?;
            },
        }
    }

    tokio::time::sleep(RESPONSE_TIMEOUT).await;

    for msg_to_delete in msgs_to_delete {
        msg_to_delete.delete(&ctx.http, None).await.map_err(EventError::CouldNotDeleteMessage)?;

        tokio::time::sleep(RESPONSE_INTERVAL).await;
    }

    Ok(())
}
