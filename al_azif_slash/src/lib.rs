mod commands {
    pub mod battle;
    pub mod exp;
    pub mod id;
    pub mod ping;
}
pub mod prelude;

use crate::prelude::*;

pub async fn register_commands(bot: &impl AsBot, ctx: &Context) -> Result<()> {
    bot.get_main_guild().set_commands(&ctx.http, &[
        battle::register(),
        exp::register(),
        id::register(),
    ]).await?;

    Ok(())
}

pub async fn run_command(bot: &impl AsBot, ctx: &Context, slash: &CommandInteraction) -> Result<()> {
    let execution_result = match slash.data.name.as_str() {
        "battle" => battle::run_command(bot, slash, &slash.data.options()).await,
        "exp" => exp::run_command(bot, slash, &slash.data.options()).await,
        "id" => id::run_command(bot, slash, &slash.data.options()).await,
        "ping" => ping::run_command(ctx, slash).await,
        _ => return Ok(()),
    };

    let models = execution_result?;

    for model in models {
        match model {
            ResponseModel::Send { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    return Ok(());
                };

                slash.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::from(first_blueprint.clone())
                )).await?;

                for blueprint in blueprints.iter().skip(1) {
                    slash.channel_id.send_message(&ctx.http,
                        CreateMessage::from(blueprint.clone())
                    ).await?;
                }
            },
            ResponseModel::SendEphemeral { blueprint  } => {
                slash.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::from(blueprint.clone())
                        .ephemeral(true)
                )).await?;
            },
            ResponseModel::SendLoose { blueprints } => {
                for blueprint in blueprints {
                    slash.channel_id.send_message(&ctx.http,
                        CreateMessage::from(blueprint.clone())
                    ).await?;
                }
            },
            _ => unreachable!("Unsupported ResponseModel for slash command: {:?}", model),
        }
    }
    
    Ok(())
}

pub async fn run_component(bot: &impl AsBot, ctx: &Context, comp: &ComponentInteraction, args: &[&str]) -> Result<()> {
    let execution_result = match args[0] {
        "id" => id::run_component(bot, comp, &args[1..]).await,
        _ => return Ok(()),
    };

    let models = execution_result?;

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