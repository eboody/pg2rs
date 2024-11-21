use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_revisions {
    pub id: i32,
    pub activity: i32,
    pub collection: String,
    pub item: String,
    pub data: Option<Json>,
    pub delta: Option<Json>,
    pub parent: Option<i32>,
    pub version: Option<Uuid>,
}

impl From<Row> for directus_revisions {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            activity: row.get("activity"),
            collection: row.get("collection"),
            item: row.get("item"),
            data: row.get("data"),
            delta: row.get("delta"),
            parent: row.get("parent"),
            version: row.get("version"),
        }
    }
}
