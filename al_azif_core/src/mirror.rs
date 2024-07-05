use crate::prelude::*;

pub type InMemoryStore<T> = Mutex<HashMap<FixedString, (Arc<RwLock<T>>, Instant)>>;

pub struct Mirror<T: Reflective> {
    lock: Arc<RwLock<T>>,
}
impl<T: Reflective> Mirror<T> {
    pub async fn get(store_lock: impl AsRef<InMemoryStore<T>>, tag: &str) -> Result<Self> {
        let store_lock = store_lock.as_ref();
        let mut store = store_lock.lock().await;

        let Some((value_lock, instant)) = store.get_mut(tag) else {
            let value_lock = Arc::new(RwLock::new(database::get::<T>(tag)?));

            store.insert(FixedString::from_str_trunc(tag), (value_lock.clone(), Instant::now()));

            return Ok(Self { lock: value_lock });
        };

        *instant = Instant::now();

        Ok(Self { lock: value_lock.clone() })
    }
    pub async fn set_and_get(store_lock: impl AsRef<InMemoryStore<T>>, value: T) -> Result<Self> {
        database::set(&value)?;

        let tag = FixedString::from_str_trunc(value.get_tag());

        let store_lock = store_lock.as_ref();
        let mut store = store_lock.lock().await;
        
        let value_lock = Arc::new(RwLock::new(value));

        store.insert(tag, (value_lock.clone(), Instant::now()));

        Ok(Self { lock: value_lock.clone() })
    }
    pub async fn cut(store_lock: impl AsRef<InMemoryStore<T>>, tag: &str) -> Result<()> {
        let store_lock = store_lock.as_ref();
        let mut store = store_lock.lock().await;

        store.remove(tag);

        database::cut::<T>(tag)?;

        Ok(())
    }
    pub async fn read(&self) -> ReadMirror<'_, T> {
        ReadMirror { guard: self.lock.read().await }
    }
    pub async fn write(&self) -> WriteMirror<'_, T> {
        WriteMirror { guard: Some(self.lock.write().await) }
    }
}
impl<T: Reflective> Clone for Mirror<T> {
    fn clone(&self) -> Self {
        Self { lock: self.lock.clone() }
    }
}

pub struct ReadMirror<'a, T: Reflective> {
    guard: RwLockReadGuard<'a, T>
}
impl<'a, T: Reflective> Deref for ReadMirror<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

pub struct WriteMirror<'a, T: Reflective> {
    guard: Option<RwLockWriteGuard<'a, T>>
}
impl<'a, T: Reflective> WriteMirror<'a, T> {
    pub fn downgrade(mut self) -> Result<ReadMirror<'a, T>> {
        let guard = self.guard.take().unwrap().downgrade();

        database::set(&*guard)?;

        Ok(ReadMirror { guard })
    }
}
impl<'a, T: Reflective> Deref for WriteMirror<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.as_deref().unwrap()
    }
}
impl<'a, T: Reflective> DerefMut for WriteMirror<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.as_deref_mut().unwrap()
    }
}
impl<'a, T: Reflective> Drop for WriteMirror<'a, T> {
    fn drop(&mut self) {
        let Some(guard) = &self.guard else {
            return;
        };

        if let Err(why) = database::set(&**guard) {
            error!("WriteMirror drop error: {why:?}");
        }
    }
}