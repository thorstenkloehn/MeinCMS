# MeinCMS

MeinCMS ist ein leichtgewichtiges Content-Management-System (CMS) mit Wiki-Funktionalität, entwickelt mit **ASP.NET Core MVC 10.0** und **PostgreSQL**.

## 🛠 Voraussetzungen

Bevor Sie das Projekt starten, müssen folgende Komponenten auf Ihrem System installiert sein:

*   **[.NET 10.0 SDK](https://dotnet.microsoft.com/download/dotnet/10.0)**
*   **[PostgreSQL](https://www.postgresql.org/)** (lokal oder als Docker-Container)
*   **EF Core CLI Tools**: Falls noch nicht installiert, führen Sie diesen Befehl aus:
    ```bash
    dotnet tool install --global dotnet-ef
    ```

## 🚀 Installation und Einrichtung

### 1. Konfiguration vorbereiten
Kopieren Sie die Vorlagendateien für die Konfiguration an ihre tatsächlichen Speicherorte:

```bash
cp mvc/_appsettings.Development.json mvc/appsettings.Development.json
cp mvc/_appsettings.json mvc/appsettings.json
```

*Hinweis: Passen Sie den Connection-String in `mvc/appsettings.json` an Ihre PostgreSQL-Instanz an.*

### 2. Datenbank-Setup (Linux/PostgreSQL)
Erstellen Sie die Datenbank und vergeben Sie die notwendigen Rechte für Ihren Benutzer (Beispiel für den Benutzer `thorsten`):

```bash
sudo -u postgres -i
createdb -E UTF8 -O thorsten mvc
psql -d mvc -c "GRANT ALL PRIVILEGES ON DATABASE mvc TO thorsten"
exit
```

### 3. Datenbank-Migrationen anwenden
Führen Sie die Migrationen vom Projekt-Root aus, um das Tabellenschema zu erstellen:

```bash
dotnet ef database update --project mvc
```

## 💻 Programm starten

### Web-Anwendung (Wiki)
Starten Sie die Hauptanwendung aus dem Root-Verzeichnis:

```bash
dotnet run --project mvc
```
Die Anwendung ist standardmäßig unter `http://localhost:5000` (oder dem in `launchSettings.json` definierten Port) erreichbar.

### Benutzeradministration (UserAdmin)
Für administrative Aufgaben (Benutzer erstellen/auflisten) nutzen Sie das Konsolentool:

```bash
dotnet run --project UserAdmin
```

## 📝 Was noch fehlt (Roadmap)

Folgende Funktionen sind aktuell noch in Planung oder in Arbeit:

- [ ] **Erweiterte Bearbeitung**: Ein WYSIWYG-Editor oder eine bessere Markdown-Integration für Wiki-Seiten.
- [ ] **Dateiverwaltung**: System zum Hochladen und Einbinden von Bildern und Dokumenten.
- [ ] **Tests**: Unit-Tests für das `Services`-Projekt und Integrationstests für die MVC-Controller.
- [ ] **Themes**: Unterstützung für anpassbare CSS-Themes oder ein dunkles Design.
- [ ] **Berechtigungssystem**: Feinere Rollenverteilung für Wiki-Editoren und Administratoren.

### Meilenstein: Sicherheitspruefung und Debug-Status

Stand: 09.03.2026

- Web-App Build erfolgreich (`dotnet build mvc/mvc.csproj`), dabei nur Warnungen und keine Build-Fehler.
- Paketpruefung ohne bekannte verwundbare NuGet-Pakete (`dotnet list mvc/mvc.csproj package --vulnerable`).
- Gepruefte Seiten: `/`, `/Home/Privacy`, `/testseite`, `/Neuformular/testseite/`, `/Identity/Account/Login`.
- Auffaellige Route: `/Identity/Account/Register` liefert absichtlich `404`, da die Registrierung im Code deaktiviert ist.
- Auffaellige Route: `/Edit/testseite` liefert `404`, solange die Seite noch nicht existiert.
- Kritisches Risiko: gespeicherter Wiki-Inhalt wird als HTML ausgegeben; ohne Sanitizing besteht Stored-XSS-Gefahr.
- Hohes Risiko: Wiki-Seiten koennen aktuell ohne Authentifizierung erstellt und bearbeitet werden.
- Mittleres Risiko: In der Web-App fehlt `UseAuthentication()`, obwohl Identity konfiguriert ist.
- Mittleres Risiko: Der Connection-String mit Passwort steht im Repository in `mvc/appsettings.json`.
- Mittleres Risiko: `UserAdmin` erlaubt extrem schwache Passwoerter fuer neue Benutzer.

## 🏗 Projektstruktur

- `mvc/`: Die ASP.NET Core MVC Webanwendung.
- `Services/`: Gemeinsame Geschäftslogik und Dienste.
- `UserAdmin/`: Konsolenanwendung für administrative Zwecke.
- `Module.Lernplattform/`: Ein (in Entwicklung befindliches) Modul für Lerninhalte.
