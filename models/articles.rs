use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct articles {
    pub id: Uuid,
    pub status: String,
    pub sort: Option<i32>,
    pub user_created: Option<Uuid>,
    pub date_created: Option<String>,
    pub user_updated: Option<Uuid>,
    pub date_updated: Option<String>,
    pub image: Option<Uuid>,
    pub author: Option<Uuid>,
}

impl From<Row> for articles {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            status: row.get("status"),
            sort: row.get("sort"),
            user_created: row.get("user_created"),
            date_created: row.get("date_created"),
            user_updated: row.get("user_updated"),
            date_updated: row.get("date_updated"),
            image: row.get("image"),
            author: row.get("author"),
        }
    }
}
