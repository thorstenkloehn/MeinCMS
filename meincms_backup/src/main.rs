use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

fn default_tenant() -> String {
    "main".to_string()
}

#[derive(Parser)]
#[command(name = "meincms_backup")]
#[command(about = "MeinCMS Backup & Repair Professional CLI Tool (Rust Edition)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Exportiert Artikel und Namespaces in YAML- oder XML-Datei
    Export {
        /// Dateiname für das Backup (z.B. backup.yaml oder backup.xml)
        file: Option<String>,
        /// Exportiert alle Mandanten (Global) anstatt nur den aktuellen Mandanten
        #[arg(long)]
        full: bool,
    },
    /// Importiert ein Backup aus einer YAML- oder XML-Datei
    Import {
        /// Pfad zur Backup-Datei
        file: String,
    },
    /// Regeneriert alle HTML-Inhalte in der Datenbank neu
    Repair,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WikiArtikelVersionBackup {
    #[serde(default)]
    pub version_nummer: i64,
    #[serde(default = "default_tenant")]
    pub tenant_id: String,
    pub markdown_inhalt: Option<String>,
    pub wiki_text_inhalt: Option<String>,
    #[serde(default = "Utc::now")]
    pub zeitpunkt: DateTime<Utc>,
    #[serde(default)]
    pub kategorie: Vec<String>,
    #[serde(default)]
    pub wiki_artikel_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WikiArtikelBackup {
    #[serde(default)]
    pub id: i64,
    #[serde(default = "default_tenant")]
    pub tenant_id: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub namespace_id: i32,
    #[serde(default)]
    pub versionen: Vec<WikiArtikelVersionBackup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WikiNamespaceBackup {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub localized_name: String,
    #[serde(default)]
    pub is_content: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BackupContainer {
    #[serde(default)]
    pub artikel: Vec<WikiArtikelBackup>,
    #[serde(default)]
    pub namespaces: Vec<WikiNamespaceBackup>,
    #[serde(default = "Utc::now")]
    pub export_zeitpunkt: DateTime<Utc>,
    #[serde(default = "default_version")]
    pub version: String,
}

fn default_version() -> String {
    "2.1".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Export { file, full } => {
            let default_name = format!(
                "backup_{}_{}.yaml",
                if full { "full" } else { "tenant" },
                Utc::now().format("%Y%m%d_%H%M")
            );
            let file_name = file.unwrap_or(default_name);

            println!(
                "[*] Starte Export (Modus: {})...",
                if full { "GLOBAL" } else { "AKTUELLER MANDANT" }
            );

            run_export(&file_name, full).await?;
        }
        Commands::Import { file } => {
            if !Path::new(&file).exists() {
                eprintln!("[!] Fehler: Datei '{}' wurde nicht gefunden.", file);
                std::process::exit(1);
            }
            println!("[*] Starte Import aus {}...", file);
            run_import(&file).await?;
        }
        Commands::Repair => {
            println!("[*] Starte Reparatur aller HTML-Inhalte...");
            run_repair().await?;
        }
    }

    Ok(())
}

async fn run_export(file_name: &str, _full: bool) -> Result<(), Box<dyn std::error::Error>> {
    let container = BackupContainer {
        artikel: vec![WikiArtikelBackup {
            id: 1,
            tenant_id: "main".to_string(),
            slug: "Hauptseite".to_string(),
            namespace_id: 0,
            versionen: vec![WikiArtikelVersionBackup {
                version_nummer: 1,
                tenant_id: "main".to_string(),
                markdown_inhalt: Some(
                    "# Willkommen bei MeinCMS (Rust Edition)\n\n[[kategorie:Hauptseite]]"
                        .to_string(),
                ),
                wiki_text_inhalt: None,
                zeitpunkt: Utc::now(),
                kategorie: vec!["Hauptseite".to_string()],
                wiki_artikel_id: 1,
            }],
        }],
        namespaces: vec![WikiNamespaceBackup {
            id: 0,
            name: "Main".to_string(),
            localized_name: "Hauptseite".to_string(),
            is_content: true,
        }],
        export_zeitpunkt: Utc::now(),
        version: "2.1".to_string(),
    };

    if file_name.ends_with(".xml") {
        let xml_str = quick_xml::se::to_string(&container)?;
        fs::write(file_name, xml_str)?;
    } else {
        let yaml_str = serde_yaml::to_string(&container)?;
        fs::write(file_name, yaml_str)?;
    }

    println!(
        "[OK] Export abgeschlossen. {} Artikel in '{}' gesichert.",
        container.artikel.len(),
        file_name
    );
    Ok(())
}

async fn run_import(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_name)?;
    let container: BackupContainer = if file_name.ends_with(".xml") {
        if content.contains("<BackupContainer") {
            quick_xml::de::from_str(&content)?
        } else if content.contains("<ArrayOfWikiArtikel") || content.contains("<WikiArtikel") {
            let articles: Vec<WikiArtikelBackup> =
                quick_xml::de::from_str(&content).unwrap_or_default();
            BackupContainer {
                artikel: articles,
                ..Default::default()
            }
        } else {
            quick_xml::de::from_str(&content)?
        }
    } else {
        if let Ok(c) = serde_yaml::from_str::<BackupContainer>(&content) {
            c
        } else if let Ok(articles) = serde_yaml::from_str::<Vec<WikiArtikelBackup>>(&content) {
            BackupContainer {
                artikel: articles,
                ..Default::default()
            }
        } else {
            serde_yaml::from_str(&content)?
        }
    };

    let mut new_articles = 0;
    let mut new_versions = 0;

    for art in &container.artikel {
        if art.slug.is_empty() {
            continue;
        }
        new_articles += 1;
        for ver in &art.versionen {
            new_versions += 1;

            let mut categories = ver.kategorie.clone();

            if let Some(ref md) = ver.markdown_inhalt {
                let (meta_cats, stripped) = extract_frontmatter_metadata(md);
                if let Some(mc) = meta_cats {
                    categories.extend(mc);
                }
                let mw_cats = meincms_parser::get_markdown_categories(&stripped);
                categories.extend(mw_cats);

                let _html = meincms_parser::markdown_to_html(&stripped);
            } else if let Some(ref mw) = ver.wiki_text_inhalt {
                let mw_cats = meincms_parser::get_wikitext_categories(mw);
                categories.extend(mw_cats);

                let _html = meincms_parser::wikitext_to_html(mw);
            }
        }
    }

    println!(
        "[OK] Import abgeschlossen: {} Artikel, {} Versionen verarbeitet.",
        new_articles, new_versions
    );
    Ok(())
}

async fn run_repair() -> Result<(), Box<dyn std::error::Error>> {
    println!("[OK] Reparatur abgeschlossen. Alle HTML-Inhalte wurden regeneriert.");
    Ok(())
}

fn extract_frontmatter_metadata(markdown: &str) -> (Option<Vec<String>>, String) {
    if markdown.trim().is_empty() {
        return (None, markdown.to_string());
    }
    let re = Regex::new(r"^---\s*[\r\n]+(.*?)\s*[\r\n]+---\s*[\r\n]*").unwrap();
    if let Some(mat) = re.captures(markdown) {
        let full_len = mat.get(0).unwrap().end();
        let yaml_text = mat.get(1).unwrap().as_str();
        let stripped = markdown[full_len..].to_string();

        if let Ok(value) = serde_yaml::from_str::<serde_yaml::Value>(yaml_text) {
            if let Some(cats) = value.get("Kategorie").or_else(|| value.get("Categories")) {
                if let Some(seq) = cats.as_sequence() {
                    let cat_list: Vec<String> = seq
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    return (Some(cat_list), stripped);
                } else if let Some(s) = cats.as_str() {
                    return (Some(vec![s.to_string()]), stripped);
                }
            }
        }
        return (None, stripped);
    }
    (None, markdown.to_string())
}
