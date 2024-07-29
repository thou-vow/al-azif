use crate::_prelude::*;

pub trait AsBot: AsRef<InMemoryStore<Battle>> + AsRef<InMemoryStore<Id>> + AsRef<InMemoryStore<Player>> {
    fn get_cache(&self) -> Arc<InMemoryDatabase>;
    fn get_lang(&self) -> &Lang;
    fn get_main_guild(&self) -> &GuildId;
    fn spawn_flush_routine(&self) {
        let cache = self.get_cache();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(CACHE_FLUSH_ROUTINE).await;

                let now = Instant::now();

                cache
                    .battles
                    .lock()
                    .await
                    .retain(|_, (_, last_accessed)| now - *last_accessed < CACHE_EXPIRE_TIME);
                cache.ids.lock().await.retain(|_, (_, last_accessed)| now - *last_accessed < CACHE_EXPIRE_TIME);
                cache
                    .players
                    .lock()
                    .await
                    .retain(|_, (_, last_accessed)| now - *last_accessed < CACHE_EXPIRE_TIME);
            }
        });
    }
}

#[derive(Default)]
pub struct InMemoryDatabase {
    pub battles: InMemoryStore<Battle>,
    pub ids:     InMemoryStore<Id>,
    pub players: InMemoryStore<Player>,
}

pub enum Lang {
    En,
    Pt,
}

pub macro lang_diff($bot:expr, en: $en:expr, pt: $pt:expr) {{
    match $bot.get_lang() {
        Lang::En => $en,
        Lang::Pt => $pt,
    }
}}
