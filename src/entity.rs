use rug::Integer;

use crate::{
	conflict_member::ConflictMember,
	define_rehydrated,
	mirror::{Mirror, MirrorableBehavior},
};

define_rehydrated!(Entity {
	attributes: 		 Attributes,
	conflict_member: Option<Mirror<ConflictMember>>,
	health:          Health,
	name:            Box<str>,
});
impl MirrorableBehavior for Entity {
	const DIR: &str = "entity";
}

define_rehydrated!(Health { current: Integer, max: Integer });

define_rehydrated!(Attributes {
	charisma:     Integer,
	cognition:    Integer,
	constitution: Integer,
	dexterity:    Integer,
	might:        Integer,
	movement:     Integer,
	spirit:       Integer,
});

impl Mirror<Entity> {}
