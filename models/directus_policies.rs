use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_policies {
    pub id: Uuid,
    pub name: String,
    pub icon: String,
    pub description: Option<String>,
    pub ip_access: Option<String>,
    pub enforce_tfa: bool,
    pub admin_access: bool,
    pub app_access: bool,
}

impl From<Row> for directus_policies {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            icon: row.get("icon"),
            description: row.get("description"),
            ip_access: row.get("ip_access"),
            enforce_tfa: row.get("enforce_tfa"),
            admin_access: row.get("admin_access"),
            app_access: row.get("app_access"),
        }
    }
}
