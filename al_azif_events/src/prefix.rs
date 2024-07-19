use crate::prelude::*;
use al_azif_prefix::commands::*;

pub async fn run_command(bot: &impl AsBot, ctx: &Context, msg: &Message) -> Result<()> {
    let mut args = msg.content[PREFIX.len()..].split_ascii_whitespace();

    let cmd_name = args.next().unwrap().to_lowercase();

    let execution_result = if attack::ALIASES.contains(&cmd_name.as_str()) {
        attack::run_command(bot, msg, &args.collect::<Box<[&str]>>()).await
    } else if dodge::ALIASES.contains(&cmd_name.as_str()) {
        dodge::run_command(bot, msg).await
    } else {
        return Ok(());
    };

    let models = execution_result?;

    perform_response_models(ctx, msg, models).await
}

pub async fn perform_response_models<'a>(
    ctx: &Context,
    msg: &Message,
    models: Vec<ResponseModel<'a>>,
) -> Result<()> {
    for model in models {
        match model {
            ResponseModel::Send { blueprints } => {
                let Some(first_blueprint) = blueprints.first() else {
                    return Ok(());
                };

                msg.channel_id
                    .send_message(
                        &ctx.http,
                        CreateMessage::from(first_blueprint.clone()).reference_message(msg),
                    )
                    .await?;

                for blueprint in blueprints.iter().skip(1) {
                    msg.channel_id
                        .send_message(&ctx.http, CreateMessage::from(blueprint.clone()))
                        .await?;
                }
            }
            ResponseModel::SendLoose { blueprints } => {
                for blueprint in blueprints {
                    msg.channel_id
                        .send_message(&ctx.http, CreateMessage::from(blueprint.clone()))
                        .await?;
                }
            }
            _ => unreachable!("Unsupported ResponseModel for prefix command: {:?}", model),
        }
    }

    Ok(())
}
