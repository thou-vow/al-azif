use std::{collections::HashMap, sync::Arc};

use tokio::sync::{Mutex, RwLock};

use crate::{
	conflict::Conflict, conflict_member::ConflictMember, entity::Entity, environment::Environment,
	mirror::Id, persistence::Persistence,
};

pub mod conflict;
pub mod conflict_member;
pub mod entity;
pub mod environment;
pub mod mirror;
pub mod persistence;
pub mod rehydration;

#[derive(Debug)]
pub struct IdRegistry {
	pub persistence:         Persistence,
	pub conflict_map:        IdMap<Conflict>,
	pub conflict_member_map: IdMap<ConflictMember>,
	pub entity_map:          IdMap<Entity>,
	pub environment_map:     IdMap<Environment>,
}
impl AsRef<IdMap<Conflict>> for IdRegistry {
	fn as_ref(&self) -> &IdMap<Conflict> { &self.conflict_map }
}
impl AsRef<IdMap<ConflictMember>> for IdRegistry {
	fn as_ref(&self) -> &IdMap<ConflictMember> { &self.conflict_member_map }
}
impl AsRef<IdMap<Entity>> for IdRegistry {
	fn as_ref(&self) -> &IdMap<Entity> { &self.entity_map }
}
impl AsRef<IdMap<Environment>> for IdRegistry {
	fn as_ref(&self) -> &IdMap<Environment> { &self.environment_map }
}

pub type IdMap<T> = Mutex<HashMap<Id<T>, Arc<RwLock<T>>>>;
