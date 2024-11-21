use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_versions {
    pub id: Uuid,
    pub key: String,
    pub name: Option<String>,
    pub collection: String,
    pub item: String,
    pub hash: Option<String>,
    pub date_created: Option<String>,
    pub date_updated: Option<String>,
    pub user_created: Option<Uuid>,
    pub user_updated: Option<Uuid>,
    pub delta: Option<Json>,
}

impl From<Row> for directus_versions {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            key: row.get("key"),
            name: row.get("name"),
            collection: row.get("collection"),
            item: row.get("item"),
            hash: row.get("hash"),
            date_created: row.get("date_created"),
            date_updated: row.get("date_updated"),
            user_created: row.get("user_created"),
            user_updated: row.get("user_updated"),
            delta: row.get("delta"),
        }
    }
}
