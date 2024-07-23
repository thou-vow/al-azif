use crate::_prelude::*;

pub async fn register_commands(bot: &impl AsBot, ctx: &Context) -> Result<()> {
    bot.get_main_guild()
        .set_commands(
            &ctx.http,
            &SlashCommand::registers(),
        )
        .await?;

    Ok(())
}

pub async fn run_command(
    bot: &impl AsBot,
    ctx: &Context,
    slash: &CommandInteraction,
) -> Result<()> {
    let args = slash.data.options();

    let Some(cmd) = SlashCommand::from_name_and_args(slash.data.name.as_str(), &args) else {
        return Ok(());
    };
    
    let execution_result = cmd.run(bot, ctx, slash).await;

    let responses = execution_result?;

    perform_response_responses(ctx, slash, responses).await
}

pub async fn perform_response_responses<'a>(
    ctx: &Context,
    slash: &CommandInteraction,
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

                slash
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            first_blueprint.create_interaction_response_message(),
                        ),
                    )
                    .await?;

                for blueprint in blueprints.iter().skip(1) {
                    slash
                        .channel_id
                        .send_message(&ctx.http, blueprint.create_message())
                        .await?;
                }
            }
            Response::SendAndDelete { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    continue;
                };

                slash
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            first_blueprint.create_interaction_response_message(),
                        ),
                    )
                    .await?;
                msgs_to_delete.push(
                    ctx.http
                        .get_original_interaction_response(&slash.token)
                        .await?,
                );

                for blueprint in blueprints.iter().skip(1) {
                    msgs_to_delete.push(
                        slash
                            .channel_id
                            .send_message(&ctx.http, blueprint.create_message())
                            .await?,
                    );
                }
            }
            Response::SendEphemeral { blueprint } => {
                slash
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            blueprint
                                .create_interaction_response_message()
                                .ephemeral(true),
                        ),
                    )
                    .await?;
            }
            Response::SendLoose { blueprints } => {
                for blueprint in blueprints {
                    slash
                        .channel_id
                        .send_message(&ctx.http, blueprint.create_message())
                        .await?;
                }
            }
            Response::SendLooseAndDelete { blueprints } => {
                for blueprint in blueprints {
                    msgs_to_delete.push(
                        slash
                            .channel_id
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

    Ok(())
}
