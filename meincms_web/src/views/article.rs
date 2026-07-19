use maud::{html, Markup};

use crate::models::{WikiArtikelVersion, WikiArtikelWithVersion};
use crate::views::layout::render_base_layout;

pub fn render_article_page(
    slug: &str,
    tenant_id: &str,
    article: &WikiArtikelWithVersion,
) -> Markup {
    let latest = article.latest_version.as_ref();
    let html_body = latest
        .and_then(|v| v.html_inhalt.as_deref())
        .unwrap_or("<p><em>Dieser Artikel hat noch keinen Inhalt.</em></p>");

    let categories = latest.map(|v| v.kategorie.as_slice()).unwrap_or(&[]);

    let content = html! {
        article class="card" {
            header class="article-header" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                h1 class="wiki-article-title" id="article-heading" { (slug) }
                div class="article-actions" style="display: flex; gap: 0.75rem;" {
                    a href={ "/edit/" (slug) } class="btn" id="edit-btn" { "✏️ Bearbeiten" }
                    a href={ "/history/" (slug) } class="btn" style="background: var(--bg-primary); border: 1px solid var(--border-color);" { "📜 Historie" }
                }
            }

            div class="wiki-content" id="article-body" {
                (maud::PreEscaped(html_body))
            }

            @if !categories.is_empty() {
                div class="article-categories" style="margin-top: 2rem; padding-top: 1rem; border-top: 1px solid var(--border-color);" {
                    strong style="color: var(--text-secondary); margin-right: 0.75rem;" { "Kategorien:" }
                    @for cat in categories {
                        a href={ "/kategorie/" (cat) } class="category-tag" { (cat) }
                    }
                }
            }
        }
    };

    render_base_layout(slug, tenant_id, content)
}

