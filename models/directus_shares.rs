use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_shares {
    pub id: Uuid,
    pub name: Option<String>,
    pub collection: String,
    pub item: String,
    pub role: Option<Uuid>,
    pub password: Option<String>,
    pub user_created: Option<Uuid>,
    pub date_created: Option<String>,
    pub date_start: Option<String>,
    pub date_end: Option<String>,
    pub times_used: Option<i32>,
    pub max_uses: Option<i32>,
}

impl From<Row> for directus_shares {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            collection: row.get("collection"),
            item: row.get("item"),
            role: row.get("role"),
            password: row.get("password"),
            user_created: row.get("user_created"),
            date_created: row.get("date_created"),
            date_start: row.get("date_start"),
            date_end: row.get("date_end"),
            times_used: row.get("times_used"),
            max_uses: row.get("max_uses"),
        }
    }
}
