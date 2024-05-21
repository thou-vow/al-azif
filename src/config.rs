use crate::prelude::*;

pub fn load() {
    dotenv().ok();
}

pub fn get_bot_token() -> Result<String> {
    Ok(env::var("DISCORD_BOT_TOKEN")?)
}

pub fn get_main_guild() -> Result<GuildId> {
    Ok(env::var("DISCORD_MAIN_GUILD")?.parse()?)
}