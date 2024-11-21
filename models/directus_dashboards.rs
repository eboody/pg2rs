use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_dashboards {
    pub id: Uuid,
    pub name: String,
    pub icon: String,
    pub note: Option<String>,
    pub date_created: Option<String>,
    pub user_created: Option<Uuid>,
    pub color: Option<String>,
}

impl From<Row> for directus_dashboards {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            icon: row.get("icon"),
            note: row.get("note"),
            date_created: row.get("date_created"),
            user_created: row.get("user_created"),
            color: row.get("color"),
        }
    }
}
