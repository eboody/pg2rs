use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_relations {
    pub id: i32,
    pub many_collection: String,
    pub many_field: String,
    pub one_collection: Option<String>,
    pub one_field: Option<String>,
    pub one_collection_field: Option<String>,
    pub one_allowed_collections: Option<String>,
    pub junction_field: Option<String>,
    pub sort_field: Option<String>,
    pub one_deselect_action: String,
}

impl From<Row> for directus_relations {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            many_collection: row.get("many_collection"),
            many_field: row.get("many_field"),
            one_collection: row.get("one_collection"),
            one_field: row.get("one_field"),
            one_collection_field: row.get("one_collection_field"),
            one_allowed_collections: row.get("one_allowed_collections"),
            junction_field: row.get("junction_field"),
            sort_field: row.get("sort_field"),
            one_deselect_action: row.get("one_deselect_action"),
        }
    }
}
