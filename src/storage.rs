use sled::{Db, IVec};
use std::error::Error;

pub struct Storage {
    db: Db,
}

impl Storage {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let db = sled::open("passwords.db")?;
        Ok(Self { db })
    }

    pub fn insert(&self, key: &str, value: &[u8]) -> Result<(), Box<dyn Error>> {
        self.db.insert(key, value)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<IVec> {
        self.db.get(key).ok().flatten()
    }

    pub fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        self.db.remove(key)?;
        self.db.flush()?;
        Ok(())
    }
}
