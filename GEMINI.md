# MeinCMS - Projektübersicht

MeinCMS ist ein leichtgewichtiges Content-Management-System (CMS) mit Wiki-ähnlicher Funktionalität, entwickelt mit ASP.NET Core MVC 10.0 und PostgreSQL.

## Architektur

- **`mvc/`**: Die Haupt-Webanwendung, die das MVC-Muster implementiert.
  - **Controller**: Verwalten das Routing und die Benutzerinteraktion (z. B. `PageController.cs` für Wiki-Seiten).
  - **Models**: Definieren die Datenstrukturen (`WikiArtikel`, `WikiArtikelVersion`).
  - **Data**: Entity Framework Core `ApplicationDbContext` und Migrationen.
  - **Identity**: Integrierte ASP.NET Core Identity für die Authentifizierung.
- **`Services/`**: Eine Klassenbibliothek für gemeinsame Geschäftslogik und Dienste.
- **`UserAdmin/`**: Eine Konsolenanwendung für administrative Aufgaben, wie das Erstellen und Auflisten von Benutzern.

## Technologien

- **Framework**: .NET 10.0 (ASP.NET Core)
- **Datenbank**: PostgreSQL (via Npgsql EF Core Provider) oder SQLite (vorhanden über Pakete und `app.db`). PostgreSQL ist der aktuelle Standard in `Program.cs`.
- **ORM**: Entity Framework Core
- **Authentifizierung**: ASP.NET Core Identity
- **UI**: Razor Views, Bootstrap, jQuery

## Erstellen und Ausführen

### Voraussetzungen

- [.NET 10.0 SDK](https://dotnet.microsoft.com/download/dotnet/10.0)
- [PostgreSQL](https://www.postgresql.org/)

### Einrichtung

1.  **Umgebung konfigurieren**:
    Kopieren Sie die Vorlagen der Konfigurationsdateien an ihre tatsächlichen Speicherorte:
    ```bash
    cp mvc/_appsettings.Development.json mvc/appsettings.Development.json
    cp mvc/_appsettings.json mvc/appsettings.json
    ```
    Aktualisieren Sie `mvc/appsettings.json` mit Ihrem PostgreSQL-Verbindungsstring.

2.  **Datenbank-Setup**:
    Erstellen Sie die Datenbank und vergeben Sie die Rechte (Beispiel für Linux):
    ```bash
    sudo -u postgres -i
    createdb -E UTF8 -O ihr_benutzer mvc
    psql -d mvc -c "GRANT ALL PRIVILEGES ON DATABASE mvc TO ihr_benutzer"
    exit
    ```

3.  **Migrationen anwenden**:
    Führen Sie die Migrationen vom Projekt-Root aus:
    ```bash
    dotnet ef database update --project mvc
    ```

### Ausführen der Anwendungen

- **Webanwendung**:
  ```bash
  dotnet run --project mvc
  ```
- **Benutzeradministrations-Tool**:
  ```bash
  dotnet run --project UserAdmin
  ```

## Entwicklungskonventionen

- **Projektstruktur**: Folgt dem Standardlayout von ASP.NET Core MVC.
- **Benennungskonventionen**: PascalCase für Klassen, Methoden und Eigenschaften; camelCase für lokale Variablen und private Felder.
- **Wiki-Routing**: Der `PageController` verarbeitet dynamische Wiki-Pfade mithilfe von Catch-all-Routenparametern (`{*slug}`).
- **Datenbank-Updates**: Verwenden Sie immer Entity Framework Core-Migrationen für Schemaänderungen.
- **Gemeinsame Logik**: Bevorzugen Sie es, wiederverwendbare Geschäftslogik im Projekt `Services` zu platzieren.

## TODO / Roadmap

- [ ] Implementierung einer robusten Bearbeitung und Versionierung von Wiki-Seiten.
- [ ] Hinzufügen von Unit- und Integrationstests.
- [ ] Implementierung eines Datei-/Bildupload-Systems.
- [ ] Unterstützung für Themes oder CSS-Anpassungen.
