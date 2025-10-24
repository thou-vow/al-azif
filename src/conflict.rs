use crate::{
	conflict_member::ConflictMember,
	define_rehydrated,
	environment::Environment,
	mirror::{Mirror, MirrorableBehavior},
};

define_rehydrated!(Conflict {
	environment: Mirror<Environment>,
	conflict_members: Vec<Mirror<ConflictMember>>
});
impl MirrorableBehavior for Conflict {
	const DIR: &str = "conflict";
}

impl Mirror<Conflict> {
	pub async fn pivot_member_action_scores(&self) {
		for conflict_member in self.read().await.conflict_members.clone() {
			conflict_member.pivot_action_score().await
		}
	}
}
