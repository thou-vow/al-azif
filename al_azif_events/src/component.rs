use crate::prelude::*;

pub async fn run(bot: &impl AsBot, ctx: &Context, comp: &ComponentInteraction) -> Result<()> {
    let mut args = comp.data.custom_id.split(' ');
            
    match args.next().unwrap() {
        "prefix" => run_prefix(bot, ctx, comp, &args.collect::<Box<[&str]>>()).await,
        "slash" => run_slash(bot, ctx, comp, &args.collect::<Box<[&str]>>()).await,
        invalid => unreachable!("Unknown component args[0]: {invalid}")
    }
}

pub async fn run_prefix(bot: &impl AsBot, ctx: &Context, comp: &ComponentInteraction, args: &[&str]) -> Result<()> {
    use al_azif_prefix::commands::*;

    let execution_result = match args[0] {
        "dodge" => dodge::run_component(bot, comp, &args[1..]).await,
        _ => return Ok(()),
    };

    let models = execution_result?;

    perform_response_models(ctx, comp, models).await
}

pub async fn run_slash(bot: &impl AsBot, ctx: &Context, comp: &ComponentInteraction, args: &[&str]) -> Result<()> {
    use al_azif_slash::commands::*;

    let execution_result = match args[0] {
        "id" => id::run_component(bot, comp, &args[1..]).await,
        _ => return Ok(()),
    };

    let models = execution_result?;

    perform_response_models(ctx, comp, models).await
}

pub async fn perform_response_models(ctx: &Context, comp: &ComponentInteraction, models: Vec<ResponseModel>) -> Result<()> {
    for model in models {
        match model {
            ResponseModel::Send { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    return Ok(());
                };

                comp.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::from(first_blueprint.clone())
                )).await?;

                tokio::time::sleep(RESPONSE_INTERVAL).await;

                for blueprint in blueprints.iter().skip(1) {
                    comp.channel_id.send_message(&ctx.http,
                        CreateMessage::from(blueprint.clone())
                    ).await?;

                    tokio::time::sleep(RESPONSE_INTERVAL).await;
                }
            },
            ResponseModel::SendEphemeral { blueprint  } => {
                comp.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::from(blueprint.clone())
                        .ephemeral(true)
                )).await?;
            },
            ResponseModel::SendLoose { blueprints } => {
                for blueprint in blueprints {
                    comp.channel_id.send_message(&ctx.http,
                        CreateMessage::from(blueprint.clone())
                    ).await?;
                }

                tokio::time::sleep(RESPONSE_INTERVAL).await;
            },
            ResponseModel::Update { blueprint } => {
                comp.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::from(blueprint.clone())
                )).await?;

                tokio::time::sleep(RESPONSE_INTERVAL).await;
            },
        }
    }

    Ok(())
}