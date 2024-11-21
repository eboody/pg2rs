use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_sessions {
    pub token: String,
    pub user: Option<Uuid>,
    pub expires: String,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub share: Option<Uuid>,
    pub origin: Option<String>,
    pub next_token: Option<String>,
}

impl From<Row> for directus_sessions {
    fn from(row: Row) -> Self {
        Self {
            token: row.get("token"),
            user: row.get("user"),
            expires: row.get("expires"),
            ip: row.get("ip"),
            user_agent: row.get("user_agent"),
            share: row.get("share"),
            origin: row.get("origin"),
            next_token: row.get("next_token"),
        }
    }
}
