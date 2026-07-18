use maud::{html, Markup, DOCTYPE};

pub fn render_base_layout(title: &str, tenant_id: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="de" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="description" content="MeinCMS - Leichtgewichtiges Multi-Tenant Wiki (Rust Edition)";
                title { (title) " - MeinCMS (" (tenant_id) ")" }
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="";
                link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet";
                style { (raw_css()) }
            }
            body {
                header class="site-header" {
                    div class="header-container" {
                        a href="/" class="brand-logo" {
                            span class="logo-icon" { "⚡" }
                            span class="logo-text" { "MeinCMS" }
                            span class="tenant-badge" { (tenant_id) }
                        }
                        nav class="site-nav" {
                            a href="/" class="nav-link" { "Hauptseite" }
                            a href="/alle" class="nav-link" { "Alle Artikel" }
                            a href="/kategorien" class="nav-link" { "Kategorien" }
                        }
                        form action="/suche" method="get" class="search-form" {
                            input type="text" name="q" placeholder="Wiki durchsuchen..." id="search-input" class="search-input";
                            button type="submit" class="search-btn" { "🔍" }
                        }
                    }
                }

                main class="content-wrapper" {
                    (content)
                }

                footer class="site-footer" {
                    div class="footer-container" {
                        p { "© 2026 MeinCMS (Rust Edition) • AGPL-3.0 Lizenz • Mandant: " strong { (tenant_id) } }
                    }
                }
            }
        }
    }
}

fn raw_css() -> &'static str {
    r#"
:root {
    --bg-primary: #0f172a;
    --bg-surface: #1e293b;
    --bg-glass: rgba(30, 41, 59, 0.7);
    --border-color: rgba(255, 255, 255, 0.1);
    --accent-blue: #38bdf8;
    --accent-indigo: #6366f1;
    --text-primary: #f8fafc;
    --text-secondary: #94a3b8;
    --font-main: 'Inter', system-ui, -apple-system, sans-serif;
}

* { box-sizing: border-box; margin: 0; padding: 0; }
body {
    background: var(--bg-primary);
    color: var(--text-primary);
    font-family: var(--font-main);
    line-height: 1.6;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
}

.site-header {
    background: var(--bg-glass);
    backdrop-filter: blur(12px);
    border-bottom: 1px solid var(--border-color);
    position: sticky;
    top: 0;
    z-index: 100;
}
.header-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 1rem 1.5rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1.5rem;
}
.brand-logo {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    text-decoration: none;
    color: var(--text-primary);
    font-weight: 700;
    font-size: 1.25rem;
}
.tenant-badge {
    background: linear-gradient(135deg, var(--accent-blue), var(--accent-indigo));
    color: #fff;
    font-size: 0.75rem;
    padding: 0.2rem 0.6rem;
    border-radius: 999px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

.site-nav {
    display: flex;
    gap: 1.5rem;
}
.nav-link {
    color: var(--text-secondary);
    text-decoration: none;
    font-weight: 500;
    transition: color 0.2s ease;
}
.nav-link:hover { color: var(--accent-blue); }

.search-form { display: flex; gap: 0.5rem; }
.search-input {
    background: rgba(15, 23, 42, 0.6);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 0.5rem 1rem;
    color: var(--text-primary);
    outline: none;
}
.search-btn {
    background: var(--accent-indigo);
    border: none;
    border-radius: 8px;
    padding: 0.5rem 0.75rem;
    cursor: pointer;
    color: white;
}

.content-wrapper {
    max-width: 1200px;
    margin: 2rem auto;
    padding: 0 1.5rem;
    flex: 1;
    width: 100%;
}

.card {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 16px;
    padding: 2rem;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.3);
}

.btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    background: linear-gradient(135deg, var(--accent-blue), var(--accent-indigo));
    color: #fff;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    text-decoration: none;
    font-weight: 600;
    border: none;
    cursor: pointer;
    transition: transform 0.2s ease, opacity 0.2s ease;
}
.btn:hover { transform: translateY(-2px); opacity: 0.95; }

.wiki-article-title {
    font-size: 2.25rem;
    margin-bottom: 1.5rem;
    background: linear-gradient(135deg, #fff, var(--text-secondary));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
}
.wiki-content { line-height: 1.8; font-size: 1.05rem; }
.wiki-content p { margin-bottom: 1.25rem; }
.wiki-content h1, .wiki-content h2, .wiki-content h3 { margin: 1.5rem 0 1rem 0; color: var(--accent-blue); }

.category-tag {
    display: inline-block;
    background: rgba(56, 189, 248, 0.15);
    color: var(--accent-blue);
    padding: 0.25rem 0.75rem;
    border-radius: 999px;
    text-decoration: none;
    font-size: 0.85rem;
    margin-right: 0.5rem;
}

.site-footer {
    border-top: 1px solid var(--border-color);
    padding: 2rem 0;
    text-align: center;
    color: var(--text-secondary);
    font-size: 0.9rem;
    margin-top: 3rem;
}
"#
}
