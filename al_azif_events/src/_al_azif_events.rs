pub mod component;
pub mod prefix;
pub mod _prelude;
pub mod slash;
pub mod unclassified {
    pub mod receive;
}

use crate::_prelude::*;

pub async fn try_interaction(bot: &impl AsBot, ctx: &Context, intr: &Interaction) -> Result<()> {
    match intr {
        Interaction::Command(slash) => slash::run_command(bot, ctx, slash).await?,
        Interaction::Component(comp) => component::run(bot, ctx, comp).await?,
        _ => {}
    }

    Ok(())
}

pub async fn try_message(bot: &impl AsBot, ctx: &Context, msg: &Message) -> Result<()> {
    if msg.author.bot() || msg.guild_id.is_none() {
        return Ok(());
    }

    if msg.content.starts_with(PREFIX) {
        prefix::run_command(bot, ctx, msg).await?;
    }

    Ok(())
}

pub async fn try_ready(bot: &impl AsBot, ctx: &Context, _ready: &Ready) -> Result<()> {
    slash::register_commands(bot, ctx).await?;

    ctx.idle();

    bot.spawn_flush_routine();

    Ok(())
}
