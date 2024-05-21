pub mod prelude;

use crate::prelude::*;

pub async fn try_interaction(bot: &impl AsBot, ctx: &Context, intr: &Interaction) -> Result<()> {
    match intr {
        Interaction::Command(slash) => al_azif_slash::run_command(bot, ctx, slash).await?,
        Interaction::Component(comp) => {
            let mut args = comp.data.custom_id.split(' ');
            
            match args.next().unwrap() {
                "prefix" => {},
                "slash" => al_azif_slash::run_component(bot, ctx, comp, &args.collect::<Box<[&str]>>()).await?,
                invalid => Err(anyhow!("Component is neither 'prefix' or 'slash': {invalid}"))?
            }
        },
        _ => {}
    }

    Ok(())
}

pub async fn try_message(bot: &impl AsBot, ctx: &Context, msg: &Message) -> Result<()> {
    if msg.author.bot() || msg.guild_id.is_none() {
        return Ok(());
    }

    if msg.content.starts_with(PREFIX) {
        al_azif_prefix::run_command(bot, ctx, msg).await?;
    }

    Ok(())
}

pub async fn try_ready(bot: &impl AsBot, ctx: &Context, _ready: &Ready) -> Result<()> {
    al_azif_slash::register_commands(bot, ctx).await?;
    
    ctx.idle();

    bot.spawn_flush_routine();

    Ok(())
}
