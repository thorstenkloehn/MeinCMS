# wissen-ahrensburg.de (MeinCMS)
CMS mit Wiki-Funktion & Multi-Tenancy (100% Rust Workspace Edition, PostgreSQL).

## Architektur & Tech (Rust Workspace)
- **meincms_parser/**: Hochleistungs Markdown & MediaWiki Parser Crate (mit C-FFI Exporten).
- **meincms_web/**: Axum 0.7 Webanwendung. Multi-Tenancy via Hostname, Maud Templating, Admin-Auth, No-Cache Middleware.
- **meincms_backup/**: Backup & Repair CLI Tool (YAML/XML Ex-/Import & Repair).
- **meincms_admin/**: User & Admin Management CLI Tool (Argon2 Password Hashing).
- **Tech**: Rust 1.80+, Tokio, Axum, Maud, SQLx, PostgreSQL, Argon2.
- **Lizenz**: AGPL-3.0.

## Befehle (Rust & Qualitätssicherung)
- Workspace Check & Test: `cargo check` & `cargo test --workspace`
- Linter & Formatierung: `cargo clippy` & `cargo fmt --check`
- Dokumentation (mdBook): `mdbook build docs`
- Web Backend: `cargo run -p meincms_web`
- Backup CLI: `cargo run -p meincms_backup -- [export|import|repair]`
- Admin CLI: `cargo run -p meincms_admin`

## Konventionen & Regeln für KI-Agenten
- **Git & Sicherheit:** NIEMALS `.env` oder Passwörter einchecken. `!.env.example` als Referenz nutzen.
- **Produktion / Unix Socket:** PostgreSQL-Verbindung bevorzugt über Unix Domain Socket (`DATABASE_URL="postgres://user:pass@/var/run/postgresql/meincms"`). Webserver via `UNIX_SOCKET`.
- **Mandanten:** Multi-Tenancy mit Filterung nach `TenantId` ("main" = Standard, "doc" = Technik).
- **Sicherheit:** Markdown & MediaWiki strikt gegen XSS/CSRF sichern, keine Inline-Skripte. No-Cache-Header erzwingen.
- **Frontend / Maud:** Editor-Toggling via Vanilla JS (`DOMContentLoaded` & `style.display`), nicht rein über CSS-Klassen.
- **Backend:** Beim Speichern von Wiki-Artikeln das jeweils nicht genutzte Inhaltsfeld (Markdown/MediaWiki) explizit leeren.
- **DB-Modell:** `WikiArtikel`, `WikiArtikelVersion`, `WikiNamespace`, `WikiCategory`.
- **Backup & Repair:** Nach Import oder Parser-Änderungen stets `repair` über `meincms_backup` ausführen.
- **Dokumentation & mdBook:** Bei Konfigurations- oder Architekturänderungen stets die Doku in `docs/` aktualisieren und danach `mdbook build docs` ausführen.
