use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct languages {
    pub code: String,
    pub name: Option<String>,
    pub direction: String,
}

impl From<Row> for languages {
    fn from(row: Row) -> Self {
        Self {
            code: row.get("code"),
            name: row.get("name"),
            direction: row.get("direction"),
        }
    }
}
