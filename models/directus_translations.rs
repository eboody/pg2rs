use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_translations {
    pub id: Uuid,
    pub language: String,
    pub key: String,
    pub value: String,
}

impl From<Row> for directus_translations {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            language: row.get("language"),
            key: row.get("key"),
            value: row.get("value"),
        }
    }
}
