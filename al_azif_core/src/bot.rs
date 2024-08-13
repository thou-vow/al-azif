use crate::{_prelude::*, mirror};

pub trait AsBot: AsRef<InMemoryStore<Battle>> + AsRef<InMemoryStore<Id>> + AsRef<InMemoryStore<Player>> {
    fn get_in_memory_database(&self) -> Arc<InMemoryDatabase>;
    fn get_lang(&self) -> &Lang;
    fn get_listened_channel_id(&self) -> Option<ChannelId>;
    fn get_main_guild_id(&self) -> GuildId;
    fn get_reqwest_client(&self) -> ReqwestClient;
    fn get_songbird_manager(&self) -> Arc<Songbird>;
    fn get_main_voice_channel_id(&self) -> ChannelId;
    fn spawn_flush_routine(&self) {
        let in_memory_database = self.get_in_memory_database();

        tokio::spawn(mirror::data_flush_routine(in_memory_database));
    }

    fn set_listened_channel_id(&mut self, channel_id: Option<ChannelId>);
}

#[derive(Debug, Default)]
pub struct InMemoryDatabase {
    pub battles: InMemoryStore<Battle>,
    pub ids:     InMemoryStore<Id>,
    pub players: InMemoryStore<Player>,
}

pub enum Lang {
    En,
    Pt,
}
