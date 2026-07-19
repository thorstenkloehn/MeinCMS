# 🛡️ Sicherheits-Handbuch: Ubuntu, Rust & NPM Ökosystem

Dieses Dokument bietet eine vollständige Übersicht aller offiziellen Sicherheitsportale, CVE-Datenbanken und CLI-Werkzeuge zur Überprüfung von Sicherheitslücken in **C/Ubuntu-Paketen**, **Rust Crates** sowie **NPM Node.js Paketen**.

---

## 1. Ubuntu Linux & C/C++ Sicherheitsportale

Canonical betreibt dedizierte Sicherheitsservices für alle C-Pakete und Systembibliotheken (`glibc`, `openssl`, `zlib`, `curl`):

### Offizielle Ubuntu Webseiten
* **Ubuntu Security Notices (USN):**  
  🔗 [https://ubuntu.com/security/notices](https://ubuntu.com/security/notices)  
  *Offizielle Ankündigungen aller Sicherheitsupdates und Patches für Ubuntu-Pakete.*

* **Ubuntu CVE Tracker:**  
  🔗 [https://ubuntu.com/security/cve](https://ubuntu.com/security/cve)  
  *Gezielte Suche nach spezifischen Sicherheitslücken (CVEs) in einzelnen Paketen.*

* **Ubuntu Package Directory:**  
  🔗 [https://packages.ubuntu.com](https://packages.ubuntu.com)  
  *Übersicht aller Quell- und Binärpakete (`main`, `universe`, `security`).*

### Ubuntu Terminal-Befehle
* **`pro status`**: Prüft den allgemeinen Sicherheits- und ESM-Status (Expanded Security Maintenance).
* **`apt-cache policy <paketname>`**: Zeigt Paketversion und Security-Repository-Herkunft (z.B. `noble-security`).
* **`debsecan`**: Analysiert installierte Pakete auf bekannte CVEs (`sudo apt install debsecan && debsecan`).

### C / C++ Statische & Dynamische Sicherheits-Scanner (SAST & Memory Safety)

Für C-Pakete und C/C++-Schnittstellen (wie den C-ABI Export `meincms_parser.h`) kommen spezialisierte SAST- und Laufzeit-Scanner zum Einsatz:

* **Flawfinder (SAST für C/C++):**  
  Scannt C/C++-Quellcode auf bekannte Sicherheitsrisiken und unsichere API-Funktionen (z.B. `strcpy`, `sprintf`):
  ```bash
  sudo apt install flawfinder
  flawfinder .
  ```

* **Cppcheck (Statische C/C++ Analyse):**  
  Spürt Buffer Overflows, Speicherlecks, Nullpointer-Dereferenzierungen und uninitialisierte Variablen auf:
  ```bash
  sudo apt install cppcheck
  cppcheck --enable=all --suppress=missingIncludeSystem .
  ```

* **Clang-Tidy & Clang Static Analyzer:**  
  LLVM-basierte statische Code-Analyse zur Überprüfung von C/C++ Sicherheitsregeln (CERT / MISRA):
  ```bash
  clang-tidy --checks='clang-analyzer-*,cert-*' main.c
  ```

* **AddressSanitizer (ASan) & Valgrind (Dynamische Speicherprüfung):**  
  Erkennt Speicherzugriffsfehler (Use-After-Free, Out-of-Bounds, Memory Leaks) zur Laufzeit:
  ```bash
  # Valgrind Laufzeitanalyse
  sudo apt install valgrind
  valgrind --leak-check=full ./mein_c_programm

  # AddressSanitizer beim GCC/Clang Kompilieren aktivieren
  gcc -fsanitize=address -g main.c -lmeincms_parser -o main_app
  ```

---

## 2. Rust Ökosystem Sicherheitsportale & Audit Tools

Das Rust-Ökosystem verfügt über eine hochmoderne, automatisierte Sicherheitsinfrastruktur zur Überprüfung aller Cargo-Abhängigkeiten (`crates.io`).

### Offizielle Rust Security Webseiten

* **RustSec Security Advisory Database:**  
  🔗 [https://advisories.rust-lang.org](https://advisories.rust-lang.org)  
  *Die offizielle Datenbank aller gemeldeten Sicherheitslücken (Advisories) und zerschlagenen („yanked“) Crates auf crates.io.*

* **GitHub RustSec Advisory Repository:**  
  🔗 [https://github.com/rustsec/advisories](https://github.com/rustsec/advisories)  
  *Das Quellcode-Repository der Rust-Sicherheitsdatenbank (Community-gepflegt).*

* **Rust Language Security Notices:**  
  🔗 [https://blog.rust-lang.org/category/security.html](https://blog.rust-lang.org/category/security.html)  
  *Offizieller Rust-Blog für kritische Sicherheitsmeldungen zu Compiler, Standardbibliothek (`std`) und Cargo.*

### Rust Terminal-Befehle & CLI Tools

* **`cargo audit`** *(Empfohlen)*:  
  Scannt die Datei `Cargo.lock` des Projekts automatisch gegen die RustSec-Datenbank und meldet bekannte Sicherheitslücken:
  ```bash
  # Installation (einmalig)
  cargo install cargo-audit

  # Sicherheitsprüfung im Workspace ausführen
  cargo audit
  ```

* **`cargo deny`**:  
  Prüft den Workspace nicht nur auf Sicherheitslücken, sondern auch auf unerwünschte Lizenzen und doppelte Abhängigkeiten:
  ```bash
  cargo install cargo-deny
  cargo deny check
  ```

* **`cargo-geiger` (Rust Speicher-Sicherheitswert & `unsafe` Rating):**  
  Analysiert das Projekt und alle Abhängigkeiten auf die Verwendung von `unsafe` Rust-Code und berechnet den relativen Sicherheitswert:
  ```bash
  cargo install cargo-geiger
  cargo geiger
  ```

* **Miri (Undefined Behavior Detection in Rust):**  
  Offizieller Rust Mid-level IR (MIR) Interpreter zur Feststellung von undefiniertem Verhalten (Undefined Behavior, Memory Leaks, Data Races) in `unsafe`-Rust-Code:
  ```bash
  rustup component add miri
  cargo miri test
  ```


* **`deps` (`cargo-deps`, `cargo tree` & `deps.dev`)**:  
  Ein Tool zur Analyse der Abhängigkeiten eines Projekts. Es verarbeitet Abhängigkeitsbäume, identifiziert veraltete oder doppelte Crates und visualisiert Abhängigkeitsgraphen:
  ```bash
  # Visualisierung des Abhängigkeitsbaums im Terminal
  cargo tree

  # Erstellung eines visuellen Abhängigkeitsgraphen via cargo-deps
  cargo install cargo-deps
  cargo deps | dot -Tpng > deps_graph.png

  # Online-Analyse über Google Open Source Insights: https://deps.dev
  ```

---

## 3. NPM & Node.js Ökosystem Sicherheitsportale & Audit Tools

Auch für NPM-Pakete existieren globale Datenbanken und integrierte Auditing-Werkzeuge.

### Offizielle NPM Security Webseiten

* **GitHub Advisory Database (NPM Security):**  
  🔗 [https://github.com/advisories?query=type%3Anpm](https://github.com/advisories?query=type%3Anpm)  
  *Die zentrale GitHub/NPM Datenbank zur Abfrage aller CVEs und advisories für Node.js/NPM Pakete.*

* **NPM Security Documentation & Best Practices:**  
  🔗 [https://docs.npmjs.com/auditing-package-dependencies-for-security-vulnerabilities](https://docs.npmjs.com/auditing-package-dependencies-for-security-vulnerabilities)  
  *Dokumentation zum automatischen Sicherheits-Audit von Paketabhängigkeiten.*

### NPM Terminal-Befehle & CLI Tools

* **`npm audit`**:  
  Vergleicht `package.json` und `package-lock.json` automatisch mit der GitHub Advisory Database:
  ```bash
  npm audit
  ```

* **`npm audit fix`**:  
  Aktualisiert verwundbare Pakete automatisch auf die nächstsichere Version (ohne Breaking Changes).

* **ESLint Security Plugin (`eslint-plugin-security` - SAST für JS/TS):**  
  Statische Code-Analyse speziell für JavaScript & Node.js zur Erkennung unsicherer Befehle (`eval()`, ReDoS-Regex, Unsanitized Inputs, Child Processes):
  ```bash
  # Installation des Security Plugins
  npm install --save-dev eslint eslint-plugin-security

  # Ausführung des Security-Linter-Scans
  npx eslint --plugin security --rule 'security/detect-object-injection: error' .
  ```

* **Retire.js (Vulnerability Scanner für JS-Bibliotheken):**  
  Spezialisierter Scanner zur Erkennung bekannter Sicherheitslücken in JavaScript-Dateien und NPM-Paketen:
  ```bash
  # Globale Installation
  npm install -g retire

  # Scan des Projekt-Ordners
  retire --path .
  ```

* **`npm ls` & `npm outdated` (Abhängigkeits-Analyse):**  
  Analysiert die Baumstruktur aller NPM-Pakete und hebt veraltete Bibliotheken hervor:
  ```bash
  # Abhängigkeitsbaum anzeigen
  npm ls --all

  # Veraltete Pakete prüfen
  npm outdated
  ```

### NPM Security & Package Rating (Paket-Sicherheitsbewertung)

* **Socket.dev (Supply-Chain-Security Rating):**  
  🔗 [https://socket.dev](https://socket.dev)  
  *Analysiert NPM-Pakete auf Verhaltensrisiken wie versteckte Telemetrie, Install-Skripte, Typosquatting und Berechtigungen (Netzwerk, Dateisystem).*

* **npms.io (NPM Package Quality & Health Score):**  
  🔗 [https://npms.io](https://npms.io)  
  *Berechnet einen kombinierten Score (Quality, Maintenance, Popularity) für jedes NPM-Paket.*


---

## 4. Snyk Security Scanner (SAST & Open Source Security)

**Snyk** ist eine führende Sicherheitsplattform zur statischen Code-Analyse (SAST - Static Application Security Testing) und zur Überprüfung von Abhängigkeiten über mehrere Programmiersprachen hinweg.

### Snyk CLI Installation & Authentifizierung
```bash
# Globale Installation des Snyk CLI via NPM
npm install -g snyk

# Authentifizierung des CLI-Tools mit dem Snyk-Account
snyk auth
```

### Quellcode- & Abhängigkeitsscans mit Snyk
```bash
# Quellcode-Analyse (SAST) im aktuellen Projektverzeichnis ausführen
snyk code test

# Quellcode-Analyse für einen spezifischen Pfad ausführen
snyk code test /pfad/zum/code

# Prüfung aller Projekt-Abhängigkeiten (Open Source Vulnerabilities)
snyk test
```

---

## 5. Plattformübergreifende Vulnerability Datenbanken & Scanner (OSV)

* **Google OSV (Open Source Vulnerabilities):**  
  🔗 [https://osv.dev](https://osv.dev)  
  *Eine verteilte Schwachstellendatenbank für Open Source, die Sicherheitslücken über Sprachgrenzen hinweg (Rust/Cargo, NPM, PyPI, Linux/Ubuntu, Go, C/C++) aggregiert.*

* **`osv-scanner` CLI Tool:**  
  Ein von Google bereitgestellter universeller Vulnerability Scanner, der Projekt-Abhängigkeiten direkt gegen die verteilte OSV-Datenbank prüft:
  ```bash
  # Installation des OSV-Scanners (v2)
  go install github.com/google/osv-scanner/v2/cmd/osv-scanner@v2

  # Rekursiver Scan aller Projekt-Abhängigkeiten im aktuellen Verzeichnis
  osv-scanner -r .

  # Oder rekursiver Scan für einen spezifischen Projektpfad
  osv-scanner -r path/to/your/project
  ```

### Weitere alternative Security Scanner zu OSV-Scanner

Neben OSV-Scanner und Snyk stehen weitere professionelle, plattformübergreifende Open-Source-Scanner bereit:

* **Trivy (Aqua Security):**  
  Ein umfassender Open-Source Scanner für Dateisysteme, Code-Abhängigkeiten (Cargo, NPM, PIP) und Container:
  ```bash
  # Installation unter Ubuntu
  sudo apt install trivy

  # Scan des Projektverzeichnisses auf Schwachstellen
  trivy fs /pfad/zum/projekt
  ```

* **Grype (Anchore):**  
  Ein extrem schneller Vulnerability-Scanner für Quellcode-Verzeichnisse und Software-Stücklisten (SBOMs):
  ```bash
  # Installation via Install-Skript
  curl -sSfL https://raw.githubusercontent.com/anchore/grype/main/install.sh | sh -s -- -b /usr/local/bin

  # Scan eines Projektverzeichnisses
  grype dir:/pfad/zum/projekt
  ```

* **Semgrep (SAST Code Scanner):**  
  Ein leichter, regelbasierter SAST-Scanner für statische Code-Analysen auf Sicherheitslücken:
  ```bash
  # Installation via PIP
  pip install semgrep

  # Statischen Code-Scan im Projekt ausführen
  semgrep scan
  ```

### Sicherheitswert-Bewertungsstandards (Severity Rating) vs. OSV

> 💡 **Wichtige Unterscheidung:**  
> **Google OSV** ist eine *Schwachstellendatenbank* und ein *Vergleichs-Scanner*, jedoch **kein eigenständiges Sicherheitswert-Bewertungssystem**. OSV aggregiert Daten aus Quellen wie NVD (National Vulnerability Database), RustSec und GitHub Security Advisories.
>
> Der eigentliche **Sicherheitswert (Severity Score)** einer Schwachstelle in OSV, Rust, C oder C++ wird nach internationalen Standards berechnet:
>
> 1. **CVSS (Common Vulnerability Scoring System, v3.1 / v4.0):**  
>    Quantitativer Sicherheitswert von **0.0 bis 10.0**:
>    - `0.0` - `3.9`: **Low** (Geringes Risiko)
>    - `4.0` - `6.9`: **Medium** (Mittleres Risiko)
>    - `7.0` - `8.9`: **High** (Hohes Risiko)
>    - `9.0` - `10.0`: **Critical** (Kritisches Risiko)
>
> 2. **CWE (Common Weakness Enumeration):**  
>    Standardisierte Kategorisierung von Sicherheitsfehler-Klassen (z. B. `CWE-119` für Buffer Overflow, `CWE-79` für XSS, `CWE-416` für Use-After-Free).
>
> 3. **EPSS (Exploit Prediction Scoring System):**  
>    Prozentuale Wahrscheinlichkeit (0% bis 100%), dass eine Schwachstelle in den nächsten 30 Tagen tatsächlich in freier Wildbahn ausgenutzt wird.


---

## 📑 Zusammenfassende Befehlsübersicht für dieses Projekt

| Ökosystem | Befehl | Zweck |
| :--- | :--- | :--- |
| **Ubuntu C/System** | `apt-cache policy libc6` | Prüft Security-Patch-Stand von C-Bibliotheken |
| **Ubuntu C/System** | `debsecan` | Analysiert lokale Ubuntu-Pakete auf CVEs |
| **C / C++ SAST** | `flawfinder .` | Scannt C/C++ Quellcode auf unsichere API-Funktionen |
| **C / C++ SAST** | `cppcheck --enable=all .` | Analysiert C/C++ Quellcode auf Speicherlecks & Nullpointer |
| **C / C++ Runtime** | `valgrind --leak-check=full ./app` | Laufzeit-Speicherfehler-Analyse (Memory Leaks, Buffer Overflows) |
| **Projekt / Cargo** | `deps` (`cargo tree` / `cargo-deps`) | Ein Tool zur Analyse der Abhängigkeiten eines Projekts |
| **Rust** | `cargo audit` | Scannt alle Rust-Crates (`Cargo.lock`) gegen RustSec DB |
| **Rust** | `cargo deny check` | Prüft Sicherheitslücken & Lizenzkonformität |
| **Rust Safety** | `cargo geiger` | Misst den Speicher-Sicherheitswert & `unsafe`-Rust-Anteil |
| **Rust UB Check** | `cargo miri test` | Erkennt undefiniertes Verhalten (Undefined Behavior) in `unsafe` Rust |
| **NPM** | `npm audit` | Scannt alle NPM-Pakete (`package-lock.json`) gegen GitHub Advisory DB |
| **NPM SAST** | `npx eslint --plugin security .` | Statische Analyse von JS/TS-Code auf Sicherheitsfehler |
| **NPM Security** | `retire --path .` | Scannt JS-Bibliotheken & Pakete auf bekannte Sicherheitslücken |
| **NPM Rating** | Socket.dev / npms.io | Bewertet Paket-Sicherheit, Supply-Chain-Risiko & Wartungs-Score |

| **Multi-Language** | `snyk code test [pfad]` | Statische Quellcode-Analyse (SAST) auf Sicherheitslücken |
| **Multi-Language** | `snyk test` | Scannt Projekt-Abhängigkeiten via Snyk Platform |
| **Multi-Language** | `osv-scanner -r .` | Scannt Projekt-Abhängigkeiten gegen die verteilte OSV-Schwachstellendatenbank |
| **Multi-Language** | `trivy fs /pfad/zum/projekt` | Alternative: Open-Source Scanner für Dateisysteme & Abhängigkeiten |
| **Multi-Language** | `grype dir:/pfad/zum/projekt` | Alternative: Schneller Vulnerability Scanner von Anchore |
| **Multi-Language** | `semgrep scan` | Alternative: Regelbasierte statische Code-Analyse (SAST) |




