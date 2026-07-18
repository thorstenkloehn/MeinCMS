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
export DATABASE_URL="postgres://meincms:dein_sicheres_passwort@localhost:5432/meincms"
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
# Entwicklungsmodus:
PORT=5000 cargo run -p meincms_web

# Produktionsmodus (aus Release-Binary):
PORT=5000 ./target/release/meincms_web
```

Nach dem Start ist die Anwendung unter `http://localhost:5000` erreichbar.

---

## 6. Nginx Reverse Proxy (Produktion)

Um die Anwendung im Produktivbetrieb mit HTTPS / Let's Encrypt abzusichern, erstelle eine Nginx-Konfiguration:

```nginx
server {
    listen 80;
    server_name wissen-ahrensburg.de;

    location / {
        proxy_pass http://127.0.0.1:5000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

---

## 🧪 Installation testen

Führe alle Unittests des Workspaces aus, um die korrekte Installation zu bestätigen:

```bash
cargo test --workspace
```
