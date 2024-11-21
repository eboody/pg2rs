use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_comments {
    pub id: Uuid,
    pub collection: String,
    pub item: String,
    pub comment: String,
    pub date_created: Option<String>,
    pub date_updated: Option<String>,
    pub user_created: Option<Uuid>,
    pub user_updated: Option<Uuid>,
}

impl From<Row> for directus_comments {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            collection: row.get("collection"),
            item: row.get("item"),
            comment: row.get("comment"),
            date_created: row.get("date_created"),
            date_updated: row.get("date_updated"),
            user_created: row.get("user_created"),
            user_updated: row.get("user_updated"),
        }
    }
}
