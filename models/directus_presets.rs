use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_presets {
    pub id: i32,
    pub bookmark: Option<String>,
    pub user: Option<Uuid>,
    pub role: Option<Uuid>,
    pub collection: Option<String>,
    pub search: Option<String>,
    pub layout: Option<String>,
    pub layout_query: Option<Json>,
    pub layout_options: Option<Json>,
    pub refresh_interval: Option<i32>,
    pub filter: Option<Json>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

impl From<Row> for directus_presets {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            bookmark: row.get("bookmark"),
            user: row.get("user"),
            role: row.get("role"),
            collection: row.get("collection"),
            search: row.get("search"),
            layout: row.get("layout"),
            layout_query: row.get("layout_query"),
            layout_options: row.get("layout_options"),
            refresh_interval: row.get("refresh_interval"),
            filter: row.get("filter"),
            icon: row.get("icon"),
            color: row.get("color"),
        }
    }
}
