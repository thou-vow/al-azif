use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Player {
    pub tag: FixedString,
    pub owned_ids_tags: HashSet<FixedString>,
    pub borrowed_ids_tags: HashSet<FixedString>,
}
impl Player {
    pub fn new(tag: &str) -> Self {
        Self {
            tag: FixedString::from_str_trunc(tag),
            owned_ids_tags: HashSet::new(),
            borrowed_ids_tags: HashSet::new(),
        } 
    }
}
impl Reflective for Player {
    const FOLDER_PATH: &'static str = "./database/players";
    fn get_tag(&self) -> &str {
        self.tag.as_ref()
    }
}