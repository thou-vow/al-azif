use crate::_prelude::*;

pub async fn register_commands(bot: &impl AsBot, ctx: &Context) -> Result<()> {
    use al_azif_slash::commands::*;

    bot.get_main_guild()
        .set_commands(&ctx.http, &[battle::register(), exp::register(), help::register(), id::register()])
        .await?;

    Ok(())
}

pub async fn run_command(bot: &impl AsBot, ctx: &Context, slash: &CommandInteraction) -> Result<()> {
    use al_azif_slash::commands::*;
    use ResolvedOption as RO;
    use ResolvedValue as RV;

    let args = slash.data.options();

    let execution_result = match (slash.data.name.as_str(), args.as_slice()) {
        (battle::NAME, [RO { name: battle::end::NAME, value: RV::SubCommand(_), .. }, ..]) => {
            battle::end::run(bot, slash).await
        },
        (battle::NAME, [RO { name: battle::join::NAME, value: RV::SubCommand(sub_args), .. }, ..]) => {
            battle::join::run(bot, slash, sub_args).await
        },
        (battle::NAME, [RO { name: battle::start::NAME, value: RV::SubCommand(sub_args), .. }, ..]) => {
            battle::start::run(bot, slash, sub_args).await
        },
        (exp::NAME, [RO { name: exp::bestow::NAME, value: RV::SubCommand(sub_args), .. }, ..]) => {
            exp::bestow::run(bot, sub_args).await
        },
        (help::NAME, _) => help::run().await,
        (id::NAME, [RO { name: id::distribute::NAME, value: RV::SubCommand(sub_args), .. }, ..]) => {
            id::distribute::run(bot, sub_args).await
        },
        (ping::NAME, _) => ping::run(ctx, slash).await,
        _ => return Err(anyhow::anyhow!("Unknown command")),
    };

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
                        CreateInteractionResponse::Message(first_blueprint.create_interaction_response_message()),
                    )
                    .await?;

                for blueprint in blueprints.iter().skip(1) {
                    slash.channel_id.send_message(&ctx.http, blueprint.create_message()).await?;
                }
            },
            Response::SendAndDelete { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    continue;
                };

                slash
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(first_blueprint.create_interaction_response_message()),
                    )
                    .await?;
                msgs_to_delete.push(ctx.http.get_original_interaction_response(&slash.token).await?);

                for blueprint in blueprints.iter().skip(1) {
                    msgs_to_delete
                        .push(slash.channel_id.send_message(&ctx.http, blueprint.create_message()).await?);
                }
            },
            Response::SendEphemeral { blueprint } => {
                slash
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            blueprint.create_interaction_response_message().ephemeral(true),
                        ),
                    )
                    .await?;
            },
            Response::SendLoose { blueprints } => {
                for blueprint in blueprints {
                    slash.channel_id.send_message(&ctx.http, blueprint.create_message()).await?;
                }
            },
            Response::SendLooseAndDelete { blueprints } => {
                for blueprint in blueprints {
                    msgs_to_delete
                        .push(slash.channel_id.send_message(&ctx.http, blueprint.create_message()).await?);
                }
            },
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
