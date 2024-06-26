mod commands {
    pub mod attack;
    pub mod dodge;
}
pub mod prelude;

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
    
    let (blueprints, mode) = execution_result?;

    let responses = 'execute_blueprints: {
        let mut responses = Vec::new();
        
        let Some(first_blueprint) = blueprints.first() else {
            break 'execute_blueprints responses;
        };

        responses.push(msg.channel_id.send_message(&ctx.http,
            CreateMessage::from(first_blueprint.clone())
                .reference_message(msg)
        ).await?);

        for blueprint in blueprints.iter().skip(1) {
            responses.push(msg.channel_id.send_message(&ctx.http,
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