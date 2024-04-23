use rocket::{serde, Build, Rocket};
use rocket_sync_db_pools::{
    r2d2,
    rusqlite,
    Config, PoolResult, Poolable,
};

use std::ops::{Deref, DerefMut};
use std::time::Duration;

use crate::database::MyPoolManager;

/// a wrapper for rusqlite's Connection
///
/// the is mostly yoinked from rocket_sync_db_pools implementation of Poolable for rusqlite Connection
pub struct MyConnection(pub rusqlite::Connection);

impl Poolable for MyConnection {
    type Manager = MyPoolManager;
    type Error = std::convert::Infallible;

    fn pool(db_name: &str, rocket: &Rocket<Build>) -> PoolResult<Self> {
        use rocket::figment::providers::Serialized;

        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        #[serde(crate = "rocket::serde", rename_all = "snake_case")]
        enum OpenFlag {
            ReadOnly,
            ReadWrite,
            Create,
            Uri,
            Memory,
            NoMutex,
            FullMutex,
            SharedCache,
            PrivateCache,
            Nofollow,
        }

        let figment = Config::figment(db_name, rocket);
        let config: Config = figment.extract()?;
        let open_flags: Vec<OpenFlag> = figment
            .join(Serialized::default("open_flags", <Vec<OpenFlag>>::new()))
            .extract_inner("open_flags")?;

        let mut flags = rusqlite::OpenFlags::default();
        for flag in open_flags {
            let sql_flag = match flag {
                OpenFlag::ReadOnly => rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
                OpenFlag::ReadWrite => rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
                OpenFlag::Create => rusqlite::OpenFlags::SQLITE_OPEN_CREATE,
                OpenFlag::Uri => rusqlite::OpenFlags::SQLITE_OPEN_URI,
                OpenFlag::Memory => rusqlite::OpenFlags::SQLITE_OPEN_MEMORY,
                OpenFlag::NoMutex => rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
                OpenFlag::FullMutex => rusqlite::OpenFlags::SQLITE_OPEN_FULL_MUTEX,
                OpenFlag::SharedCache => rusqlite::OpenFlags::SQLITE_OPEN_SHARED_CACHE,
                OpenFlag::PrivateCache => rusqlite::OpenFlags::SQLITE_OPEN_PRIVATE_CACHE,
                OpenFlag::Nofollow => rusqlite::OpenFlags::SQLITE_OPEN_NOFOLLOW,
            };

            flags.insert(sql_flag)
        }

        let manager = MyPoolManager::new(&*config.url)
            .with_flags(flags)
            .with_init(|conn| conn.execute_batch("PRAGMA foreign_keys = ON;")); // custom options

        let pool = r2d2::Pool::builder()
            .max_size(config.pool_size)
            .connection_timeout(Duration::from_secs(config.timeout as u64))
            .build(manager)?;

        Ok(pool)
    }
}

impl Deref for MyConnection {
    type Target = rusqlite::Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MyConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
