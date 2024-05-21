use crate::prelude::*;

#[derive(Default, Deserialize, Serialize)]
pub struct Id {
    pub tag: Box<str>,
    pub ego: Ego,
    pub lvl: i64,
    pub xp: i64,
    pub hp: i64,
    pub sp: i64,
    pub points_to_distribute: i64,
    pub attributes: Attributes,
    pub color: Option<u32>,
    pub current_battle: Option<Box<str>>,
}
impl Id {
    pub fn new(tag: &str) -> Self {
        Self {
            tag: tag.into(),
            ego: Ego::default(),
            lvl: 0,
            xp: 0,
            hp: 50,
            sp: 50,
            points_to_distribute: 200,
            attributes: Attributes::new(),
            color: None,
            current_battle: None,
        }
    }
}
impl Id {
    pub async fn join_battle(&mut self, battle: &mut Battle) {
        let opponent = Opponent {
            action_value: self.attributes.movement,
        };

        battle.opponents.insert(self.tag.clone(), opponent);

        if battle.state.action_value_cap < self.attributes.movement {
            battle.state.action_value_cap = self.attributes.movement;
        }
        
        self.current_battle = Some(battle.tag.clone());
    }
    pub fn take_damage(&mut self, value: i64) {
        self.hp = max(self.hp - value, 0);
    }
}
impl Reflective for Id {
    const FOLDER_PATH: &'static str = "./database/ids";
    fn get_tag(&self) -> &str {
        self.tag.as_ref()
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Ego {
    pub name: Box<str>,
    pub gender: Gender,
    pub age: Age,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Attributes {
    pub constitution: i64,
    pub spirit: i64,
    pub might: i64,
    pub movement: i64,
    pub dexterity: i64,
    pub cognition: i64,
    pub charisma: i64,
}
impl Attributes {
    pub fn new() -> Self {
        Self {
            constitution: 5,
            spirit: 5,
            might: 5,
            movement: 5,
            dexterity: 5,
            cognition: 5,
            charisma: 5,
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub enum Gender {
    #[default] Other,
    Female,
    Male,
}

#[derive(Default, Deserialize, Serialize)]
pub enum Age {
    Child,
    Teen,
    #[default] Young,
    Adult,
    MiddleAged,
    Senior,
}