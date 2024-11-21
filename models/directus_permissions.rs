use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_permissions {
    pub id: i32,
    pub collection: String,
    pub action: String,
    pub permissions: Option<Json>,
    pub validation: Option<Json>,
    pub presets: Option<Json>,
    pub fields: Option<String>,
    pub policy: Uuid,
}

impl From<Row> for directus_permissions {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            collection: row.get("collection"),
            action: row.get("action"),
            permissions: row.get("permissions"),
            validation: row.get("validation"),
            presets: row.get("presets"),
            fields: row.get("fields"),
            policy: row.get("policy"),
        }
    }
}
