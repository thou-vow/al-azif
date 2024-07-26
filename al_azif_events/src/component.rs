use crate::_prelude::*;

pub async fn run(bot: &impl AsBot, ctx: &Context, comp: &ComponentInteraction) -> Result<()> {
    let mut args = comp.data.custom_id.split(' ');

    match args.next() {
        Some("#slash") => run_slash(bot, ctx, comp, args.collect::<Vec<&str>>()).await,
        Some(_) => run_prefix(bot, ctx, comp, args.collect::<Vec<&str>>()).await,
        None => Err(EventError::EmptyComponentInteractionCustomId),
    }
}

pub async fn run_slash(
    bot: &impl AsBot,
    ctx: &Context,
    comp: &ComponentInteraction,
    args: Vec<&str>,
) -> Result<()> {
    use al_azif_slash::commands::*;

    let execution_result = match args.as_slice() {
        ["id", "distribute", "goto_attributes", id_tag] => {
            id::distribute::goto_attributes::run_component(bot, id_tag).await.map_err(EventError::Slash)
        },
        ["id", "distribute", "goto_incrementors", id_tag, attribute_str] => {
            id::distribute::goto_incrementors::run_component(bot, id_tag, attribute_str)
                .await
                .map_err(EventError::Slash)
        },
        ["id", "distribute", "invest_in", id_tag, attribute_str, selected_value] => {
            id::distribute::invest_in::run_component(
                bot,
                id_tag,
                attribute_str,
                parse_comp_arg!(selected_value, i64)?,
            )
            .await
            .map_err(EventError::Slash)
        },
        _ => Err(EventError::InvalidSlashComponent { custom_id: comp.data.custom_id.clone() }),
    };

    let responses = execution_result?;

    perform_response_responses(ctx, comp, responses).await
}

pub async fn run_prefix(
    bot: &impl AsBot,
    ctx: &Context,
    comp: &ComponentInteraction,
    args: Vec<&str>,
) -> Result<()> {
    use al_azif_prefix::commands::*;

    let execution_result = match args.as_slice() {
        [receive::NAME] => receive::run_component(bot, comp).await.map_err(EventError::Prefix),
        _ => Err(EventError::InvalidPrefixComponent { custom_id: comp.data.custom_id.clone() }),
    };

    let responses = execution_result?;

    perform_response_responses(ctx, comp, responses).await
}
pub async fn perform_response_responses<'a>(
    ctx: &Context,
    comp: &ComponentInteraction,
    responses: Responses<'a>,
) -> Result<()> {
    let mut msgs_to_delete = Vec::new();

    for response in responses {
        match response {
            Response::DeleteOriginal => (),
            Response::Send { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    continue;
                };

                comp.create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(first_blueprint.create_interaction_response_message()),
                )
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

                comp.create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(first_blueprint.create_interaction_response_message()),
                )
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
                    CreateInteractionResponse::Message(
                        blueprint.create_interaction_response_message().ephemeral(true),
                    ),
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
                }

                tokio::time::sleep(RESPONSE_INTERVAL).await;
            },
            Response::SendLooseAndDelete { blueprints } => {
                for blueprint in blueprints {
                    msgs_to_delete.push(
                        comp.channel_id
                            .send_message(&ctx.http, blueprint.create_message())
                            .await
                            .map_err(EventError::CouldNotSendMessage)?,
                    );
                }
            },
            Response::Update { blueprint } => {
                comp.create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(blueprint.create_interaction_response_message()),
                )
                .await
                .map_err(EventError::CouldNotCreateInteractionResponse)?;

                tokio::time::sleep(RESPONSE_INTERVAL).await;
            },
            Response::UpdateDelayless { blueprint } => {
                comp.create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(blueprint.create_interaction_response_message()),
                )
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
