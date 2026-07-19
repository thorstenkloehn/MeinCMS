# wissen-ahrensburg.de - MeinCMS (Rust Edition)

Ein hochperformantes, mandantenfähiges (Multi-Tenancy) Wiki-CMS System, vollständig in **Rust** entwickelt.

---

## ⚡ Features

- **Vollständige Rust Codebase**: Maximale Performance, Memory Safety & minimaler Speicherverbrauch.
- **Mandantenfähigkeit (Multi-Tenancy)**: Automatische Mandantentrennung via Hostname (`main` vs. `doc`).
- **MediaWiki & Markdown Parser**: Eigener Compiler-Parser (`meincms_parser`) mit AST-Builder, HTML Escaping, **eingebetteter Rhai-Skriptsprache (`{{#rhai: ...}}`)** & C-FFI Anbindung.
- **Sicherheit & Admin-Schutz**:
  - Rollenbasierter Admin-Schutz (`AdminAuth`) für Artikel-Erstellung & -Bearbeitung.
  - Argon2id-Passwörthashing für Benutzeraccounts.
  - Globale Security-Header (`No-Cache`, `X-Frame-Options: DENY`, `X-Content-Type-Options: nosniff`).
  - Sperre gegen das Auslesen sensibler Dateien (`/.gitignore`, `/.env`, `/config/users.json` liefern HTTP `403 Forbidden`).
- **Backup & Repair CLI**: YAML/XML Ex- & Import. HTML wird **nicht** gesichert, sondern beim Import/Repair automatisch regeneriert (spart 70 % Speicherplatz).

---

📘 **Ausführliche Dokumentation:** Siehe [Administrator-Handbuch](docs/src/administrator_handbuch.md) für alle Details zu Architektur, Deployment, Sicherheit & Verwaltung.

---

## 📦 Installation & Bauen (Build)

### Voraussetzungen
- **Rust 1.80+** (`cargo` und `rustc`)

### 1. Repository klonen & kompilieren
```bash
# Repository klonen
git clone https://github.com/thorstenkloehn/wissen-ahrensburg.de.git
cd wissen-ahrensburg.de

# Gesamten Workspace im Release-Modus kompilieren
cargo build --release
```

### 2. Anwendung starten
```bash
# Webserver auf Standardport (5000) starten
cargo run -p meincms_web

# Oder direkt das Release-Binary ausführen:
./target/release/meincms_web
```

---

## ⚙️ Einstellungen ändern (Konfiguration)

### 1. Port & Datenbank-Verbindung (Umgebungsvariablen / .env)
Die Konfiguration erfolgt über die `.env`-Datei oder Umgebungsvariablen:

```bash
# TCP-Modus (Entwicklung):
PORT=5000 DATABASE_URL="postgres://postgres:dein_passwort@localhost:5432/meincms" ./target/release/meincms_web

# Unix-Socket-Modus (Produktions-Server):
DATABASE_URL="postgres://postgres:dein_passwort@/var/run/postgresql/meincms" UNIX_SOCKET="/run/meincms/meincms.sock" ./target/release/meincms_web
```

### 2. Administrator-Account verwalten (Passwort & Nutzer ändern)
Nutze das mitgelieferte CLI-Tool `meincms_admin`:

```bash
# Interaktives Admin-Menü starten
cargo run -p meincms_admin

# Oder direkt neuen Admin erstellen:
cargo run -p meincms_admin -- create-user --username admin@wissen-ahrensburg.de
```

---

## 🗑️ Alte / Bestehende Daten löschen (Zurücksetzen)

Falls du alte Testdaten oder Benutzer-Accounts vollständig zurücksetzen möchtest:

### 1. Lokale Benutzer-Accounts löschen
Die Administrator-Accounts und Passwort-Hashes liegen in `config/users.json`:
```bash
# Löscht alle lokal gespeicherten Admin-Accounts
rm -f config/users.json
```
Beim nächsten Start von `meincms_admin` wirst du aufgefordert, einen neuen Notfall-Admin anzulegen.

### 2. Datenbank-Inhalte / Wiki-Seiten zurücksetzen
Falls du die Artikel in PostgreSQL zurücksetzen möchtest:
```sql
-- In deiner PostgreSQL-Datenbank ausführen:
TRUNCATE TABLE wiki_artikel_versions, wiki_artikels RESTART IDENTITY CASCADE;
```

### 3. Neues Backup einspielen
```bash
# Ein sauberes Backup importieren und HTML regenerieren
cargo run -p meincms_backup -- import mein_backup.yaml
```

---

## 🛠️ Workspace-Befehle

| Befehl | Beschreibung |
| :--- | :--- |
| `cargo test --workspace` | Führt alle Unit-Tests im gesamten Rust-Workspace aus |
| `cargo run -p meincms_web` | Startet den Axum-Webserver |
| `cargo run -p meincms_backup -- export backup.yaml` | Exportiert alle Artikel als YAML-Backup |
| `cargo run -p meincms_backup -- repair` | Regeneriert alle HTML-Inhalte aus den Quellen |
| `cargo run -p meincms_admin` | Öffnet das Administrator-Verwaltungs-Menü |
| `npm run build:docs` | Baut die mdBook Dokus inkl. automatischer Einbindung von `AGENTS.md` & Skills |
| `npm run ver` | Baut Dokus inkl. `AGENTS.md` & Skills und veröffentlicht automatisch via gh-pages |

---

## 📜 Lizenz

Dieses Projekt ist unter der **GNU Affero General Public License v3.0 (AGPL-3.0)** lizenziert. Weitere Details finden Sie in der [LICENSE](LICENSE) Datei.
