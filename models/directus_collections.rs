use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_collections {
    pub collection: String,
    pub icon: Option<String>,
    pub note: Option<String>,
    pub display_template: Option<String>,
    pub hidden: bool,
    pub singleton: bool,
    pub translations: Option<Json>,
    pub archive_field: Option<String>,
    pub archive_app_filter: bool,
    pub archive_value: Option<String>,
    pub unarchive_value: Option<String>,
    pub sort_field: Option<String>,
    pub accountability: Option<String>,
    pub color: Option<String>,
    pub item_duplication_fields: Option<Json>,
    pub sort: Option<i32>,
    pub group: Option<String>,
    pub collapse: String,
    pub preview_url: Option<String>,
    pub versioning: bool,
}

impl From<Row> for directus_collections {
    fn from(row: Row) -> Self {
        Self {
            collection: row.get("collection"),
            icon: row.get("icon"),
            note: row.get("note"),
            display_template: row.get("display_template"),
            hidden: row.get("hidden"),
            singleton: row.get("singleton"),
            translations: row.get("translations"),
            archive_field: row.get("archive_field"),
            archive_app_filter: row.get("archive_app_filter"),
            archive_value: row.get("archive_value"),
            unarchive_value: row.get("unarchive_value"),
            sort_field: row.get("sort_field"),
            accountability: row.get("accountability"),
            color: row.get("color"),
            item_duplication_fields: row.get("item_duplication_fields"),
            sort: row.get("sort"),
            group: row.get("group"),
            collapse: row.get("collapse"),
            preview_url: row.get("preview_url"),
            versioning: row.get("versioning"),
        }
    }
}
