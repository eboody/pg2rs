use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_fields {
    pub id: i32,
    pub collection: String,
    pub field: String,
    pub special: Option<String>,
    pub interface: Option<String>,
    pub options: Option<Json>,
    pub display: Option<String>,
    pub display_options: Option<Json>,
    pub readonly: bool,
    pub hidden: bool,
    pub sort: Option<i32>,
    pub width: Option<String>,
    pub translations: Option<Json>,
    pub note: Option<String>,
    pub conditions: Option<Json>,
    pub required: Option<bool>,
    pub group: Option<String>,
    pub validation: Option<Json>,
    pub validation_message: Option<String>,
}

impl From<Row> for directus_fields {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            collection: row.get("collection"),
            field: row.get("field"),
            special: row.get("special"),
            interface: row.get("interface"),
            options: row.get("options"),
            display: row.get("display"),
            display_options: row.get("display_options"),
            readonly: row.get("readonly"),
            hidden: row.get("hidden"),
            sort: row.get("sort"),
            width: row.get("width"),
            translations: row.get("translations"),
            note: row.get("note"),
            conditions: row.get("conditions"),
            required: row.get("required"),
            group: row.get("group"),
            validation: row.get("validation"),
            validation_message: row.get("validation_message"),
        }
    }
}
