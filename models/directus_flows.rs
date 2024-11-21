use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_flows {
    pub id: Uuid,
    pub name: String,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub trigger: Option<String>,
    pub accountability: Option<String>,
    pub options: Option<Json>,
    pub operation: Option<Uuid>,
    pub date_created: Option<String>,
    pub user_created: Option<Uuid>,
}

impl From<Row> for directus_flows {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            icon: row.get("icon"),
            color: row.get("color"),
            description: row.get("description"),
            status: row.get("status"),
            trigger: row.get("trigger"),
            accountability: row.get("accountability"),
            options: row.get("options"),
            operation: row.get("operation"),
            date_created: row.get("date_created"),
            user_created: row.get("user_created"),
        }
    }
}
