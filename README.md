# wissen-ahrensburg.de - MeinCMS

Ein Mandantenfähiges (Multi-Tenancy) Wiki-CMS System entwickelt mit ASP.NET Core 10 und PostgreSQL.

## Features

- **Mandantenfähigkeit**: Unterstützung für mehrere Domains/Mandanten (z.B. `wissen-ahrensburg.de` für Stadtinhalte und `doc.wissen-ahrensburg.de` für technische Dokumentation).
- **MediaWiki Support**: Hochperformanter, compiler-basierter Parser für MediaWiki WikiText (Tokenizer -> AST -> Serializer). Optimiert für stabile Textverarbeitung und korrekte HTML-Struktur.
- **Sicherheit**:
  - Erfolgreich bestandenes Sicherheits-Audit.
  - Strikte Content-Security-Policy (CSP) **ohne Inline-Skripte**.
  - `HtmlSanitizer` gegen XSS.
  - CSRF-Schutz (Antiforgery-Token) global aktiviert.
  - Gehärtete Identity-Password-Policies und Account-Lockout.
- **Wiki-Funktionalität**: Markdown- und WikiText-basierte Artikel mit voller Versionierung, Diff-Ansicht und Volltextsuche.
- **Backup & Migration**: XML/YAML-Export/Import Tools inklusive HTML-Repair-Modus.
- **Performance**: Unterstützung für Unix Domain Sockets für Nginx-Integration.
- **Dokumentation**: Umfangreiche Anleitungen für Produktion und Backup im Ordner `Anleitung/`.

## Schnellstart

1.  **Datenbank**: PostgreSQL konfigurieren.
2.  **Appsettings**: `mvc/appsettings.json` anpassen.
3.  **Migrationen**: `dotnet ef database update --project mvc`
4.  **Starten**: `dotnet run --project mvc`

## Lizenz

Dieses Projekt ist unter der **GNU Affero General Public License v3.0 (AGPL-3.0)** lizenziert. Weitere Details finden Sie in der [LICENSE](LICENSE) Datei.

## KI-Assistenz & Subagenten (Antigravity)

Dieses Projekt ist für die Zusammenarbeit mit KI-Assistenten (insbesondere Antigravity) optimiert. Um den Kontext- und Tokenverbrauch extrem niedrig zu halten und den Hauptchat schnell und übersichtlich zu gestalten, wurden spezialisierte **Subagenten (Unteragenten)** konfiguriert. 

Diese Agenten arbeiten unsichtbar im Hintergrund. Sie führen zeitaufwändige Recherchen, Code-Änderungen und Tests durch und melden nur das fertige Ergebnis zurück.

### Verfügbare Subagenten:
- **`meincms_worker`**: Der Allrounder. Zuständig für allgemeine Dateiänderungen, Bugfixes, das Ausführen von `.NET`-Befehlen und Tests.
- **`parser_specialist`**: Der Experte für die komplexe Wiki-Syntax. Spezialisiert auf die Architektur von Tokenizern, AST-Buildern und HTML-Serializern. Löst spezifische Rendering-Bugs für MediaWiki und Markdown.

### Wie benutze ich sie?
Als Nutzer kannst du den Hauptagenten einfach anweisen, eine Aufgabe zu delegieren. Zum Beispiel:
> *"Schick den `meincms_worker` los, um das Kontaktformular anzupassen."*
Oder:
> *"Lass den `parser_specialist` analysieren, warum Tabellen im Wiki falsch gerendert werden."*

Bei sehr großen Aufgaben entscheidet der Hauptagent oft sogar selbstständig, die Arbeit an einen Subagenten abzugeben, um deine Tokens zu schonen!
