# 📚 Externe Bibliotheken & Crates

Diese Dokumentation bietet einen vollständigen und detaillierten Überblick über alle externen Rust-Crates und Bibliotheken, die im Projekt **MeinCMS (wissen-ahrensburg.de)** verwendet werden. 

MeinCMS ist als modularer **Rust Workspace** aufgebaut. Jedes Subsystem (`meincms_parser`, `meincms_web`, `meincms_backup`, `meincms_admin`) nutzt spezialisierte, hochperformante Open-Source-Bibliotheken für seine jeweiligen Aufgaben.

---

## 📋 Schnellübersicht aller Abhängigkeiten

| Crate / Bibliothek | Version | Subsystem(e) | Hauptaufgabe & Kategorie |
| :--- | :--- | :--- | :--- |
| **`axum`** | `0.7` | `meincms_web` | Asynchrones Web-Framework |
| **`tokio`** | `1.x` | `meincms_web`, `meincms_backup` | Asynchrone Runtime & I/O |
| **`sqlx`** | `0.8` | `meincms_web`, `meincms_backup` | Asynchroner PostgreSQL-Treiber & SQL Query Builder |
| **`maud`** | `0.27` | `meincms_web` | Typsicheres Compile-Time HTML-Templating |
| **`rhai`** | `1.19` | `meincms_parser` | Eingebettete Makro-Skripting-Engine |
| **`argon2`** | `0.5` | `meincms_admin` | Kryptografisches Passwort-Hashing (Argon2id) |
| **`html-escape`** | `0.2` | `meincms_parser` | XSS-Schutz & Entschärfen von HTML-Entities |
| **`clap`** | `4.5` | `meincms_backup`, `meincms_admin` | CLI Argument-Parsing mit Subcommands |
| **`serde`** | `1.0` | *Alle Subsysteme* | Framework zur Serialisierung & Deserialisierung |
| **`serde_json`** | `1.0` | `meincms_parser`, `meincms_web`, `meincms_admin` | JSON Parsing & Formatting |
| **`serde_yaml`** | `0.9` | `meincms_backup` | YAML Im- & Export für Backups |
| **`quick-xml`** | `0.41` | `meincms_backup` | Schneller XML-Parser & Serializer |
| **`tower`** | `0.5` | `meincms_web` | Abstraktion für modulare Netzdienste |
| **`tower-http`** | `0.6` | `meincms_web` | HTTP-Middleware (Brotli/Gzip, Static Files, CORS, Tracing) |
| **`hyper`** | `1.x` | `meincms_web` | Low-Level HTTP/1- und HTTP/2-Server-Engine |
| **`hyper-util`** | `0.1` | `meincms_web` | Hilfsfunktionen für Hyper 1.0 (Unix Sockets & Listener) |
| **`tracing`** | `0.1` | `meincms_web` | Strukturiertes Logging & Tracing |
| **`tracing-subscriber`** | `0.3` | `meincms_web` | Log-Formatierung & Log-Level-Filterung (`RUST_LOG`) |
| **`dotenvy`** | `0.15` | `meincms_web` | Auslesen von `.env` Umgebungsvariablen |
| **`chrono`** | `0.4` | `meincms_web`, `meincms_backup` | Datum-, Uhrzeit- & Zeitstempelverwaltung |
| **`regex`** | `1.10` | `meincms_parser`, `meincms_backup` | Reguläre Ausdrücke (MediaWiki Syntax & Reparatur) |
| **`rpassword`** | `7.3` | `meincms_admin` | Verdeckte Passworteingabe auf der Konsole |

---

## 🧩 Details nach Subsystemen

### 1. `meincms_parser` (Markdown & MediaWiki Compiler)

Das Crate `meincms_parser` ist das Herzstück der Inhaltsverarbeitung. Es wandelt Benutzereingaben (Markdown oder MediaWiki/WikiText) sicher in HTML um und führt dynamische Skripte aus.

- **`regex` (v1.10)**: 
  - **Zweck:** Auswertung und Erkennung von Wiki-Syntaxstrukturen wie Wikilinks (`[[Titel]]`), Kategorie-Tags (`[[kategorie:Name]]`), Frontmatter und Makro-Blöcken (`{{#rhai: ...}}`).
  - **Warum gewählt:** Standard-Regex-Engine für Rust, hochoptimiert und gegen ReDoS-Angriffe (Regular Expression Denial of Service) geschützt.
- **`html-escape` (v0.2)**: 
  - **Zweck:** Entschärfen potenziell gefährlicher Zeichen (`<`, `>`, `&`, `"`, `'`) vor der HTML-Generierung.
  - **Warum gewählt:** Gewährleistet strikten Schutz gegen Cross-Site-Scripting (XSS), da unsicherer Benutzercode niemals unmaskiert gerendert wird.
