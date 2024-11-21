use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct spatial_ref_sys {
    pub srid: i32,
    pub auth_name: Option<String>,
    pub auth_srid: Option<i32>,
    pub srtext: Option<String>,
    pub proj_4text: Option<String>,
}

impl From<Row> for spatial_ref_sys {
    fn from(row: Row) -> Self {
        Self {
            srid: row.get("srid"),
            auth_name: row.get("auth_name"),
            auth_srid: row.get("auth_srid"),
            srtext: row.get("srtext"),
            proj_4text: row.get("proj4text"),
        }
    }
}
