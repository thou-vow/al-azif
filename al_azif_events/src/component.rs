use crate::_prelude::*;

pub async fn run(bot: &impl AsBot, ctx: &Context, comp: &ComponentInteraction) -> Result<()> {
    let mut args = comp.data.custom_id.split(' ');

    match args.next().unwrap() {
        "#slash" => run_slash(bot, ctx, comp, &args.collect::<Vec<&str>>()).await,
        "unclassified" => run_unclassified(bot, ctx, comp, &args.collect::<Vec<&str>>()).await,
        invalid => unreachable!("Unknown component args[0]: {invalid}"),
    }
}

pub async fn run_slash(bot: &impl AsBot, ctx: &Context, comp: &ComponentInteraction, args: &[&str]) -> Result<()> {
    use al_azif_slash::commands::*;

    let execution_result = match args[0] {
        "id" => id::run_component(bot, comp, &args[1 ..]).await,
        _ => return Ok(()),
    };

    let responses = execution_result?;

    perform_response_responses(ctx, comp, responses).await
}

pub async fn run_unclassified(
    bot: &impl AsBot,
    ctx: &Context,
    comp: &ComponentInteraction,
    args: &[&str],
) -> Result<()> {
    use unclassified::*;

    let execution_result = match args[0] {
        "receive" => receive::run_component(bot, comp, args[1].parse()?).await,
        _ => return Ok(()),
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
                .await?;

                tokio::time::sleep(RESPONSE_INTERVAL).await;

                for blueprint in blueprints.iter().skip(1) {
                    comp.channel_id.send_message(&ctx.http, blueprint.create_message()).await?;

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
                .await?;
                msgs_to_delete.push(ctx.http.get_original_interaction_response(&comp.token).await?);

                tokio::time::sleep(RESPONSE_INTERVAL).await;

                for blueprint in blueprints.iter().skip(1) {
                    msgs_to_delete
                        .push(comp.channel_id.send_message(&ctx.http, blueprint.create_message()).await?);

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
                .await?;
            },
            Response::SendLoose { blueprints } => {
                for blueprint in blueprints {
                    comp.channel_id.send_message(&ctx.http, blueprint.create_message()).await?;
                }

                tokio::time::sleep(RESPONSE_INTERVAL).await;
            },
            Response::SendLooseAndDelete { blueprints } => {
                for blueprint in blueprints {
                    msgs_to_delete
                        .push(comp.channel_id.send_message(&ctx.http, blueprint.create_message()).await?);
                }
            },
            Response::Update { blueprint } => {
                comp.create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(blueprint.create_interaction_response_message()),
                )
                .await?;

                tokio::time::sleep(RESPONSE_INTERVAL).await;
            },
            Response::UpdateDelayless { blueprint } => {
                comp.create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(blueprint.create_interaction_response_message()),
                )
                .await?;
            },
        }
    }

    tokio::time::sleep(RESPONSE_TIMEOUT).await;

    for msg_to_delete in msgs_to_delete {
        msg_to_delete.delete(&ctx.http, None).await?;

        tokio::time::sleep(RESPONSE_INTERVAL).await;
    }

    Ok(())
}
