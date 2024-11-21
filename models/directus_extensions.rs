use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_extensions {
    pub enabled: bool,
    pub id: Uuid,
    pub folder: String,
    pub source: String,
    pub bundle: Option<Uuid>,
}

impl From<Row> for directus_extensions {
    fn from(row: Row) -> Self {
        Self {
            enabled: row.get("enabled"),
            id: row.get("id"),
            folder: row.get("folder"),
            source: row.get("source"),
            bundle: row.get("bundle"),
        }
    }
}
