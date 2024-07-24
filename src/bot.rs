use crate::prelude::*;

pub struct Bot {
    pub cache:      Arc<InMemoryDatabase>,
    pub main_guild: GuildId,
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
    fn get_cache(&self) -> Arc<InMemoryDatabase> { self.cache.clone() }

    fn get_main_guild(&self) -> &GuildId { &self.main_guild }
}

impl AsRef<InMemoryStore<Battle>> for Bot {
    fn as_ref(&self) -> &InMemoryStore<Battle> { &self.cache.battles }
}
impl AsRef<InMemoryStore<Id>> for Bot {
    fn as_ref(&self) -> &InMemoryStore<Id> { &self.cache.ids }
}
impl AsRef<InMemoryStore<Player>> for Bot {
    fn as_ref(&self) -> &InMemoryStore<Player> { &self.cache.players }
}
