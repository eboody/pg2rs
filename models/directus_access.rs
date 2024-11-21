use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_access {
    pub id: Uuid,
    pub role: Option<Uuid>,
    pub user: Option<Uuid>,
    pub policy: Uuid,
    pub sort: Option<i32>,
}

impl From<Row> for directus_access {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            role: row.get("role"),
            user: row.get("user"),
            policy: row.get("policy"),
            sort: row.get("sort"),
        }
    }
}
