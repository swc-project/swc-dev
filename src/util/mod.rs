use anyhow::{anyhow, Error};
use once_cell::sync::Lazy;
use std::{collections::HashMap, env, fmt::Display, path::PathBuf, sync::RwLock};

pub mod cargo;
pub mod node;

pub type AHashMap<K, V> = HashMap<K, V, ahash::RandomState>;

// pub type AHashSet<V> = HashSet<V, ahash::RandomState>;

pub(crate) trait CargoEditResultExt<T>: Into<cargo_edit::Result<T>> {
    fn map_err_op(self, op: impl Display) -> Result<T, Error> {
        self.into()
            .map_err(|err| anyhow!("failed to {}: {:?}", op, err))
    }
}

impl<T> CargoEditResultExt<T> for cargo_edit::Result<T> {}

pub(crate) fn find_executable(name: &str) -> Option<PathBuf> {
    static CACHE: Lazy<RwLock<HashMap<String, PathBuf>>> = Lazy::new(|| Default::default());

    {
        let locked = CACHE.read().unwrap();
        if let Some(cached) = locked.get(name) {
            return Some(cached.clone());
        }
    }

    let path = env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    });

    if let Some(path) = path.clone() {
        let mut locked = CACHE.write().unwrap();
        locked.insert(name.to_string(), path);
    }

    path
}
