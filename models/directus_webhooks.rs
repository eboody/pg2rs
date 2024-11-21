use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_webhooks {
    pub id: i32,
    pub name: String,
    pub method: String,
    pub url: String,
    pub status: String,
    pub data: bool,
    pub actions: String,
    pub collections: String,
    pub headers: Option<Json>,
    pub was_active_before_deprecation: bool,
    pub migrated_flow: Option<Uuid>,
}

impl From<Row> for directus_webhooks {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            method: row.get("method"),
            url: row.get("url"),
            status: row.get("status"),
            data: row.get("data"),
            actions: row.get("actions"),
            collections: row.get("collections"),
            headers: row.get("headers"),
            was_active_before_deprecation: row.get("was_active_before_deprecation"),
            migrated_flow: row.get("migrated_flow"),
        }
    }
}
