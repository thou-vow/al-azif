mod bot;
mod config;
pub mod prelude;

use crate::prelude::*;

fn main() {
    let subscriber = Subscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    subscriber::set_global_default(subscriber).unwrap();

    let rt = Runtime::new().unwrap();

    if let Err(why) = rt.block_on(try_main()) {
        error!("Main error: {why}");
    }
}

async fn try_main() -> Result<()> {
    config::load();

    let intents = GatewayIntents::GUILDS
                                | GatewayIntents::GUILD_MESSAGES
                                | GatewayIntents::GUILD_VOICE_STATES
                                | GatewayIntents::MESSAGE_CONTENT;
    
    let mut client = Client::builder(&config::get_bot_token()?, intents)
        .event_handler(Bot {
            cache: Arc::new(InMemoryDatabase::default()),
            main_guild: config::get_main_guild()?,
        })
        .await?;

    client.start().await?;
    
    Ok(())
}