pub fn render_edit_page(
    slug: &str,
    tenant_id: &str,
    existing_version: Option<&WikiArtikelVersion>,
) -> Markup {
    let initial_syntax = existing_version
        .and_then(|v| v.wiki_text_inhalt.as_ref())
        .map(|_| "mediawiki")
        .unwrap_or("markdown");

    let markdown_val = existing_version
        .and_then(|v| v.markdown_inhalt.as_deref())
        .unwrap_or("");
    let mediawiki_val = existing_version
        .and_then(|v| v.wiki_text_inhalt.as_deref())
        .unwrap_or("");
    let kategorien_val = existing_version
        .map(|v| v.kategorie.join(", "))
        .unwrap_or_default();

    let content = html! {
        div class="card" {
            h1 class="wiki-article-title" { "Seite bearbeiten: " (slug) }

            form action={ "/save/" (slug) } method="post" id="edit-form" style="display: flex; flex-direction: column; gap: 1.5rem;" {
                div class="form-group" {
                    label style="display: block; font-weight: 600; margin-bottom: 0.5rem;" { "Syntax wählen:" }
                    select name="syntax" id="syntax-select" style="background: var(--bg-primary); color: white; border: 1px solid var(--border-color); padding: 0.5rem 1rem; border-radius: 8px; font-size: 1rem;" {
                        option value="markdown" selected[initial_syntax == "markdown"] { "Markdown Syntax" }
                        option value="mediawiki" selected[initial_syntax == "mediawiki"] { "MediaWiki Syntax" }
                    }
                }

                div id="markdown-container" class="form-group" style="display: block;" {
                    label style="display: block; font-weight: 600; margin-bottom: 0.5rem;" { "Markdown Inhalt:" }
                    textarea name="markdown_inhalt" id="markdown-textarea" rows="16" style="width: 100%; background: var(--bg-primary); color: white; border: 1px solid var(--border-color); padding: 1rem; border-radius: 8px; font-family: monospace; font-size: 1rem;" {
                        (markdown_val)
                    }
                }

                div id="mediawiki-container" class="form-group" style="display: none;" {
                    label style="display: block; font-weight: 600; margin-bottom: 0.5rem;" { "MediaWiki Inhalt:" }
                    textarea name="wiki_text_inhalt" id="mediawiki-textarea" rows="16" style="width: 100%; background: var(--bg-primary); color: white; border: 1px solid var(--border-color); padding: 1rem; border-radius: 8px; font-family: monospace; font-size: 1rem;" {
                        (mediawiki_val)
                    }
                }

                div class="form-group" {
                    label style="display: block; font-weight: 600; margin-bottom: 0.5rem;" { "Kategorien (kommagetrennt):" }
                    input type="text" name="kategorien_raw" value=(kategorien_val) placeholder="Hauptseite, Dokumentation" style="width: 100%; background: var(--bg-primary); color: white; border: 1px solid var(--border-color); padding: 0.75rem 1rem; border-radius: 8px;" ;
                }

                div style="display: flex; gap: 1rem; margin-top: 1rem;" {
                    button type="submit" class="btn" id="save-btn" { "💾 Speichern" }
                    a href={ "/wiki/" (slug) } class="btn" style="background: var(--bg-primary); border: 1px solid var(--border-color);" { "Abbrechen" }
                }
            }

            // JavaScript with DOMContentLoaded and style.display toggling as required by user rules!
            script { (maud::PreEscaped(r#"
document.addEventListener("DOMContentLoaded", function() {
    var syntaxSelect = document.getElementById("syntax-select");
    var markdownContainer = document.getElementById("markdown-container");
    var mediawikiContainer = document.getElementById("mediawiki-container");

    function updateEditorVisibility() {
        if (syntaxSelect.value === "mediawiki") {
            markdownContainer.style.display = "none";
            mediawikiContainer.style.display = "block";
        } else {
            markdownContainer.style.display = "block";
            mediawikiContainer.style.display = "none";
        }
    }

    syntaxSelect.addEventListener("change", updateEditorVisibility);
    updateEditorVisibility();
});
"#)) }
        }
    };

    render_base_layout(&format!("Bearbeiten: {}", slug), tenant_id, content)
}

pub fn render_history_page(slug: &str, tenant_id: &str, versions: &[WikiArtikelVersion]) -> Markup {
    let content = html! {
        div class="card" {
            h1 class="wiki-article-title" { "Versionshistorie: " (slug) }

            @if versions.is_empty() {
                p { "Keine Historie vorhanden." }
            } @else {
                table style="width: 100%; border-collapse: collapse; margin-top: 1.5rem;" {
                    thead {
                        tr style="border-bottom: 2px solid var(--border-color); text-align: left;" {
                            th style="padding: 0.75rem;" { "Version #" }
                            th style="padding: 0.75rem;" { "Zeitpunkt" }
                            th style="padding: 0.75rem;" { "Kategorien" }
                            th style="padding: 0.75rem;" { "Aktion" }
                        }
                    }
                    tbody {
                        @for v in versions {
                            tr style="border-bottom: 1px solid var(--border-color);" {
                                td style="padding: 0.75rem; font-weight: 600;" { "#" (v.version_nummer) }
                                td style="padding: 0.75rem;" { (v.zeitpunkt.format("%d.%m.%Y %H:%M:%S").to_string()) }
                                td style="padding: 0.75rem;" { (v.kategorie.join(", ")) }
                                td style="padding: 0.75rem;" {
                                    a href={ "/version/" (v.version_nummer) } class="btn" style="padding: 0.4rem 0.8rem; font-size: 0.85rem;" { "👁️ Ansehen" }
                                }
                            }
                        }
                    }
                }
            }

            div style="margin-top: 1.5rem;" {
                a href={ "/wiki/" (slug) } class="btn" style="background: var(--bg-primary); border: 1px solid var(--border-color);" { "Zurück zum Artikel" }
            }
        }
    };

    render_base_layout(&format!("Historie: {}", slug), tenant_id, content)
}
