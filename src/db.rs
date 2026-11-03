use std::sync::{Mutex, MutexGuard};

use rusqlite::{params, Connection};

const SCHEMA: &str = include_str!("../sql/schema.sql");
const UPSERT_USER: &str = include_str!("../sql/upsert_user.sql");

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn open(path: &str) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(SCHEMA)?;
        Ok(Database { conn: Mutex::new(conn) })
    }

    fn conn(&self) -> MutexGuard<'_, Connection> {
        self.conn.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn upsert_user(
        &self,
        id: &str,
        username: &str,
        avatar: Option<&str>,
        now: i64,
    ) -> rusqlite::Result<()> {
        self.conn()
            .execute(UPSERT_USER, params![id, username, avatar, now])?;
        Ok(())
    }
}
