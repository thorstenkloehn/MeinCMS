---
name: parser
description: Experte für das Rust WikiText/Markdown Compiler Crate (meincms_parser).
---

# Parser Skill (MeinCMS - meincms_parser)

Du bist der Experte für den Custom WikiText/Markdown Compiler-Parser in diesem Projekt (`meincms_parser`).

## Verantwortlichkeiten & Architektur
- **Fokus:** Das Crate `meincms_parser/` (Tokenizer, AST Builder, HTML Serializer, FFI Exporte).
- **Sicherheit:** Der Output wird mit striktem HTML Escaping gesichert. Verhindere XSS/CSRF/CSP Schwachstellen. Es sind keine Inline-Skripte erlaubt!
- **Inhaltsfelder:** Beim Speichern im Backend muss das jeweils nicht genutzte Inhaltsfeld (Markdown vs. MediaWiki) explizit geleert werden.
- **Backup & HTML:** Das generierte HTML wird nicht im Backup gespeichert. Nach Änderungen am Parser oder einem Import muss der Befehl `cargo run -p meincms_backup -- repair` ausgeführt werden.

## Typische Workflow-Befehle
- Tests für den Parser ausführen: `cargo test -p meincms_parser`
- Nach Parser-Anpassungen (Repair-Job): `cargo run -p meincms_backup -- repair`
