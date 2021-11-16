use std::borrow::Cow;
use std::collections::HashSet;
use std::fs::{create_dir_all, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use heed::types::{ByteSlice, OwnedType, Str};
use heed::{CompactionOption, Database, Env, EnvOpenOptions};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::error::{AuthResolverError, Result};
use super::Key;

const AUTH_STORE_SIZE: usize = 1_073_741_824; //1GiB
const AUTH_DB_PATH: &str = "auth";
const KEY_DB_NAME: &str = "api-keys";

#[derive(Clone)]
pub struct HeedAuthStore {
    env: Env,
    keys: Database<Str, SerdeJsonCodec<Key>>,
}

impl HeedAuthStore {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().join(AUTH_DB_PATH);
        create_dir_all(&path)?;
        let mut options = EnvOpenOptions::new();
        options.map_size(AUTH_STORE_SIZE); // 1GB
        options.max_dbs(1);
        let env = options.open(path)?;
        let keys = env.create_database(Some(KEY_DB_NAME))?;
        Ok(Self { env, keys })
    }

    pub fn put_key(&self, key: Key) -> Result<()> {
        let mut wtxn = self.env.write_txn()?;
        self.keys.put(&mut wtxn, &key.key, &key);
        wtxn.commit()?;

        Ok(())
    }

    pub fn get_key(&self, key: impl AsRef<str>) -> Result<Option<Key>> {
        let mut rtxn = self.env.read_txn()?;
        self.keys.get(&mut rtxn, key.as_ref()).map_err(|e| e.into())
    }
}

/// Heed codec allowing to encode/decode everithing that implement Serialize and Deserialize
/// in order to store it in heed.
/// This is obviously not the best approach and should never be used for big and numerous objects,
/// but it is a simple one.
pub struct SerdeJsonCodec<T>(std::marker::PhantomData<T>);

impl<'a, T> heed::BytesDecode<'a> for SerdeJsonCodec<T>
where
    T: Deserialize<'a> + 'a,
{
    type DItem = T;

    fn bytes_decode(bytes: &'a [u8]) -> Option<Self::DItem> {
        serde_json::from_slice(bytes).ok()
    }
}

impl<'a, T> heed::BytesEncode<'a> for SerdeJsonCodec<T>
where
    T: Serialize + 'a,
{
    type EItem = T;

    fn bytes_encode(item: &Self::EItem) -> Option<Cow<[u8]>> {
        serde_json::to_vec(item).map(|bytes| Cow::Owned(bytes)).ok()
    }
}
