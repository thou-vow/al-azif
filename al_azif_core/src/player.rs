use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Player {
    pub tag: Box<str>,
}
impl Player {
    pub fn new(tag: &str) -> Self {
        Self {
            tag: tag.into()
        } 
    }
}
impl Reflective for Player {
    const FOLDER_PATH: &'static str = "./database/players";
    fn get_tag(&self) -> &str {
        self.tag.as_ref()
    }
}