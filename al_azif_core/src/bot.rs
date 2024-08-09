use crate::{_prelude::*, mirror};

pub trait AsBot: AsRef<InMemoryStore<Battle>> + AsRef<InMemoryStore<Id>> + AsRef<InMemoryStore<Player>> {
    fn get_in_memory_database(&self) -> Arc<InMemoryDatabase>;
    fn get_lang(&self) -> &Lang;
    fn get_main_guild(&self) -> &GuildId;
    fn spawn_flush_routine(&self) {
        let in_memory_database = self.get_in_memory_database();

        tokio::spawn(mirror::data_flush_routine(in_memory_database));
    }
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
