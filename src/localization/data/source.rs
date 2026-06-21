use crate::common::result::NbResult;

pub trait LocalizationLocalSource: Send + Sync {
    fn dev_find_by_language_code(&self, language_code: &str) -> NbResult<Option<String>>;
    fn dev_upsert(&self, language_code: &str, checksum: &str, localization_json: &str)
    -> NbResult<()>;

    fn prod_find_by_language_code(&self, language_code: &str) -> NbResult<Option<String>>;
    fn prod_find_checksum(&self, language_code: &str, checksum: &str) -> NbResult<Option<String>>;
    fn prod_upsert(
        &self,
        language_code: &str,
        checksum: &str,
        localization_json: &str,
    ) -> NbResult<()>;
}

#[cfg(feature = "cache-sqlite")]
pub(crate) mod sqlite {
    use std::sync::{Arc, Mutex};

    use rusqlite::{Connection, OptionalExtension, params};

    use super::LocalizationLocalSource;
    use crate::common::result::{ErrorModel, NbResult};

    pub(crate) struct SqliteLocalizationDatabase {
        conn: Mutex<Connection>,
    }

    impl SqliteLocalizationDatabase {
        pub(crate) fn open(path: &str) -> NbResult<Arc<Self>> {
            let conn = Connection::open(path).map_err(map_error)?;
            Self::init(conn)
        }

        #[cfg(test)]
        pub(crate) fn in_memory() -> NbResult<Arc<Self>> {
            let conn = Connection::open_in_memory().map_err(map_error)?;
            Self::init(conn)
        }

        fn init(conn: Connection) -> NbResult<Arc<Self>> {
            // The two localization Room tables, verbatim. `IF NOT EXISTS` keeps an
            // existing on-device database untouched, so no schema migration runs.
            conn.execute_batch(
                r#"
                CREATE TABLE IF NOT EXISTS `localization` (
                    `languageCode` TEXT NOT NULL,
                    `checksum` TEXT NOT NULL,
                    `localizationJson` TEXT NOT NULL,
                    PRIMARY KEY(`languageCode`)
                );
                CREATE TABLE IF NOT EXISTS `localization_production` (
                    `languageCode` TEXT NOT NULL,
                    `checksum` TEXT NOT NULL,
                    `localizationJson` TEXT NOT NULL,
                    PRIMARY KEY(`languageCode`)
                );
                "#,
            )
            .map_err(map_error)?;
            Ok(Arc::new(Self {
                conn: Mutex::new(conn),
            }))
        }

        fn lock(&self) -> NbResult<std::sync::MutexGuard<'_, Connection>> {
            self.conn
                .lock()
                .map_err(|_| ErrorModel::cache("Localization database connection poisoned"))
        }
    }

    impl LocalizationLocalSource for SqliteLocalizationDatabase {
        fn dev_find_by_language_code(&self, language_code: &str) -> NbResult<Option<String>> {
            let conn = self.lock()?;
            conn.query_row(
                "SELECT localizationJson FROM localization WHERE languageCode = ?1 LIMIT 1",
                params![language_code],
                |r| r.get::<_, String>(0),
            )
            .optional()
            .map_err(map_error)
        }

        fn dev_upsert(
            &self,
            language_code: &str,
            checksum: &str,
            localization_json: &str,
        ) -> NbResult<()> {
            self.lock()?
                .execute(
                    "INSERT OR REPLACE INTO localization (languageCode, checksum, localizationJson) VALUES (?1, ?2, ?3)",
                    params![language_code, checksum, localization_json],
                )
                .map_err(map_error)?;
            Ok(())
        }

        fn prod_find_by_language_code(&self, language_code: &str) -> NbResult<Option<String>> {
            let conn = self.lock()?;
            conn.query_row(
                "SELECT localizationJson FROM localization_production WHERE languageCode = ?1 LIMIT 1",
                params![language_code],
                |r| r.get::<_, String>(0),
            )
            .optional()
            .map_err(map_error)
        }

        fn prod_find_checksum(
            &self,
            language_code: &str,
            checksum: &str,
        ) -> NbResult<Option<String>> {
            let conn = self.lock()?;
            conn.query_row(
                "SELECT checksum FROM localization_production WHERE languageCode = ?1 AND checksum = ?2 LIMIT 1",
                params![language_code, checksum],
                |r| r.get::<_, String>(0),
            )
            .optional()
            .map_err(map_error)
        }

        fn prod_upsert(
            &self,
            language_code: &str,
            checksum: &str,
            localization_json: &str,
        ) -> NbResult<()> {
            self.lock()?
                .execute(
                    "INSERT OR REPLACE INTO localization_production (languageCode, checksum, localizationJson) VALUES (?1, ?2, ?3)",
                    params![language_code, checksum, localization_json],
                )
                .map_err(map_error)?;
            Ok(())
        }
    }

    fn map_error(error: rusqlite::Error) -> ErrorModel {
        ErrorModel::cache(error.to_string())
    }
}
