use crate::_prelude::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot deserialize, why: {0}")]
    CannotDeserialize(serde_json::Error),
    #[error("Cannot read file, why: {0}")]
    CannotReadFile(io::Error),
    #[error("Cannot remove file, why: {0}")]
    CannotRemoveFile(io::Error),
    #[error("Cannot serialize, why: {0}")]
    CannotSerialize(serde_json::Error),
    #[error("Cannot write file, why: {0}")]
    CannotWriteFile(io::Error),
}

pub trait Reflective: for<'de> Deserialize<'de> + Send + Serialize + Sync {
    const FOLDER_PATH: &'static str;
    fn get_tag(&self) -> &str;
}

pub fn get<T: Reflective>(tag: impl AsRef<str>) -> Result<T> {
    let tag = tag.as_ref();

    _get(tag)
}
fn _get<T: Reflective>(tag: &str) -> Result<T> {
    let full_path = f!("{}/{tag}.bin", T::FOLDER_PATH);

    let serialized = fs::read_to_string(full_path).map_err(DatabaseError::CannotReadFile)?;

    Ok(serde_json::from_str(&serialized).map_err(DatabaseError::CannotDeserialize)?)
}

pub fn set<T: Reflective>(value: &T) -> Result<()> {
    let full_path = f!("{}/{}.json", T::FOLDER_PATH, value.get_tag());

    let serialized = serde_json::to_string(value).map_err(DatabaseError::CannotSerialize)?;

    fs::write(full_path, serialized).map_err(DatabaseError::CannotWriteFile)?;

    Ok(())
}

pub fn cut<T: Reflective>(tag: impl AsRef<str>) -> Result<()> {
    let tag = tag.as_ref();

    _cut::<T>(tag)
}
fn _cut<T: Reflective>(tag: &str) -> Result<()> {
    let full_path = f!("{}/{tag}.json", T::FOLDER_PATH);

    fs::remove_file(full_path).map_err(DatabaseError::CannotRemoveFile)?;

    Ok(())
}
