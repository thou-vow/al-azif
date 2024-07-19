use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Id {
    pub tag: FixedString,
    pub name: FixedString,
    pub gender: Gender,
    pub age: Age,
    pub lvl: i64,
    pub xp: i64,
    pub hp: i64,
    pub sp: i64,
    pub points_to_distribute: i64,
    pub constitution: i64,
    pub spirit: i64,
    pub might: i64,
    pub movement: i64,
    pub dexterity: i64,
    pub cognition: i64,
    pub charisma: i64,
    pub color: Option<u32>,
    pub current_battle_tag: Option<FixedString>,
}
impl Id {
    pub fn new(tag: impl AsRef<str>) -> Self {
        Id::_new(tag.as_ref())
    }
    fn _new(tag: &str) -> Self {
        Self {
            tag: FixedString::from_str_trunc(tag),
            name: FixedString::from_static_trunc("Unknown"),
            gender: Gender::Other,
            age: Age::Child,
            lvl: 0,
            xp: 0,
            hp: 50,
            sp: 50,
            points_to_distribute: 200,
            constitution: 5,
            spirit: 5,
            might: 5,
            movement: 5,
            dexterity: 5,
            cognition: 5,
            charisma: 5,
            color: None,
            current_battle_tag: None,
        }
    }
}
impl Id {
    pub async fn join_battle(&mut self, battle: &mut Battle) {
        let opponent = Opponent {
            tag: self.tag.clone(),
            turn_value: self.movement,
            last_total_increased_turn_value_amount: self.movement,
        };

        battle.opponents.insert(self.tag.clone(), opponent);

        self.current_battle_tag = Some(battle.tag.clone());
    }
    pub async fn start_turn<'a>(
        &mut self,
        _battle: &mut Battle,
    ) -> Result<Vec<ResponseBlueprint<'a>>> {
        let mut blueprints = Vec::new();

        blueprints.push(ResponseBlueprint {
            content: Some(f!("🕒 | Agora é a vez de **{}**.", self.name).into()),
            ..Default::default()
        });

        Ok(blueprints)
    }
    pub async fn end_turn<'a>(
        &mut self,
        battle: &mut Battle,
    ) -> Result<Vec<ResponseBlueprint<'a>>> {
        let mut blueprints = Vec::new();

        battle
            .opponents
            .get_mut(&self.tag)
            .unwrap()
            .sub_turn_value(battle.turn_value_cap);

        blueprints.push(ResponseBlueprint {
            content: Some(f!("🏁 | Fim do turno de **{}**.", self.name).into()),
            ..Default::default()
        });

        Ok(blueprints)
    }
    pub fn evaluate_damage_to_receive(&self, value: i64) -> i64 {
        value
    }
    pub fn receive_damage(&mut self, value: i64) {
        self.hp = (self.hp - self.evaluate_damage_to_receive(value)).clamp(0, self.hp)
    }
}
impl Reflective for Id {
    const FOLDER_PATH: &'static str = "./database/ids";
    fn get_tag(&self) -> &str {
        self.tag.as_ref()
    }
}

#[derive(Deserialize, Serialize)]
pub enum Gender {
    Other = 0,
    Female = 1,
    Male = 2,
}

#[derive(Deserialize, Serialize)]
pub enum Age {
    Child = 1,
    Teen = 2,
    Young = 3,
    Adult = 4,
    MiddleAged = 5,
    Senior = 6,
}
