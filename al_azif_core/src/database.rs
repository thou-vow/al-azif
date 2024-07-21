use crate::_prelude::*;

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

    let serialized = fs::read(full_path)?;

    Ok(bincode::deserialize(&serialized)?)
}

pub fn set<T: Reflective>(value: &T) -> Result<()> {
    let full_path = f!("{}/{}.bin", T::FOLDER_PATH, value.get_tag());

    let serialized = bincode::serialize(value)?;

    fs::write(full_path, serialized)?;

    Ok(())
}

pub fn cut<T: Reflective>(tag: impl AsRef<str>) -> Result<()> {
    let tag = tag.as_ref();

    _cut::<T>(tag)
}
fn _cut<T: Reflective>(tag: &str) -> Result<()> {
    let full_path = f!("{}/{tag}.bin", T::FOLDER_PATH);

    fs::remove_file(full_path)?;

    Ok(())
}
