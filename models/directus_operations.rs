use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_operations {
    pub id: Uuid,
    pub name: Option<String>,
    pub key: String,
    pub type: String,
    pub position_x: i32,
    pub position_y: i32,
    pub options: Option<Json>,
    pub resolve: Option<Uuid>,
    pub reject: Option<Uuid>,
    pub flow: Uuid,
    pub date_created: Option<String>,
    pub user_created: Option<Uuid>,
}

impl From<Row> for directus_operations {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            key: row.get("key"),
            type: row.get("type"),
            position_x: row.get("position_x"),
            position_y: row.get("position_y"),
            options: row.get("options"),
            resolve: row.get("resolve"),
            reject: row.get("reject"),
            flow: row.get("flow"),
            date_created: row.get("date_created"),
            user_created: row.get("user_created"),
        }
    }
}
