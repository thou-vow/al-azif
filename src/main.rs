mod bot;
mod config;
pub mod prelude;

use crate::prelude::*;

fn main() {
    env::set_var("RUST_LIB_BACKTRACE", "1");

    let subscriber = Subscriber::builder().with_max_level(Level::DEBUG).finish();

    subscriber::set_global_default(subscriber).unwrap();

    let rt = Runtime::new().unwrap();

    if let Err(why) = rt.block_on(try_main()) {
        error!("Error in main: {why:?}");
    }
}

async fn try_main() -> Result<()> {
    let config = Config::load()?;

    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&config.discord_bot_token, intents)
        .event_handler(Bot {
            lang:       Lang::Pt,
            cache:      Arc::new(InMemoryDatabase::default()),
            main_guild: config.discord_main_guild,
        })
        .await?;

    client.start().await?;

    Ok(())
}