- **`rhai` (v1.19)**: 
  - **Zweck:** Eingebettete Skriptsprache zur Ausführung dynamischer Makros in Wiki-Artikeln (z. B. mathematische Berechnungen, Zeichenkettenoperationen).
  - **Warum gewählt:** Rhai wurde speziell für Rust entwickelt, kompiliert ohne externe C-Abhängigkeiten und bietet ein konfigurierbares Ausführungslimit (Sandbox), das Endlosschleifen verhindert.
- **`serde` (v1.0)** & **`serde_json` (v1.0)**: 
  - **Zweck:** Umwandlung interner AST-Metadaten (Abstract Syntax Tree) und C-FFI Export-Datenstrukturen in/aus JSON.

---

### 2. `meincms_web` (Axum Async Web-Backend)

Das Subsystem `meincms_web` stellt das eigentliche Webseiten-Backend für Besucher und Administratoren bereit.

- **`axum` (v0.7)**: 
  - **Zweck:** Asynchrones Web-Framework für Routing, Request-Handler, Extractor-Mechanismen und Server-Antworten.
  - **Warum gewählt:** Entwickelt vom Tokio-Team, bietet maximale Ergonomie, Typen-Sicherheit und exzellente Performance ohne Laufzeit-Overhead.
- **`tokio` (v1.x, Feature `full`)**: 
  - **Zweck:** Event-gesteuerte asynchrone Runtime für Multithreading, I/O-Operationen und Server-Sockets.
  - **Warum gewählt:** Der De-facto-Standard in der Rust-Ökosystem für performante Netzwerkanwendungen.
- **`sqlx` (v0.8, Features: `postgres`, `runtime-tokio-rustls`, `chrono`, `json`)**: 
  - **Zweck:** Asynchroner PostgreSQL-Datenbanktreiber und Query-Builder.
  - **Warum gewählt:** Bietet Typsicherheit beim Ausführen von Datenbankabfragen und unterstützt native Unix Domain Sockets (`/var/run/postgresql/meincms`) sowie JSON-Spalten.
- **`maud` (v0.27)**: 
  - **Zweck:** Schreiben von HTML-Templates direkt in Rust-Code via `html! { ... }` Makro.
  - **Warum gewählt:** Maud-Templates werden zur Compile-Zeit in hochoptimierten Rust-Code umgewandelt. Das vermeidet Parsing-Kosten zur Laufzeit und macht XSS-Escaping typsicher.
- **`tower-http` (v0.6)** & **`tower` (v0.5)**: 
  - **Zweck:** Middleware-Komponenten für Komprimierung (Brotli & Gzip), Ausliefern statischer CSS/JS-Dateien, CORS und Request-Logging.
- **`hyper` (v1.x)** & **`hyper-util` (v0.1)**: 
  - **Zweck:** Unterbau für den HTTP-Server. Ermöglicht das Binden des Webservers an TCP-Ports oder Unix-Domain-Sockets (`UNIX_SOCKET`).
- **`tracing` (v0.1)** & **`tracing-subscriber` (v0.3)**: 
  - **Zweck:** Strukturierte Diagnose- und Log-Ausgaben im Serverbetrieb. Steuerung der Detailtiefe über die Umgebungsvariable `RUST_LOG`.
- **`dotenvy` (v0.15)**: 
  - **Zweck:** Automatisches Laden von Konfigurationswerten aus einer `.env`-Datei beim Anwendungsstart.
- **`chrono` (v0.4)**: 
  - **Zweck:** Datums- und Zeitberechnungen für Artikel-Revisionen und HTTP-Header.

---

### 3. `meincms_backup` (YAML/XML Export, Import & Repair CLI)

Das CLI-Werkzeug `meincms_backup` dient der Datensicherung, Datenmigration und der Regenerierung beschädigter HTML-Inhalte.

- **`clap` (v4.5, Feature `derive`)**: 
  - **Zweck:** Verarbeiten von Befehlszeilen-Argumenten und Optionen (`export`, `import`, `repair`, `--full`).
  - **Warum gewählt:** Deklarative Definition von CLI-Interfaces mit automatischer Generierung von Hilfetexten (`--help`).
- **`serde_yaml` (v0.9)**: 
  - **Zweck:** Serialisierung und Deserialisierung von Wiki-Artikeln in gut lesbare YAML-Sicherungsdateien.
