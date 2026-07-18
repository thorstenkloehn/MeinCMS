# wissen-ahrensburg.de (MeinCMS)
CMS mit Wiki-Funktion & Multi-Tenancy (100% Rust Workspace Edition, PostgreSQL).

## Architektur & Tech (Rust Workspace)
- **meincms_parser/**: Hochleistungs Markdown & MediaWiki Parser Crate (mit C-FFI Exporten).
- **meincms_web/**: Axum 0.7 Webanwendung. Multi-Tenancy via Hostname, Maud Templating, Admin-Auth, No-Cache Middleware.
- **meincms_backup/**: Backup & Repair CLI Tool (YAML/XML Ex-/Import & Repair).
- **meincms_admin/**: User & Admin Management CLI Tool (Argon2 Password Hashing).
- **Tech**: Rust 1.80+, Tokio, Axum, Maud, SQLx, PostgreSQL, Argon2.
- **Lizenz**: AGPL-3.0.

## Befehle (Rust)
- Workspace Check & Test: `cargo check` & `cargo test --workspace`
- Web Backend: `cargo run -p meincms_web`
- Backup CLI: `cargo run -p meincms_backup -- [export|import|repair]`
- Admin CLI: `cargo run -p meincms_admin`

## Konventionen & Fehlervermeidung
- Mandant "main" (Standard), "doc" (Technik). Automatische Filterung nach `TenantId`.
- Markdown & MediaWiki strikt gegen XSS/CSRF gesichert, keine Inline-Skripte, No-Cache-Middleware.
- Editor-Toggling: `style.display` via JS (`DOMContentLoaded`) nutzen, nicht nur CSS-Klassen.
- Backend: Beim Speichern das jeweils nicht genutzte Inhaltsfeld (Markdown/MediaWiki) explizit leeren.
- DB-Modell: `WikiArtikel`, `WikiArtikelVersion`, `WikiNamespace`, `WikiCategory`.
- Backup speichert kein generiertes HTML, stattdessen `repair` nach Import/Parser-Änderung ausführen.
