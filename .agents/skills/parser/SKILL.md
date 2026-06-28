---
name: parser
description: Experte für den eigenen WikiText/Markdown Compiler-Parser im mvc/ Verzeichnis.
---

# Parser Skill (MeinCMS - wissen-ahrensburg.de)

Du bist der Experte für den Custom WikiText/Markdown Compiler-Parser in diesem Projekt.

## Verantwortlichkeiten & Architektur
- **Fokus:** Das `mvc/` Verzeichnis, insbesondere die Klassen, die für das Parsen von WikiText und Markdown zuständig sind.
- **Sicherheit:** Der Output muss strikt mit `HtmlSanitizer` gesichert werden. Verhindere XSS/CSRF/CSP Schwachstellen. Es sind keine Inline-Skripte erlaubt!
- **Datenmodell:** Berücksichtige die DB-Modelle `WikiArtikel`, `WikiArtikelVersion`, `WikiNamespace`, und `WikiCategory`.
- **Inhaltsfelder:** Beim Speichern im Backend muss das jeweils nicht genutzte Inhaltsfeld (Markdown vs. MediaWiki) explizit geleert werden.
- **Backup & HTML:** Das generierte HTML wird nicht im Backup gespeichert. Nach Änderungen am Parser oder einem Import muss zwingend der Befehl `dotnet run --project backup -- repair` ausgeführt werden, um das HTML aus dem Markdown/WikiText neu zu generieren.

## Typische Workflow-Befehle
- Tests für den Parser ausführen: `dotnet test mvc.Tests`
- Nach Parser-Anpassungen (Repair-Job): `dotnet run --project backup -- repair`
