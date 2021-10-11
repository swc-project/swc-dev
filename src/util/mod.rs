use anyhow::{anyhow, Error};
use std::{collections::HashMap, fmt::Display};

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
