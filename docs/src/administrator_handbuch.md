# 📘 Administrator-Handbuch für MeinCMS (Rust Edition)

Willkommen beim offiziellen Administrator-Handbuch für **MeinCMS (wissen-ahrensburg.de)**.  
Dieses Handbuch beschreibt die gesamte Architektur, alle Funktionen, Konfigurationsoptionen, Verwaltungswerkzeuge sowie Best Practices für den sicheren Betrieb im Internet.

---

## 📋 Inhaltsverzeichnis
1. [Systemübersicht & Architektur](#1-systemübersicht--architektur)
2. [Funktionsumfang (Was die Anwendung kann)](#2-funktionsumfang-was-die-anwendung-kann)
3. [Installation, Deployment & Betrieb](#3-installation-deployment--betrieb)
4. [Konfiguration & Umgebungsvariablen](#4-konfiguration--umgebungsvariablen)
5. [Benutzer- & Admin-Verwaltung (meincms_admin)](#5-benutzer--admin-verwaltung-meincms_admin)
6. [Backup, Import & Repair (meincms_backup)](#6-backup-import--repair-meincms_backup)
7. [Sicherheit, Datenschutz & Härtung](#7-sicherheit-datenschutz--härtung)
8. [Fehlerbehebung & Wartung](#8-fehlerbehebung--wartung)

---

## 1. Systemübersicht & Architektur

MeinCMS ist als moderner **Rust Cargo Workspace** aufgebaut. Es besteht aus vier hochperformanten Subsystemen:

```mermaid
flowchart TD
    subgraph Cargo Workspace
        P["meincms_parser\n(Markdown & MediaWiki Compiler Crate)"]
        W["meincms_web\n(Axum 0.7 Async Web Backend)"]
        B["meincms_backup\n(YAML/XML Export, Import & Repair CLI)"]
        A["meincms_admin\n(Argon2 User & Passwords CLI)"]
    end
    W --> P
    B --> P
    A --> P
```

### Die Subsysteme im Detail:
- **`meincms_parser`**: Compiler-Crate für Markdown und MediaWiki-Syntax mit automatischer Kategorie-Extraktion, XSS-Escaping und C-FFI-Schnittstellen.
- **`meincms_web`**: Async Axum Webserver mit Maud-Templating, Hostname-basiertem Multi-Tenancy (`main` vs. `doc`) und rollenbasierter Authentifizierung.
- **`meincms_backup`**: CLI-Werkzeug für die datensparende Sicherung und Wiederherstellung von Wiki-Inhalten (YAML & XML).
- **`meincms_admin`**: CLI-Werkzeug zur Verwaltung von Administrator-Konten und Argon2id-Passwörtern.

---

## 2. Funktionsumfang (Was die Anwendung kann)

### ✍️ Wiki & Editor
- **Duale Syntax-Unterstützung**: Artikel können in **Markdown** oder **MediaWiki (WikiText)** verfasst werden.
- **Dynamische Rhai-Skript-Makros**: Admins und Nutzer können eingebettete Rhai-Skripte in Artikeln ausführen, z. B. `{{#rhai: 5 * 10}}` oder `{{#script: "Hallo " + "Welt"}}`. Die Skripte sind durch ein Sandbox-Limit (max. Operations-Tiefe) gegen Endlosschleifen geschützt.
- **Dynamisches Umschalten**: Im Editor lässt sich die Syntax per Dropdown umschalten (`style.display`).
- **Kategorien-System**: Automatische Erkennung von `[[kategorie:Name]]` im Text oder über Frontmatter-Metadaten.
- **Versionierung & Historie**: Zu jedem Artikel wird bei jeder Änderung eine neue Revisionsversion angelegt. Alte Stände können eingesehen und verglichen werden.

### 🌐 Multi-Tenancy (Mandantenfähigkeit)
- **Automatische Domain-Zuordnung**:
  - `wissen-ahrensburg.de` oder `localhost` ➔ Mandant **`main`** (Haupt-Wiki)
  - `doc.wissen-ahrensburg.de` oder `doc.localhost` ➔ Mandant **`doc`** (Technische Dokumentation)
- **Strikte Daten-Isolation**: Inhalte und Suchen werden in der Datenbank isoliert nach `TenantId` gefiltert.

### 🔒 Rollenbasierter Schreibschutz (`AdminAuth`)
- **Lesen für alle**: Besucher können Artikel, Kategorien und Suchen uneingeschränkt lesen.
- **Schreiben nur für Admins**: Das Erstellen (`/edit/*slug`) und Speichern (`POST /save/*slug`) von Artikeln erfordert zwingend eine aktive Admin-Sitzung. Bei unangemeldetem Zugriff erfolgt eine automatische Umleitung zum Login-Formular (`/login`).

---

## 3. Installation, Deployment & Betrieb

### Voraussetzungen
- **Rust Compiler** (Version 1.80 oder neuer)
- **PostgreSQL** (Version 14 oder neuer)

### 1. Bauen im Release-Modus
```bash
cd wissen-ahrensburg.de
cargo build --release
```
Die fertigen Binärdateien befinden sich anschließend in `./target/release/`.

### 2. Manueller Start
```bash
PORT=5000 DATABASE_URL="postgres://postgres:passwort@localhost:5432/meincms" ./target/release/meincms_web
```

### 3. Einrichtung als Linux Systemd-Dienst (`meincms.service`)
Erstelle die Datei `/etc/systemd/system/meincms.service`:

```ini
[Unit]
Description=MeinCMS Rust Web Backend
After=network.target postgresql.service

[Service]
Type=simple
User=www-data
WorkingDirectory=/var/www/wissen-ahrensburg.de
ExecStart=/var/www/wissen-ahrensburg.de/target/release/meincms_web
Environment="UNIX_SOCKET=/run/meincms/meincms.sock"
Environment="DATABASE_URL=postgres://meincms:SicheresPasswort@/var/run/postgresql/meincms"
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Dienst aktivieren & starten:
```bash
sudo systemctl daemon-reload
sudo systemctl enable meincms
sudo systemctl start meincms
```

### 4. Reverse Proxy Setup (Caddy oder Nginx mit Let's Encrypt HTTPS)

#### Option A: Caddy (Empfohlen - automatische SSL-Zertifikate)
`/etc/caddy/Caddyfile`:
```caddy
wissen-ahrensburg.de {
    reverse_proxy localhost:5000
}

doc.wissen-ahrensburg.de {
    reverse_proxy localhost:5000
}
```

#### Option B: Nginx
`/etc/nginx/sites-available/meincms`:
```nginx
server {
    server_name wissen-ahrensburg.de doc.wissen-ahrensburg.de;

    location / {
        proxy_pass http://127.0.0.1:5000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### 5. Dokumentation bauen & veröffentlichen (mdBook)

Die Handbuch-Dokumentation wird automatisiert mit **mdBook** gebaut und bindet `AGENTS.md` und alle Subagent-Skills aus `.agents/` ein:

- **Dokumentation lokal bauen:**
  ```bash
  npm run build:docs
  ```
  *(Synchronisiert `AGENTS.md` & Skills nach `docs/src` und führt `mdbook build docs` aus)*

- **Dokumentation bauen & automatisch veröffentlichen:**
  ```bash
  npm run ver
  ```
  *(Baut die Dokumentation inkl. `AGENTS.md` & Skills und lädt sie via GitHub Pages auf `handbuch.wissen-ahrensburg.de` hoch)*

---

## 4. Konfiguration & Umgebungsvariablen

Das Verhalten des Webbackends lässt sich über Umgebungsvariablen steuern:

| Variable | Standardwert | Beschreibung |
| :--- | :--- | :--- |
| `PORT` | `5000` | Der HTTP-Port, auf dem der Axum-Webserver lauscht |
| `DATABASE_URL` | *(In-Memory Fallback)* | PostgreSQL-Verbindungszeichenfolge (`postgres://user:pass@host:5432/dbname`) |
| `RUST_LOG` | `meincms_web=info,tower_http=info` | Loglevel für Serverausgaben (`debug`, `info`, `warn`, `error`) |

---

## 5. Benutzer- & Admin-Verwaltung (`meincms_admin`)

Das Werkzeug `meincms_admin` dient der Verwaltung aller Administratoren.

### Interaktiver Modus
```bash
cargo run -p meincms_admin
```

### CLI-Befehle
```bash
# 1. Benutzer auflisten
cargo run -p meincms_admin -- list-users

# 2. Neuen Administrator erstellen
cargo run -p meincms_admin -- create-user --username admin@wissen-ahrensburg.de

# 3. Passwort eines Benutzers ändern
cargo run -p meincms_admin -- reset-password --username admin@wissen-ahrensburg.de
```

> 🔒 **Sicherheitshinweis:** Passwörter werden mit **Argon2id** gehasht und in `config/users.json` bzw. in der Datenbank abgelegt.

---

## 6. Backup, Import & Repair (`meincms_backup`)

Das Werkzeug `meincms_backup` übernimmt den gesicherten Im- und Export sowie die Reparatur aller Inhalte.

### 💾 Backup Exportieren
```bash
# Exportiert den aktuellen Mandanten als YAML-Datei
cargo run -p meincms_backup -- export mein_backup.yaml

# Exportiert als XML-Datei
cargo run -p meincms_backup -- export mein_backup.xml

# Globaler Export aller Mandanten
cargo run -p meincms_backup -- export full_backup.yaml --full
```

### 📥 Backup Importieren
```bash
cargo run -p meincms_backup -- import meine_sicherung.yaml
```
*Hinweis:* Das Import-Tool erkennt automatisch, ob es sich um ein einzelnes Backup-Objekt oder ein Array von Artikeln handelt und unterstützt Abwärtskompatibilität für alte PascalCase-Formate.

### 🔧 Repair (HTML Regenerierung)
Um Speicherplatz zu sparen, speichert MeinCMS **kein generiertes HTML** in Backup-Dateien. Nach einem Import oder nach Änderungen am Parser muss der Repair-Befehl ausgeführt werden:
```bash
cargo run -p meincms_backup -- repair
```

---

## 7. Sicherheit, Datenschutz & Härtung

1. **Dateisystem-Schutz:**
   - Zugriffe auf verdeckte Dateien (`/.gitignore`, `/.env`) oder Konfigurationspfade (`/config/users.json`) werden vom Webserver mit **HTTP 403 Forbidden** blockiert.
2. **Sicherheits-Header:**
   - `Cache-Control: no-store, no-cache, must-revalidate` (Verhindert Zwischenspeicherung sensibler Seiten).
   - `X-Frame-Options: DENY` (Schutz vor Clickjacking).
   - `X-Content-Type-Options: nosniff` (Schutz vor MIME-Sniffing).
3. **Passwort-Sicherheit:**
   - Kein Klartext-Passwort im Quellcode oder in Repositories.
   - Argon2id Hashing mit Salt.

---

## 8. Fehlerbehebung & Wartung

### Problem: Admin-Passwort vergessen
1. Führe den Admin-CLI Befehl aus:
   ```bash
   cargo run -p meincms_admin -- reset-password --username admin@wissen-ahrensburg.de
   ```
2. Falls `config/users.json` beschädigt ist, lösche die Datei: `rm -f config/users.json`. Beim nächsten Aufruf von `meincms_admin` wird ein neuer Admin angelegt.

### Problem: Port bereits belegt (`AddrInUse`)
Wenn der Port (z. B. 5000) belegt ist, wähle einen anderen Port:
```bash
PORT=5005 cargo run -p meincms_web
```

### Problem: HTML-Seiten zeigen leere Abschnitte
Führe den Repair-Befehl aus, um die HTML-Ausgabe aus dem Quelltext neu zu generieren:
```bash
cargo run -p meincms_backup -- repair
```

---
*Stand: Juli 2026 • Lizenz: AGPL-3.0 • MeinCMS Rust Workspace*