- **`quick-xml` (v0.41)**: 
  - **Zweck:** Extrem schnelles Lesen und Schreiben von XML-Sicherungen.
  - **Warum gewählt:** Verarbeitet selbst sehr große XML-Backups mit minimalem Speicherbedarf. Geschützt gegen O(N²) Duplicate Attribute Scans und Unbounded Namespace Allocation (RUSTSEC-2026-0194 / RUSTSEC-2026-0195).
- **`sqlx` & `tokio`**: 
  - **Zweck:** Direktes Einlesen und Zurückschreiben von Datensätzen in die PostgreSQL-Datenbank im CLI-Kontext.

---

### 4. `meincms_admin` (User- & Admin-Management CLI)

Das Werkzeug `meincms_admin` dient der sicheren Verwaltung von Administrator-Konten und Zugangsdaten.

- **`argon2` (v0.5)**: 
  - **Zweck:** Hashen und Überprüfen von Administrator-Passwörtern nach dem Argon2id-Standard.
  - **Warum gewählt:** Sieger der Password Hashing Competition (PHC). Bietet höchstmöglichen Schutz gegen GPU- und ASIC-basierte Brute-Force-Angriffe durch konfigurierbare Speicher- und Zeitkomplexität.
- **`rpassword` (v7.3)**: 
  - **Zweck:** Sicheres Einlesen von Passwörtern im Terminal ohne Konsolenausgabe (Kein Echo des Passworts auf dem Bildschirm).
- **`clap` (v4.5)** & **`serde_json` (v1.0)**: 
  - **Zweck:** CLI-Steuerung und Speichern der Admin-Konfiguration (`config/users.json`).

---

## 🔒 Sicherheits- & Wartungshinweise

1. **Abhängigkeits-Analyse mit `deps` & `cargo tree`:**
   Ein Tool zur Analyse der Abhängigkeiten eines Projekts. Es hilft dabei, transitive Abhängigkeiten zu verstehen und doppelte Crates aufzuspüren:
   ```bash
   cargo tree
   ```
   Ebenfalls steht über [https://deps.dev](https://deps.dev) (Google Open Source Insights) eine tiefgehende Analyse der Abhängigkeiten bereit.

2. **Sicherheits-Audits mit `osv` (Open Source Vulnerabilities):**
   `osv` ist eine verteilte Schwachstellendatenbank für Open Source (`osv.dev`). Das CLI-Tool `osv-scanner` ermöglicht sprachübergreifende Audits:
   ```bash
   go install github.com/google/osv-scanner/v2/cmd/osv-scanner@v2
   osv-scanner -r path/to/your/project
   ```

3. **Alternative Scanner zu OSV-Scanner (Trivy, Grype, Semgrep):**
   - **Trivy (Aqua Security):** Open-Source Scanner für Dateisysteme & Abhängigkeiten (`trivy fs /pfad/zum/projekt`).
   - **Grype (Anchore):** Schneller Schwachstellenscanner für Verzeichnisse & SBOMs (`grype dir:/pfad/zum/projekt`).
   - **Semgrep:** Regelbasierte statische Code-Analyse / SAST (`semgrep scan`).

4. **Statische JavaScript Code-Analyse mit ESLint & `eslint-plugin-security`:**
   ESLint prüft Frontend-Skripte und Node.js-Build-Skripte auf Sicherheitsrisiken und Code-Qualität:
   ```bash
   npm install --save-dev eslint eslint-plugin-security
   npx eslint .
   ```

5. **Statische Code-Analyse & Vulnerability Scanning mit Snyk:**
   Snyk ermöglicht statische Quellcode-Analysen (SAST) sowie Scans von Projektschnittstellen:
   ```bash
   npm install -g snyk
   snyk auth
   snyk code test
   ```

5. **RustSec Security & Safety Audits (`cargo audit`, `cargo-deny` & `cargo-geiger`):**
   Um Sicherheitslücken, Lizenzen, verbotene Crates sowie die Verwendung von `unsafe`-Codeblöcken frühzeitig zu prüfen:
   ```bash
   cargo install cargo-audit cargo-deny cargo-geiger
   cargo audit
   cargo-deny check
   cargo geiger --manifest-path meincms_parser/Cargo.toml
   ```
   Projektbezogene Ausnahmen sind in `.cargo/audit.toml` und `deny.toml` konfiguriert.

6. **Aktualisierung der Crates:**
   Crates innerhalb der angegebenen Minor-Versionen können mit folgendem Befehl auf den neuesten Stand gebracht werden:
   ```bash
   cargo update
   ```

7. **Lizenzkonformität:**
   Alle verwendeten Bibliotheken nutzen verträgliche Open-Source-Lizenzen (wie MIT, Apache-2.0 oder BSD), die vollständig mit der **AGPL-3.0-Lizenz** von MeinCMS kompatibel sind.



