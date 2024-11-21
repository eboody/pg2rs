use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_roles {
    pub id: Uuid,
    pub name: String,
    pub icon: String,
    pub description: Option<String>,
    pub parent: Option<Uuid>,
}

impl From<Row> for directus_roles {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            icon: row.get("icon"),
            description: row.get("description"),
            parent: row.get("parent"),
        }
    }
}
