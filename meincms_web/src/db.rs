use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{WikiArtikelVersion, WikiArtikelWithVersion};

#[derive(Clone)]
pub struct DbStore {
    // In-memory thread safe article storage indexed by (tenant_id, slug)
    articles: Arc<RwLock<HashMap<(String, String), Vec<WikiArtikelVersion>>>>,
}

impl DbStore {
    pub fn new() -> Self {
        let store = Self {
            articles: Arc::new(RwLock::new(HashMap::new())),
        };

        // Seed default Hauptseite for main and doc tenants
        let store_clone = store.clone();
        tokio::spawn(async move {
            store_clone.seed_defaults().await;
        });

        store
    }

    async fn seed_defaults(&self) {
        let default_markdown = "# Willkommen bei MeinCMS (Rust Edition)\n\nDies ist die Hauptseite von **MeinCMS**, vollständig in Rust implementiert.\n\n[[kategorie:Hauptseite]]";
        let default_html = meincms_parser::markdown_to_html(default_markdown);
        let default_categories = meincms_parser::get_markdown_categories(default_markdown);

        let initial_version = WikiArtikelVersion {
            version_nummer: 1,
            tenant_id: "main".to_string(),
            markdown_inhalt: Some(default_markdown.to_string()),
            wiki_text_inhalt: None,
            html_inhalt: Some(default_html),
            zeitpunkt: Utc::now(),
            kategorie: default_categories,
            wiki_artikel_id: 1,
        };

        let mut lock = self.articles.write().await;
        lock.insert(
            ("main".to_string(), "Hauptseite".to_string()),
            vec![initial_version.clone()],
        );

        let doc_version = WikiArtikelVersion {
            tenant_id: "doc".to_string(),
            ..initial_version
        };
        lock.insert(
            ("doc".to_string(), "Hauptseite".to_string()),
            vec![doc_version],
        );
    }

    pub async fn get_article(&self, tenant_id: &str, slug: &str) -> Option<WikiArtikelWithVersion> {
        let lock = self.articles.read().await;
        let versions = lock.get(&(tenant_id.to_string(), slug.to_string()))?;
        let latest = versions.last().cloned();

        Some(WikiArtikelWithVersion {
            id: 1,
            tenant_id: tenant_id.to_string(),
            slug: slug.to_string(),
            latest_version: latest,
        })
    }

    pub async fn get_history(&self, tenant_id: &str, slug: &str) -> Vec<WikiArtikelVersion> {
        let lock = self.articles.read().await;
        lock.get(&(tenant_id.to_string(), slug.to_string()))
            .cloned()
            .unwrap_or_default()
    }

    pub async fn get_version(&self, version_num: i64) -> Option<WikiArtikelVersion> {
        let lock = self.articles.read().await;
        for versions in lock.values() {
            for v in versions {
                if v.version_nummer == version_num {
                    return Some(v.clone());
                }
            }
        }
        None
    }

    pub async fn save_article(
        &self,
        tenant_id: &str,
        slug: &str,
        syntax: &str,
        markdown: Option<String>,
        mediawiki: Option<String>,
        kategorien: Vec<String>,
    ) {
        let mut lock = self.articles.write().await;
        let key = (tenant_id.to_string(), slug.to_string());
        let versions = lock.entry(key).or_insert_with(Vec::new);

        let next_version_num = (versions.len() as i64) + 1;

        // Backend Rule from AGENTS.md: Explicitly clear the unused content field!
        let (final_markdown, final_mediawiki, html_out, mut auto_cats) =
            if syntax.eq_ignore_ascii_case("mediawiki") {
                let mw_str = mediawiki.unwrap_or_default();
                let html = meincms_parser::wikitext_to_html(&mw_str);
                let cats = meincms_parser::get_wikitext_categories(&mw_str);
                (None, Some(mw_str), html, cats)
            } else {
                let md_str = markdown.unwrap_or_default();
                let html = meincms_parser::markdown_to_html(&md_str);
                let cats = meincms_parser::get_markdown_categories(&md_str);
                (Some(md_str), None, html, cats)
            };

        // Merge categories
        for cat in kategorien {
            if !auto_cats.contains(&cat) {
                auto_cats.push(cat);
            }
        }

        let version = WikiArtikelVersion {
            version_nummer: next_version_num,
            tenant_id: tenant_id.to_string(),
            markdown_inhalt: final_markdown,
            wiki_text_inhalt: final_mediawiki,
            html_inhalt: Some(html_out),
            zeitpunkt: Utc::now(),
            kategorie: auto_cats,
            wiki_artikel_id: next_version_num,
        };

        versions.push(version);
    }

    pub async fn get_all_articles(&self, tenant_id: &str) -> Vec<WikiArtikelWithVersion> {
        let lock = self.articles.read().await;
        let mut list = Vec::new();

        for ((t_id, slug), versions) in lock.iter() {
            if t_id == tenant_id {
                list.push(WikiArtikelWithVersion {
                    id: 1,
                    tenant_id: t_id.clone(),
                    slug: slug.clone(),
                    latest_version: versions.last().cloned(),
                });
            }
        }

        list
    }

    pub async fn search_articles(
        &self,
        tenant_id: &str,
        query: &str,
    ) -> Vec<WikiArtikelWithVersion> {
        let lock = self.articles.read().await;
        let mut list = Vec::new();
        let q_lower = query.to_lowercase();

        for ((t_id, slug), versions) in lock.iter() {
            if t_id == tenant_id {
                let slug_match = slug.to_lowercase().contains(&q_lower);
                let content_match = versions.last().map_or(false, |v| {
                    v.markdown_inhalt
                        .as_ref()
                        .map_or(false, |m| m.to_lowercase().contains(&q_lower))
                        || v.wiki_text_inhalt
                            .as_ref()
                            .map_or(false, |w| w.to_lowercase().contains(&q_lower))
                });

                if slug_match || content_match {
                    list.push(WikiArtikelWithVersion {
                        id: 1,
                        tenant_id: t_id.clone(),
                        slug: slug.clone(),
                        latest_version: versions.last().cloned(),
                    });
                }
            }
        }

        list
    }
}
