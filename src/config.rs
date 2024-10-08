use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub discord_bot_token:             FixedString<u8>,
    pub discord_main_guild_id:         GuildId,
    pub discord_main_voice_channel_id: ChannelId,
}
impl Config {
    pub fn load() -> Result<Self> {
        let serialized = fs::read_to_string("config.toml")?;

        let config = toml::from_str(&serialized)?;

        Ok(config)
    }
}
