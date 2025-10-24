use std::path::PathBuf;

use crate::mirror::{Id, MirrorableBehavior};

#[derive(Debug)]
pub struct Persistence {
	path: PathBuf,
}
impl Persistence {
	pub fn new(path: impl Into<PathBuf>) -> Self { Self { path: path.into() } }

	pub async fn get<T: MirrorableBehavior>(&self, id: &Id<T>) -> Result<T::Raw, String> {
		let path = self.path.join(T::DIR).join(id.to_string() + ".json");

		let serialized = tokio::fs::read_to_string(&path).await.map_err(|why| why.to_string())?;

		serde_json::from_str(&serialized).map_err(|why| why.to_string())
	}

	pub async fn set<T: MirrorableBehavior>(&self, id: &Id<T>, raw: &T::Raw) -> Result<(), String> {
		let dir_path = self.path.join(T::DIR);
		let path = dir_path.join(id.to_string() + ".json");

		let serialized = serde_json::to_string_pretty(raw).map_err(|why| why.to_string())?;

		tokio::fs::create_dir_all(&dir_path).await.map_err(|why| why.to_string())?;

		tokio::fs::write(&path, serialized).await.map_err(|why| why.to_string())
	}

	pub async fn cut<T: MirrorableBehavior>(&self, id: Id<T>) -> Result<(), String> {
		let path = self.path.join(T::DIR).join(id.to_string() + ".json");

		tokio::fs::remove_file(&path).await.map_err(|why| why.to_string())?;

		Ok(())
	}
}
