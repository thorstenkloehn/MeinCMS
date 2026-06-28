# wissen-ahrensburg.de (MeinCMS)
CMS mit Wiki-Funktion & Multi-Tenancy (ASP.NET Core 10.0, PostgreSQL).

## Architektur & Tech
- **mvc/**: Webanwendung. Multi-Tenancy via Hostname (`ApplicationDbContext`). Eigene WikiText/Markdown Compiler-Parser.
- **Services/**: Logik (`PageService`, `TenantService`).
- **UserAdmin/**: Admin-Verwaltung (CLI).
- **backup/**: YAML/XML Ex-/Import & Repair.
- **Tech**: .NET 10.0, PostgreSQL, HtmlSanitizer.
- **Lizenz**: AGPL-3.0.

## Befehle
- Web: `dotnet run --project mvc` (ggf. `-- --migrate`)
- Admin: `dotnet run --project UserAdmin`
- Backup: `dotnet run --project backup -- [export|import|repair]`
- Tests: `dotnet test mvc.Tests`
- DB Update: `dotnet ef database update --project mvc`

## Konventionen & Fehlervermeidung
- Mandant "main" (Standard), "doc" (Technik). Automatische Filterung nach `TenantId`.
- Markdown via `HtmlSanitizer` gesichert, globale CSRF/CSP, keine Inline-Skripte, No-Cache-Middleware.
- Razor: `<!option>` Tags bei C#-Attributen nutzen, um RZ1031 zu vermeiden.
- Editor-Toggling: `style.display` via JS (DOMContentLoaded) nutzen, nicht nur CSS-Klassen.
- Backend: Beim Speichern das jeweils nicht genutzte Inhaltsfeld (Markdown/MediaWiki) explizit leeren.
- DB-Modell: `WikiArtikel`, `WikiArtikelVersion`, `WikiNamespace`, `WikiCategory`.
- Backup speichert kein generiertes HTML, stattdessen `repair` nach Import/Parser-Änderung ausführen.
