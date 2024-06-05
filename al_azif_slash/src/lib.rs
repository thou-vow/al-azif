mod commands {
    pub mod battle;
    pub mod exp;
    pub mod calc;
    pub mod id;
    pub mod ping;
}
pub mod prelude;

use crate::prelude::*;

pub async fn register_commands(bot: &impl AsBot, ctx: &Context) -> Result<()> {
    bot.get_main_guild().set_commands(&ctx.http, &[
        battle::register(),
        calc::register(),
        exp::register(),
        id::register(),
    ]).await?;

    Ok(())
}

pub async fn run_command(bot: &impl AsBot, ctx: &Context, slash: &CommandInteraction) -> Result<()> {
    let execution_result = match slash.data.name.as_str() {
        "battle" => battle::run_command(bot, slash, &slash.data.options()).await,
        "exp" => exp::run_command(bot, slash, &slash.data.options()).await,
        "calc" => calc::run_command(bot, slash, &slash.data.options()).await,
        "id" => id::run_command(bot, slash, &slash.data.options()).await,
        "ping" => ping::run_command(ctx, slash).await,
        _ => return Ok(()),
    };
    let (blueprints, mode) = execution_result?;

    let responses = 'execute_blueprints: {
        let mut responses = Vec::new();
        
        let Some(first_blueprint) = blueprints.first() else {
            break 'execute_blueprints responses;
        };
    
        slash.create_response(&ctx.http, CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::from(first_blueprint.clone())
        )).await?;
        responses.push(ctx.http.get_original_interaction_response(&slash.token).await?);

        for blueprint in blueprints.iter().skip(1) {
            responses.push(slash.channel_id.send_message(&ctx.http,
                CreateMessage::from(blueprint.clone())
            ).await?);
        }

        responses
    };

    match mode {
        ResponseMode::Delete => {
            tokio::time::sleep(Duration::from_secs(10)).await;
            
            for response in responses.iter().rev() {
                response.delete(&ctx.http, None).await?;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
        ResponseMode::Normal => {},
    }
    
    Ok(())
}

pub async fn run_component(bot: &impl AsBot, ctx: &Context, comp: &ComponentInteraction, args: &[&str]) -> Result<()> {
    let execution_result = match args[0] {
        "id" => id::run_component(bot, ctx, comp, &args[1..]).await,
        _ => return Ok(()),
    };
    let (blueprints, mode) = execution_result?;
    
    let responses = 'execute_blueprints: {
        let mut responses = Vec::new();
        
        let Some(first_blueprint) = blueprints.first() else {
            break 'execute_blueprints responses;
        };
    
        comp.create_response(&ctx.http, CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::from(first_blueprint.clone())
        )).await?;
        responses.push(ctx.http.get_original_interaction_response(&comp.token).await?);

        for blueprint in blueprints.iter().skip(1) {
            responses.push(comp.channel_id.send_message(&ctx.http,
                CreateMessage::from(blueprint.clone())
            ).await?);
        }

        responses
    };

    match mode {
        ResponseMode::Delete => {
            tokio::time::sleep(Duration::from_secs(10)).await;
            
            for response in responses.iter().rev() {
                response.delete(&ctx.http, None).await?;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
        ResponseMode::Normal => {},
    }

    Ok(())
}