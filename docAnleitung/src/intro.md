# Einleitung

Willkommen bei der Installations- und Betriebsanleitung für **MeinCMS** (wissen-ahrensburg.de).

Dieses System ist ein leichtgewichtiges Content-Management-System (CMS) mit Wiki-ähnlicher Funktionalität und nativer **Multi-Tenancy** (Mandantenfähigkeit). Es basiert auf **ASP.NET Core MVC 10.0** und **PostgreSQL**.

## Architektur-Überblick

- **Multi-Tenancy:** Das System unterstützt mehrere Mandanten (Tenants) auf derselben Instanz (z. B. `wissen-ahrensburg.de` für lokale Stadtinhalte und `doc.wissen-ahrensburg.de` für die technische Dokumentation). Die Trennung erfolgt dynamisch pro Request über den Hostnamen und wird über globale EF Core-Queryfilter in der Datenbank abgebildet.
- **Zwei Syntax-Welten:** Artikel können entweder in **MediaWiki-Syntax (WikiText)** oder in **Markdown** verfasst werden. Beim Wechsel im Editor findet eine automatische bidirektionale Übersetzung statt.
- **Hardening (Sicherheit):** Strikte Einhaltung von CSP (keine Inline-Skripte), global aktivierter CSRF-Schutz, HTML-Bereinigung via `HtmlSanitizer` sowie Passwort-Lockout-Richtlinien.

Diese Dokumentation führt Sie durch:
1. Das Einrichten eines lokalen Entwicklungsrechners.
2. Die Installation und Konfiguration auf einem Ubuntu/Debian Linux Produktions-Server.
3. Die fortlaufende Wartung (Backup-Erstellung, System-Updates, HTML-Reparatur nach Parser-Fixes).
