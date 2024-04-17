
use r2d2_sqlite::SqliteConnectionManager;

const SCHEMA: &str = "BEGIN;
CREATE TABLE IF NOT EXISTS users (
   id   INTEGER PRIMARY KEY NOT NULL,
   first_name TEXT NOT NULL,
   last_name TEXT,
   username TEXT
);
CREATE TABLE IF NOT EXISTS channels (
   id   INTEGER PRIMARY KEY NOT NULL,
   registered_by INTEGER NOT NULL,
   link TEXT NOT NULL,
   name TEXT NOT NULL,
   FOREIGN KEY(registered_by) REFERENCES users(id),
   UNIQUE(id, registered_by)
);
CREATE TABLE IF NOT EXISTS invitations(
   id   INTEGER PRIMARY KEY AUTOINCREMENT,
   date TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
   source INTEGER NOT NULL,
   dest INTEGER NOT NULL,
   chan INTEGER NOT NULL,
   contest INTEGER NOT NULL,
   FOREIGN KEY(source) REFERENCES users(id),
   FOREIGN KEY(dest) REFERENCES users(id),
   FOREIGN KEY(chan) REFERENCES channels(id),
   FOREIGN KEY(contest) REFERENCES contests(id),
   CHECK (source <> dest),
   UNIQUE(source, dest, chan)
);
CREATE TABLE IF NOT EXISTS contests(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  prize TEXT NOT NULL,
  end TIMESTAMP NOT NULL,
  chan INTEGER NOT NULL,
  started_at TIMESTAMP NULL,
  stopped BOOL NOT NULL DEFAULT FALSE,
  FOREIGN KEY(chan) REFERENCES channels(id),
  UNIQUE(name, chan)
);
CREATE TABLE IF NOT EXISTS being_managed_channels(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  chan INTEGER NOT NULL,
  FOREIGN KEY(chan) REFERENCES channels(id)
);
CREATE TABLE IF NOT EXISTS being_contacted_users(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user INTEGER NOT NULL,
  owner INTEGER NOT NULL,
  contest INTEGER NOT NULL,
  contacted BOOL NOT NULL DEFAULT FALSE,
  FOREIGN KEY(user) REFERENCES users(id),
  FOREIGN KEY(owner) REFERENCES users(id)
);
COMMIT;";

#[must_use]
pub fn connection() -> r2d2::Pool<SqliteConnectionManager> {
    let manager = SqliteConnectionManager::file("raf.db")
        .with_init(|c| c.execute_batch("PRAGMA foreign_keys=1;"));
    let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();
    {
        let conn = pool.get().unwrap();
        conn.execute_batch(SCHEMA).unwrap();
    }

    pool
}
