use postgres::row::Row;
use postgres::types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
pub struct directus_users {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub location: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Json>,
    pub avatar: Option<Uuid>,
    pub language: Option<String>,
    pub tfa_secret: Option<String>,
    pub status: String,
    pub role: Option<Uuid>,
    pub token: Option<String>,
    pub last_access: Option<String>,
    pub last_page: Option<String>,
    pub provider: String,
    pub external_identifier: Option<String>,
    pub auth_data: Option<Json>,
    pub email_notifications: Option<bool>,
    pub appearance: Option<String>,
    pub theme_dark: Option<String>,
    pub theme_light: Option<String>,
    pub theme_light_overrides: Option<Json>,
    pub theme_dark_overrides: Option<Json>,
    pub other_avatar: Option<Uuid>,
}

impl From<Row> for directus_users {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            email: row.get("email"),
            password: row.get("password"),
            location: row.get("location"),
            title: row.get("title"),
            description: row.get("description"),
            tags: row.get("tags"),
            avatar: row.get("avatar"),
            language: row.get("language"),
            tfa_secret: row.get("tfa_secret"),
            status: row.get("status"),
            role: row.get("role"),
            token: row.get("token"),
            last_access: row.get("last_access"),
            last_page: row.get("last_page"),
            provider: row.get("provider"),
            external_identifier: row.get("external_identifier"),
            auth_data: row.get("auth_data"),
            email_notifications: row.get("email_notifications"),
            appearance: row.get("appearance"),
            theme_dark: row.get("theme_dark"),
            theme_light: row.get("theme_light"),
            theme_light_overrides: row.get("theme_light_overrides"),
            theme_dark_overrides: row.get("theme_dark_overrides"),
            other_avatar: row.get("other_avatar"),
        }
    }
}
