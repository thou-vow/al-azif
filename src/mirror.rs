use std::{
	fmt::Display,
	marker::PhantomData,
	ops::{Deref, DerefMut},
	sync::Arc,
};

use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use uuid::Uuid;

use crate::{
	IdMap, IdRegistry,
	rehydration::{RawBehavior, RehydratedBehavior},
};

pub trait MirrorableBehavior: Send + Sync + RehydratedBehavior {
	const DIR: &str;
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Id<T: MirrorableBehavior> {
	value:   Uuid,
	_marker: PhantomData<T>,
}
impl<T: MirrorableBehavior> RawBehavior for Id<T>
where IdRegistry: AsRef<IdMap<T>>
{
	type Rehydrated = Mirror<T>;

	fn rehydrate(
		self,
		registry: &IdRegistry,
	) -> impl Future<Output = Result<Self::Rehydrated, String>> + Send {
		Box::pin(async move {
			let map = registry.as_ref();

			if let Some(resolved_reflock) = map.lock().await.get(&self) {
				return Ok(Self::Rehydrated { id: self, lock_ref: Arc::clone(resolved_reflock) });
			}

			let raw = registry.persistence.get(&self).await?;
			let resolved = raw.rehydrate(registry).await?;

			let resolved_reflock = Arc::new(RwLock::new(resolved));
			map.lock().await.insert(self.clone(), Arc::clone(&resolved_reflock));

			Ok(Self::Rehydrated { id: self, lock_ref: resolved_reflock })
		})
	}
}
impl<T: MirrorableBehavior> Display for Id<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.value.fmt(f) }
}
impl<T: MirrorableBehavior> Eq for Id<T> {}
impl<T: MirrorableBehavior> std::hash::Hash for Id<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.value.hash(state); }
}
impl<T: MirrorableBehavior> PartialEq for Id<T> {
	fn eq(&self, other: &Self) -> bool { self.value == other.value }
}

#[derive(Clone, Debug)]
pub struct Mirror<T: MirrorableBehavior> {
	id:       Id<T>,
	lock_ref: Arc<RwLock<T>>,
}
impl<T: MirrorableBehavior> Mirror<T> {
	pub fn id(&self) -> Id<T> { self.id.clone() }

	pub async fn read(&self) -> ReadMirror<'_, T> { ReadMirror { guard: self.lock_ref.read().await } }

	pub async fn write(&self) -> WriteMirror<'_, T> { WriteMirror { guard: self.lock_ref.write().await } }
}
impl<T: MirrorableBehavior> RehydratedBehavior for Mirror<T>
where IdRegistry: AsRef<IdMap<T>>
{
	type Raw = Id<T>;

	fn raw(self) -> Self::Raw { self.id }
}

#[derive(Debug)]
pub struct ReadMirror<'a, T: MirrorableBehavior> {
	guard: RwLockReadGuard<'a, T>,
}
impl<'a, T: MirrorableBehavior> ReadMirror<'a, T> {
	pub fn unread(self) {}
}
impl<'a, T: MirrorableBehavior> Deref for ReadMirror<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target { &self.guard }
}

#[derive(Debug)]
pub struct WriteMirror<'a, T: MirrorableBehavior> {
	guard: RwLockWriteGuard<'a, T>,
}
impl<'a, T: MirrorableBehavior> WriteMirror<'a, T> {
	pub fn downgrade(self) -> ReadMirror<'a, T> { ReadMirror { guard: self.guard.downgrade() } }

	pub fn unwrite(self) {}
}
impl<'a, T: MirrorableBehavior> Deref for WriteMirror<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target { &self.guard }
}
impl<'a, T: MirrorableBehavior> DerefMut for WriteMirror<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.guard }
}
