# Entwicklungsumgebung einrichten

Diese Anleitung beschreibt die Schritte zur Einrichtung eines lokalen Entwicklungsrechners für MeinCMS.

## Voraussetzungen

Stellen Sie sicher, dass folgende Software auf Ihrem System installiert ist:

1. **.NET 10.0 SDK** (oder neuer)
2. **PostgreSQL** (lokal installiert oder über Docker/Cloud-Dienst erreichbar)
3. **Git** zum Klonen und Verwalten des Codes

## 1. Konfiguration einrichten

Kopieren Sie die Vorlage für die Anwendungskonfiguration in das entsprechende Verzeichnis:

```bash
cp mvc/_appsettings.json mvc/appsettings.json
```

Öffnen Sie `mvc/appsettings.json` und passen Sie den Verbindungsstring für Ihre lokale PostgreSQL-Datenbank unter `ConnectionStrings:DefaultConnection` an:

```json
{
  "ConnectionStrings": {
    "DefaultConnection": "Host=localhost;Database=meincms_dev;Username=postgres;Password=dein_passwort"
  },
  ...
}
```

Stellen Sie sicher, dass die Mandanten-Konfiguration (TenantConfig) in der JSON-Datei Ihren lokalen Hostnamen bzw. Domains entspricht, falls Sie die Multi-Tenancy-Funktionen lokal testen möchten.

## 2. Datenbank aktualisieren

Wenden Sie alle Entity Framework Core-Migrationen auf Ihre lokale Datenbank an, um die Tabellenstruktur zu erstellen:

```bash
dotnet ef database update --project mvc
```

*Hinweis: Wenn Sie `dotnet-ef` noch nicht global installiert haben, können Sie dies mit `dotnet tool install --global dotnet-ef` tun.*

## 3. Anwendung bauen und ausführen

Starten Sie die Webanwendung aus dem Wurzelverzeichnis des Projekts:

```bash
dotnet run --project mvc
```

Alternativ können Sie die Anwendung so starten, dass ausstehende Migrationen beim Start automatisch ausgeführt werden:

```bash
dotnet run --project mvc -- --migrate
```

Die Anwendung ist standardmäßig unter `http://localhost:5000` (bzw. der in `Properties/launchSettings.json` definierten Portadresse) erreichbar.

## 4. Tests ausführen

Das Projekt verfügt über xUnit-Testprojekte für die verschiedenen Komponenten. Sie können alle Tests wie folgt ausführen:

```bash
# Alle Tests ausführen
dotnet test

# Nur die Parser-Tests für Markdown ausführen
dotnet test Mardown.Tests

# Nur die Parser-Tests für MediaWiki ausführen
dotnet test mvc.Tests --filter "FullyQualifiedName~MediaWikiParserTests"
```
