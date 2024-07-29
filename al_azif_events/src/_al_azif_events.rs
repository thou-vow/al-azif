#![feature(decl_macro)]

pub mod _prelude;
pub mod component;
pub mod prefix;
pub mod slash;

use crate::_prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] CoreError),
    #[error("Could not create response, why: {0}")]
    CouldNotCreateInteractionResponse(SerenityError),
    #[error("Could not delete message, why: {0}")]
    CouldNotDeleteMessage(SerenityError),
    #[error("Could not get original interaction response, why: {0}")]
    CouldNotGetOriginalInteractionResponse(SerenityError),
    #[error("Could not parse component arg '{arg}' into type {into_type}")]
    CouldNotParseComponentArgIntoType { arg: FixedString, into_type: &'static str },
    #[error("Could not send message, why: {0}")]
    CouldNotSendMessage(SerenityError),
    #[error("Could not set commands, why: {0}")]
    CouldNotSetSlashCommands(SerenityError),
    #[error("The component interaction custom id is empty")]
    EmptyComponentInteractionCustomId,
    #[error("Expected command option of name '{expected_name}' and type '{expected_type}'")]
    ExpectedAnotherSlashCommandOption { expected_name: &'static str, expected_type: &'static str },
    #[error("Expected command option of type '{r#type}' to be of name '{expected_name}'")]
    ExpectedAnotherSlashCommandOptionName { r#type: &'static str, expected_name: &'static str },
    #[error("Expected command option of name '{name}' to be of type '{expected_type}'")]
    ExpectedAnotherSlashCommandOptionType { name: &'static str, expected_type: &'static str },
    #[error("Invalid prefix command, name: {name}")]
    InvalidPrefixCommand { name: FixedString },
    #[error("Invalid prefix component, custom id: {custom_id}")]
    InvalidPrefixComponent { custom_id: FixedString },
    #[error("Invalid slash command, name: {name}")]
    InvalidSlashCommand { name: FixedString },
    #[error("Invalid slash component, custom id: {custom_id}")]
    InvalidSlashComponent { custom_id: FixedString },
    #[error("Missing required slash command option of name: {name}")]
    MissingRequiredSlashCommandOption { name: &'static str },
    #[error(transparent)]
    Prefix(#[from] PrefixError),
    #[error(transparent)]
    Slash(#[from] SlashError),
}

pub async fn try_interaction(bot: &impl AsBot, ctx: &Context, intr: &Interaction) -> Result<()> {
    match intr {
        Interaction::Command(slash) => slash::run(bot, ctx, slash).await?,
        Interaction::Component(comp) => component::run(bot, ctx, comp).await?,
        _ => {},
    }

    Ok(())
}

pub async fn try_message(bot: &impl AsBot, ctx: &Context, msg: &Message) -> Result<()> {
    if msg.author.bot() || msg.guild_id.is_none() {
        return Ok(());
    }

    if msg.content.starts_with(PREFIX) {
        prefix::run(bot, ctx, msg).await?;
    }

    Ok(())
}

pub async fn try_ready(bot: &impl AsBot, ctx: &Context, _ready: &Ready) -> Result<()> {
    slash::register(bot, ctx).await?;

    ctx.idle();

    bot.spawn_flush_routine();

    Ok(())
}
