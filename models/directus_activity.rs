use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_activity {
    pub id: i32,
    pub action: String,
    pub user: Option<Uuid>,
    pub timestamp: String,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub collection: String,
    pub item: String,
    pub comment: Option<String>,
    pub origin: Option<String>,
}

impl From<Row> for directus_activity {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            action: row.get("action"),
            user: row.get("user"),
            timestamp: row.get("timestamp"),
            ip: row.get("ip"),
            user_agent: row.get("user_agent"),
            collection: row.get("collection"),
            item: row.get("item"),
            comment: row.get("comment"),
            origin: row.get("origin"),
        }
    }
}
