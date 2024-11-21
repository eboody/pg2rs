use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct ebooks_directus_users {
    pub id: i32,
    pub ebooks_id: Option<Uuid>,
    pub directus_users_id: Option<Uuid>,
}

impl From<Row> for ebooks_directus_users {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            ebooks_id: row.get("ebooks_id"),
            directus_users_id: row.get("directus_users_id"),
        }
    }
}
