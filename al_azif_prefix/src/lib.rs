pub mod commands {
    pub mod attack;
    pub mod dodge;
}
pub mod prelude;
pub mod utils;

use crate::prelude::*;

pub async fn run_command(bot: &impl AsBot, ctx: &Context, msg: &Message) -> Result<()> {
    let mut args = msg.content[PREFIX.len()..].split_ascii_whitespace();

    let cmd_name = args.next().unwrap().to_lowercase();
    
    let execution_result = if attack::ALIASES.contains(&cmd_name.as_str()) {
        attack::run_command(bot, msg, &args.collect::<Box<[&str]>>()).await
    } else if dodge::ALIASES.contains(&cmd_name.as_str()) {
        dodge::run_command(bot, msg).await
    } else {
        return Ok(())
    };
    
    let models = execution_result?;

    for model in models {
        match model {
            ResponseModel::Send { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    return Ok(());
                };

                msg.channel_id.send_message(&ctx.http,
                    CreateMessage::from(first_blueprint.clone())
                        .reference_message(msg)
                ).await?;

                for blueprint in blueprints.iter().skip(1) {
                    msg.channel_id.send_message(&ctx.http,
                        CreateMessage::from(blueprint.clone())
                    ).await?;
                }
            },
            ResponseModel::SendLoose { blueprints } => {
                for blueprint in blueprints {
                    msg.channel_id.send_message(&ctx.http,
                        CreateMessage::from(blueprint.clone())
                    ).await?;
                }
            },
            _ => unreachable!("Unsupported ResponseModel for prefix command: {:?}", model),
        }
    }

    Ok(())
}

pub async fn run_component(bot: &impl AsBot, ctx: &Context, comp: &ComponentInteraction, args: &[&str]) -> Result<()> {
    let execution_result = match args[0] {
        "dodge" => dodge::run_component(bot, comp, &args[1..]).await,
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
                    
                    tokio::time::sleep(RESPONSE_INTERVAL).await;
                }
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