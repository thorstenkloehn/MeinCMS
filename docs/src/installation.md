# 1. Installation & Setup

Willkommen bei **MeinCMS** (wissen-ahrensburg.de). Dieses Kapitel führt dich Schritt für Schritt durch die Vorbereitung der Systemumgebung, das Klonen, das Kompilieren sowie die initiale Einrichtung der Datenbank und des Administrator-Accounts.

---

## 📋 Systemvoraussetzungen

Stelle sicher, dass folgende Softwarekomponenten auf deinem Linux-System installiert sind:

* **Rust & Cargo:** Version `1.80` oder neuer ([rustup.rs](https://rustup.rs/))
* **PostgreSQL:** Version `14` oder neuer
* **Git:** Zum Klonen des Quellcodes
* *(Optional)* **Nginx** oder **Caddy**: Als Reverse Proxy für TLS/HTTPS im Produktionsbetrieb.

---

## 1. Repository klonen

Klone das Repository auf deinen Zielserver:

```bash
git clone https://github.com/thorstenkloehn/wissen-ahrensburg.de.git
cd wissen-ahrensburg.de
```

---

## 2. PostgreSQL Datenbank einrichten

MeinCMS benötigt eine PostgreSQL-Datenbank. Erstelle eine neue Datenbank und einen Datenbank-Benutzer:

```sql
-- In psql (als postgres-User):
CREATE USER meincms WITH PASSWORD 'dein_sicheres_passwort';
CREATE DATABASE meincms OWNER meincms;
GRANT ALL PRIVILEGES ON DATABASE meincms TO meincms;
```

Setze anschließend die Umgebungsvariable `DATABASE_URL`:

```bash
# Option A: TCP Connection (Entwicklung)
export DATABASE_URL="postgres://meincms:dein_sicheres_passwort@localhost:5432/meincms"

# Option B: Unix Domain Socket (Produktions-Server)
export DATABASE_URL="postgres://meincms:dein_sicheres_passwort@/var/run/postgresql/meincms"
```

---

## 3. Projekt kompilieren (Build)

Der Rust Workspace besteht aus den vier Crates:
1. `meincms_parser`: Markdown & MediaWiki Compiler
2. `meincms_web`: Axum Webbackend
3. `meincms_backup`: Backup & Repair CLI
4. `meincms_admin`: Admin & Benutzer-Verwaltung

### Entwicklungs-Build (Debugging)
```bash
cargo build
```

### Produktions-Build (Optimiert)
Für maximale Performance kompiliere die Anwendung im Release-Modus:
```bash
cargo build --release
```
Die fertigen Binaries befinden sich anschließend unter `target/release/`.

---

## 4. Admin-Benutzer erstellen

Vor dem ersten Start solltest du deinen Administrator-Account anlegen. Nutze dafür das CLI-Tool `meincms_admin`:

```bash
cargo run -p meincms_admin -- create-user --username admin@wissen-ahrensburg.de
```
Das CLI-Tool fordert dich auf, ein sicheres Passwort einzugeben. Der Hash wird mittels **Argon2id** geschützt und in `config/users.json` gespeichert.

---

## 5. Webserver starten

Starte den Axum-Server mit:

```bash
# Entwicklungsmodus (TCP Port):
PORT=5000 cargo run -p meincms_web

# Produktionsmodus (Unix Domain Socket):
UNIX_SOCKET="/run/meincms/meincms.sock" ./target/release/meincms_web
```

Nach dem Start im TCP-Modus ist die Anwendung unter `http://localhost:5000` erreichbar.

---

## 6. Nginx Reverse Proxy (Produktion)

Um die Anwendung im Produktivbetrieb mit HTTPS / Let's Encrypt über den Unix-Socket abzusichern:

```nginx
server {
    listen 80;
    server_name wissen-ahrensburg.de;

    location / {
        proxy_pass http://unix:/run/meincms/meincms.sock;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

---

## 7. Dokumentation bauen & veröffentlichen (mdBook)

Die Handbuch-Dokumentation wird mit **mdBook** generiert und bindet automatisch `AGENTS.md` sowie alle Subagent-Skills ein.

* **Dokumentation lokal bauen:**
  ```bash
  npm run build:docs
  ```
  *Synchronisiert `.agents/AGENTS.md` und `.agents/skills/` nach `docs/src/` und führt `mdbook build docs` aus.*

* **Dokumentation bauen & automatisch veröffentlichen:**
  ```bash
  npm run ver
  ```
  *Baut die Dokumentation neu und lädt sie via GitHub Pages auf `handbuch.wissen-ahrensburg.de` hoch.*

---

## 🧪 Installation testen

Führe alle Unittests des Workspaces aus, um die korrekte Installation zu bestätigen:

```bash
cargo test --workspace
```
