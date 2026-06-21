use std::sync::Arc;

use crate::common::result::NBResult;

pub(crate) trait FrameLocalSource: Send + Sync {
    fn dev_find_by_route(&self, route: &str) -> NBResult<Option<String>>;
    fn dev_upsert(&self, route: &str, checksum: &str, frame_json: &str) -> NBResult<()>;

    fn prod_find_by_route(&self, route: &str) -> NBResult<Option<String>>;
    fn prod_exists(&self, route: &str) -> NBResult<bool>;
    fn prod_find_checksum(&self, route: &str, checksum: &str) -> NBResult<Option<String>>;
    fn prod_upsert(&self, route: &str, checksum: &str, frame_json: &str) -> NBResult<()>;

    fn clear_all(&self) -> NBResult<()>;
    fn clear(&self, route: &str) -> NBResult<()>;
}

#[cfg(feature = "cache-sqlite")]
pub(crate) fn new_frame_local_source(db_path: &str) -> NBResult<Arc<dyn FrameLocalSource>> {
    let source: Arc<dyn FrameLocalSource> = sqlite::SqliteFrameDatabase::open(db_path)?;
    return Ok(source);
}

#[cfg(feature = "cache-sqlite")]
pub(crate) mod sqlite {
    use std::sync::{Arc, Mutex};

    use rusqlite::{Connection, OptionalExtension, params};

    use super::FrameLocalSource;
    use crate::common::result::{ErrorModel, NBResult};

    pub(crate) struct SqliteFrameDatabase {
        conn: Mutex<Connection>,
    }

    impl SqliteFrameDatabase {
        pub(crate) fn open(path: &str) -> NBResult<Arc<Self>> {
            let conn = Connection::open(path).map_err(map_error)?;
            Self::init(conn)
        }

        fn init(conn: Connection) -> NBResult<Arc<Self>> {
            // The two frame Room tables, verbatim. `IF NOT EXISTS` keeps existing
            // on-device databases untouched, so no schema migration is needed.
            conn.execute_batch(
                r#"
                CREATE TABLE IF NOT EXISTS `frame` (
                    `route` TEXT NOT NULL,
                    `checksum` TEXT NOT NULL,
                    `frameJson` TEXT NOT NULL,
                    PRIMARY KEY(`route`)
                );
                CREATE TABLE IF NOT EXISTS `frame_production` (
                    `route` TEXT NOT NULL,
                    `checksum` TEXT NOT NULL,
                    `frameJson` TEXT NOT NULL,
                    PRIMARY KEY(`route`)
                );
                "#,
            )
            .map_err(map_error)?;
            Ok(Arc::new(Self {
                conn: Mutex::new(conn),
            }))
        }

        fn lock(&self) -> NBResult<std::sync::MutexGuard<'_, Connection>> {
            self.conn
                .lock()
                .map_err(|_| ErrorModel::cache("Frame database connection poisoned"))
        }

        fn query_text(&self, sql: &str, key: &str) -> NBResult<Option<String>> {
            let conn = self.lock()?;
            conn.query_row(sql, params![key], |r| r.get::<_, String>(0))
                .optional()
                .map_err(map_error)
        }
    }

    impl FrameLocalSource for SqliteFrameDatabase {
        fn dev_find_by_route(&self, route: &str) -> NBResult<Option<String>> {
            self.query_text("SELECT frameJson FROM frame WHERE route = ?1 LIMIT 1", route)
        }

        fn dev_upsert(&self, route: &str, checksum: &str, frame_json: &str) -> NBResult<()> {
            self.lock()?
                .execute(
                    "INSERT OR REPLACE INTO frame (route, checksum, frameJson) VALUES (?1, ?2, ?3)",
                    params![route, checksum, frame_json],
                )
                .map_err(map_error)?;
            Ok(())
        }

        fn prod_find_by_route(&self, route: &str) -> NBResult<Option<String>> {
            self.query_text(
                "SELECT frameJson FROM frame_production WHERE route = ?1 LIMIT 1",
                route,
            )
        }

        fn prod_exists(&self, route: &str) -> NBResult<bool> {
            let conn = self.lock()?;
            conn.query_row(
                "SELECT EXISTS(SELECT 1 FROM frame_production WHERE route = ?1)",
                params![route],
                |r| r.get::<_, bool>(0),
            )
            .map_err(map_error)
        }

        fn prod_find_checksum(&self, route: &str, checksum: &str) -> NBResult<Option<String>> {
            let conn = self.lock()?;
            conn.query_row(
                "SELECT checksum FROM frame_production WHERE route = ?1 AND checksum = ?2 LIMIT 1",
                params![route, checksum],
                |r| r.get::<_, String>(0),
            )
            .optional()
            .map_err(map_error)
        }

        fn prod_upsert(&self, route: &str, checksum: &str, frame_json: &str) -> NBResult<()> {
            self.lock()?
                .execute(
                    "INSERT OR REPLACE INTO frame_production (route, checksum, frameJson) VALUES (?1, ?2, ?3)",
                    params![route, checksum, frame_json],
                )
                .map_err(map_error)?;
            Ok(())
        }

        fn clear_all(&self) -> NBResult<()> {
            let conn = self.lock()?;
            conn.execute("DELETE FROM frame", []).map_err(map_error)?;
            conn.execute("DELETE FROM frame_production", [])
                .map_err(map_error)?;
            Ok(())
        }

        fn clear(&self, route: &str) -> NBResult<()> {
            let conn = self.lock()?;
            conn.execute("DELETE FROM frame WHERE route = ?1", params![route])
                .map_err(map_error)?;
            conn.execute(
                "DELETE FROM frame_production WHERE route = ?1",
                params![route],
            )
            .map_err(map_error)?;
            Ok(())
        }
    }

    fn map_error(error: rusqlite::Error) -> ErrorModel {
        ErrorModel::cache(error.to_string())
    }
}
