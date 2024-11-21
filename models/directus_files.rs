use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_files {
    pub id: Uuid,
    pub storage: String,
    pub filename_disk: Option<String>,
    pub filename_download: String,
    pub title: Option<String>,
    pub type: Option<String>,
    pub folder: Option<Uuid>,
    pub uploaded_by: Option<Uuid>,
    pub created_on: String,
    pub modified_by: Option<Uuid>,
    pub modified_on: String,
    pub charset: Option<String>,
    pub filesize: Option<i64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
    pub embed: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub tags: Option<String>,
    pub metadata: Option<Json>,
    pub focal_point_x: Option<i32>,
    pub focal_point_y: Option<i32>,
    pub tus_id: Option<String>,
    pub tus_data: Option<Json>,
    pub uploaded_on: Option<String>,
}

impl From<Row> for directus_files {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            storage: row.get("storage"),
            filename_disk: row.get("filename_disk"),
            filename_download: row.get("filename_download"),
            title: row.get("title"),
            type: row.get("type"),
            folder: row.get("folder"),
            uploaded_by: row.get("uploaded_by"),
            created_on: row.get("created_on"),
            modified_by: row.get("modified_by"),
            modified_on: row.get("modified_on"),
            charset: row.get("charset"),
            filesize: row.get("filesize"),
            width: row.get("width"),
            height: row.get("height"),
            duration: row.get("duration"),
            embed: row.get("embed"),
            description: row.get("description"),
            location: row.get("location"),
            tags: row.get("tags"),
            metadata: row.get("metadata"),
            focal_point_x: row.get("focal_point_x"),
            focal_point_y: row.get("focal_point_y"),
            tus_id: row.get("tus_id"),
            tus_data: row.get("tus_data"),
            uploaded_on: row.get("uploaded_on"),
        }
    }
}
