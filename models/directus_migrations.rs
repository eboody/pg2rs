use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_migrations {
    pub version: String,
    pub name: String,
    pub timestamp: Option<String>,
}

impl From<Row> for directus_migrations {
    fn from(row: Row) -> Self {
        Self {
            version: row.get("version"),
            name: row.get("name"),
            timestamp: row.get("timestamp"),
        }
    }
}
