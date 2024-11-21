use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_folders {
    pub id: Uuid,
    pub name: String,
    pub parent: Option<Uuid>,
}

impl From<Row> for directus_folders {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            parent: row.get("parent"),
        }
    }
}
