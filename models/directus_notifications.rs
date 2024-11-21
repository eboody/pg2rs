use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_notifications {
    pub id: i32,
    pub timestamp: Option<String>,
    pub status: Option<String>,
    pub recipient: Uuid,
    pub sender: Option<Uuid>,
    pub subject: String,
    pub message: Option<String>,
    pub collection: Option<String>,
    pub item: Option<String>,
}

impl From<Row> for directus_notifications {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            timestamp: row.get("timestamp"),
            status: row.get("status"),
            recipient: row.get("recipient"),
            sender: row.get("sender"),
            subject: row.get("subject"),
            message: row.get("message"),
            collection: row.get("collection"),
            item: row.get("item"),
        }
    }
}
