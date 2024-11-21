use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct articles_translations {
    pub id: i32,
    pub articles_id: Option<Uuid>,
    pub languages_code: Option<String>,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
}

impl From<Row> for articles_translations {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            articles_id: row.get("articles_id"),
            languages_code: row.get("languages_code"),
            title: row.get("title"),
            slug: row.get("slug"),
            summary: row.get("summary"),
            content: row.get("content"),
        }
    }
}
