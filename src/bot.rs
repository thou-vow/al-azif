use crate::prelude::*;

pub struct Bot {
    pub lang:                  Lang,
    pub listened_channel_id:   Option<ChannelId>,
    pub in_memory_database:    Arc<InMemoryDatabase>,
    pub main_guild_id:         GuildId,
    pub main_voice_channel_id: ChannelId,
    pub reqwest_client:        ReqwestClient,
    pub songbird_manager:      Arc<Songbird>,
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, intr: Interaction) {
        if let Err(why) = al_azif_events::try_interaction(self, &ctx, &intr).await {
            error!("Error in interaction create: {why:?}");
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(why) = al_azif_events::try_message(self, &ctx, &msg).await {
            error!("Error in message: {why:?}");
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        if let Err(why) = al_azif_events::try_ready(self, &ctx, &ready).await {
            error!("Error in ready: {why:?}");
        }
    }
}

impl AsBot for Bot {
    fn get_in_memory_database(&self) -> Arc<InMemoryDatabase> { self.in_memory_database.clone() }

    fn get_lang(&self) -> &Lang { &self.lang }

    fn get_listened_channel_id(&self) -> Option<ChannelId> { self.listened_channel_id }

    fn get_main_guild_id(&self) -> GuildId { self.main_guild_id }

    fn get_main_voice_channel_id(&self) -> ChannelId { self.main_voice_channel_id }

    fn get_reqwest_client(&self) -> ReqwestClient { self.reqwest_client.clone() }

    fn get_songbird_manager(&self) -> Arc<Songbird> { self.songbird_manager.clone() }

    fn set_listened_channel_id(&mut self, channel_id: Option<ChannelId>) { self.listened_channel_id = channel_id }
}

impl AsRef<InMemoryStore<Battle>> for Bot {
    fn as_ref(&self) -> &InMemoryStore<Battle> { &self.in_memory_database.battles }
}
impl AsRef<InMemoryStore<Id>> for Bot {
    fn as_ref(&self) -> &InMemoryStore<Id> { &self.in_memory_database.ids }
}
impl AsRef<InMemoryStore<Player>> for Bot {
    fn as_ref(&self) -> &InMemoryStore<Player> { &self.in_memory_database.players }
}
