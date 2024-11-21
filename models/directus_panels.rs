use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_panels {
    pub id: Uuid,
    pub dashboard: Uuid,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub show_header: bool,
    pub note: Option<String>,
    pub type: String,
    pub position_x: i32,
    pub position_y: i32,
    pub width: i32,
    pub height: i32,
    pub options: Option<Json>,
    pub date_created: Option<String>,
    pub user_created: Option<Uuid>,
}

impl From<Row> for directus_panels {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            dashboard: row.get("dashboard"),
            name: row.get("name"),
            icon: row.get("icon"),
            color: row.get("color"),
            show_header: row.get("show_header"),
            note: row.get("note"),
            type: row.get("type"),
            position_x: row.get("position_x"),
            position_y: row.get("position_y"),
            width: row.get("width"),
            height: row.get("height"),
            options: row.get("options"),
            date_created: row.get("date_created"),
            user_created: row.get("user_created"),
        }
    }
}
