use rug::Integer;

use crate::{
	conflict::Conflict,
	define_rehydrated,
	entity::Entity,
	mirror::{Mirror, MirrorableBehavior},
};

define_rehydrated!(ConflictMember {
	conflict: Mirror<Conflict>,
	entity: Mirror<Entity>,
	action_score: Integer,
});
impl MirrorableBehavior for ConflictMember {
	const DIR: &str = "conflict_member";
}

impl Mirror<ConflictMember> {
	pub async fn pivot_action_score(&self) {
		let entity = self.read().await.entity.clone();

		let score_increase = entity.read().await.attributes.movement.clone();

		self.write().await.action_score += score_increase;
	}
}
