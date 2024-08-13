mod bot;
mod config;
pub mod prelude;

use crate::prelude::*;

fn main() {
    let subscriber = tracing_subscriber::fmt().pretty().with_ansi(true).with_max_level(Level::DEBUG).finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let rt = Runtime::new().unwrap();

    if let Err(why) = rt.block_on(try_main()) {
        error!("Error in main: {why:?}");
    }
}

async fn try_main() -> Result<()> {
    let config = Config::load()?;

    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::MESSAGE_CONTENT;

    let songbird_manager = Songbird::serenity();

    let mut client = Client::builder(&config.discord_bot_token, intents)
        .event_handler(Bot {
            lang:                  Lang::Pt,
            listened_channel_id:   None,
            in_memory_database:    Arc::new(InMemoryDatabase::default()),
            main_guild_id:         config.discord_main_guild_id,
            main_voice_channel_id: config.discord_main_voice_channel_id,
            reqwest_client:        ReqwestClient::new(),
            songbird_manager:      songbird_manager.clone(),
        })
        .voice_manager::<Songbird>(songbird_manager)
        .await?;

    client.start().await?;

    Ok(())
}
