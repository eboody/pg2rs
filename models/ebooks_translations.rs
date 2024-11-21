use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct ebooks_translations {
    pub id: i32,
    pub ebooks_id: Option<Uuid>,
    pub languages_code: Option<String>,
    pub cover_image: Option<Uuid>,
    pub content: Option<String>,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub file: Option<Uuid>,
}

impl From<Row> for ebooks_translations {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            ebooks_id: row.get("ebooks_id"),
            languages_code: row.get("languages_code"),
            cover_image: row.get("cover_image"),
            content: row.get("content"),
            title: row.get("title"),
            slug: row.get("slug"),
            summary: row.get("summary"),
            file: row.get("file"),
        }
    }
}
