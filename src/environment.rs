use crate::{
	conflict::Conflict,
	define_rehydrated,
	entity::Entity,
	mirror::{Mirror, MirrorableBehavior},
};

define_rehydrated!(Environment {
	conflict: Option<Mirror<Conflict>>,
	entities: Vec<Mirror<Entity>>,
});
impl MirrorableBehavior for Environment {
	const DIR: &str = "environment";
}

impl Mirror<Environment> {}
