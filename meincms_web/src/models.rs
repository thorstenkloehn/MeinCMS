use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WikiArtikel {
    pub id: i64,
    pub tenant_id: String,
    pub slug: String,
    pub namespace_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WikiArtikelVersion {
    pub version_nummer: i64,
    pub tenant_id: String,
    pub markdown_inhalt: Option<String>,
    pub wiki_text_inhalt: Option<String>,
    pub html_inhalt: Option<String>,
    pub zeitpunkt: DateTime<Utc>,
    pub kategorie: Vec<String>,
    pub wiki_artikel_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiArtikelWithVersion {
    pub id: i64,
    pub tenant_id: String,
    pub slug: String,
    pub latest_version: Option<WikiArtikelVersion>,
}

#[derive(Debug, Deserialize)]
pub struct ArticleSaveForm {
    pub syntax: String,
    pub markdown_inhalt: Option<String>,
    pub wiki_text_inhalt: Option<String>,
    pub kategorien_raw: Option<String>,
}
