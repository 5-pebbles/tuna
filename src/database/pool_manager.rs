use rocket_sync_db_pools::{
    r2d2,
    rusqlite::{self, Error},
};
use std::path::{Path, PathBuf};

use crate::database::MyConnection;

type OnInitFn = dyn Fn(&mut MyConnection) -> Result<(), rusqlite::Error> + Send + Sync + 'static;

/// A pool manager for MyConnection to implement the Poolable trait
///
/// this is mostly yoinked from the crate r2d2_sqlite
pub struct MyPoolManager {
    path: PathBuf,
    flags: rusqlite::OpenFlags,
    on_init: Option<Box<OnInitFn>>,
}

impl r2d2::ManageConnection for MyPoolManager {
    type Connection = MyConnection;
    type Error = rusqlite::Error;

    fn connect(&self) -> Result<MyConnection, Error> {
        rusqlite::Connection::open_with_flags(&self.path, self.flags)
            .map_err(Into::into)
            .and_then(|c| match self.on_init {
                None => Ok(MyConnection(c)),
                Some(ref on_init) => {
                    let mut my_conn = MyConnection(c);
                    on_init(&mut my_conn).map(|_| my_conn)
                }
            })
    }

    fn is_valid(&self, conn: &mut MyConnection) -> Result<(), Error> {
        conn.execute_batch("").map_err(Into::into)
    }

    fn has_broken(&self, _: &mut MyConnection) -> bool {
        false
    }
}

impl MyPoolManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            flags: rusqlite::OpenFlags::default(),
            on_init: None,
        }
    }

    pub fn with_flags(self, flags: rusqlite::OpenFlags) -> Self {
        Self { flags, ..self }
    }

    pub fn with_init<F>(self, on_init: F) -> Self
    where
        F: Fn(&mut MyConnection) -> Result<(), rusqlite::Error> + Send + Sync + 'static,
    {
        let on_init: Option<Box<OnInitFn>> = Some(Box::new(on_init));
        Self { on_init, ..self }
    }
}
