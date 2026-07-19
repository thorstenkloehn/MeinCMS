# ⚙️ C-API / C-ABI, Conan, NPM & PIP Paketmanager

Dieses Kapitel beschreibt die plattformübergreifenden Schnittstellen und Paketmanager-Integrationen von **MeinCMS (wissen-ahrensburg.de)**. Es erläutert die C-API / C-ABI Exporte des Parser-Crates, die Paketierung für C/C++ über Conan sowie Ubuntu Linux-Alternativen (APT, vcpkg, pkg-config, CMake, Ubuntu Security Portale), die verwendeten NPM-Pakete für Dokumentations-Builds sowie die Anbindung an das Python/PIP-Ökosystem.

---

## 1. C-API & C-ABI Interface (`meincms_parser`)

Das Subsystem `meincms_parser` bietet eine native **C-ABI (Application Binary Interface)**-Schnittstelle, die es erlaubt, den Hochleistungs-Parser in externen C-, C++- oder Fremdsprachen-Projekten (Python, PHP, Go, C#) einzubinden.

### Configuration in `Cargo.toml`
Durch die Angabe von `cdylib` baut Cargo bei `cargo build` eine dynamische C-Bibliothek (`libmeincms_parser.so` unter Linux, `.dylib` unter macOS, `.dll` unter Windows):

```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

### Exportierte C-Funktionen (`src/ffi.rs`)

Alle exportierten Funktionen nutzen `#[no_mangle]` und `unsafe extern "C"` zur Einhaltung der Standard C-Aufrufkonvention.

| C-Funktion | Beschreibung | Rückgabe |
| :--- | :--- | :--- |
| `meincms_markdown_to_html(input: *const c_char)` | Wandelt Markdown in HTML um | `*mut c_char` (String-Pointer) |
| `meincms_markdown_get_categories(input: *const c_char)` | Extrahiert Kategorien aus Markdown als JSON-Array | `*mut c_char` (JSON String-Pointer) |
| `meincms_wikitext_to_html(input: *const c_char)` | Wandelt MediaWiki/WikiText in HTML um | `*mut c_char` (String-Pointer) |
| `meincms_wikitext_get_categories(input: *const c_char)` | Extrahiert Kategorien aus WikiText als JSON-Array | `*mut c_char` (JSON String-Pointer) |
| `meincms_free_string(ptr: *mut c_char)` | Gibt vom Rust-Heap allokierten Speicher frei | `void` |

> ⚠️ **Wichtiger Speicherhinweis:** Rückgabewerte vom Typ `*mut c_char` werden von Rust allokiert und müssen in C/C++ nach Nutzung zwingend mit `meincms_free_string(ptr)` freigegeben werden, um Speicherlecks zu vermeiden!

### Header-Datei (`meincms_parser.h`) für C / C++

```c
#ifndef MEINCMS_PARSER_H
#define MEINCMS_PARSER_H

#ifdef __cplusplus
extern "C" {
#endif

// Wandelt Markdown in HTML um
char* meincms_markdown_to_html(const char* input);

// Extrahiert Kategorien aus Markdown als JSON-String
char* meincms_markdown_get_categories(const char* input);

// Wandelt WikiText in HTML um
char* meincms_wikitext_to_html(const char* input);

// Extrahiert Kategorien aus WikiText als JSON-String
char* meincms_wikitext_get_categories(const char* input);

// Gibt CString Speicher auf dem Rust-Heap frei
void meincms_free_string(char* ptr);

#ifdef __cplusplus
}
#endif

#endif // MEINCMS_PARSER_H
```

### C++ Verwendungsbeispiel

```cpp
#include <iostream>
#include "meincms_parser.h"

int main() {
    const char* markdown = "# Hallo Welt\nDies ist ein **Test** mit [[kategorie:Dokumentation]].";
    
    // HTML konvertieren
    char* html = meincms_markdown_to_html(markdown);
    std::cout << "HTML Output: " << html << std::endl;
    meincms_free_string(html); // Speichergabe

    // Kategorien extrahieren
    char* categories_json = meincms_markdown_get_categories(markdown);
    std::cout << "Kategorien (JSON): " << categories_json << std::endl;
    meincms_free_string(categories_json); // Speichergabe

    return 0;
}
```

---

## 2. Conan Paketmanager & Ubuntu Linux Alternativen

### 2.1 Conan Paketmanager

[Conan](https://conan.io/) ist ein verbreiteter, plattformübergreifender C/C++ Paketmanager. Um `meincms_parser` als wiederverwendbares Paket in C/C++-Projekte (z. B. via CMake) einzubinden, kann ein `conanfile.py` verwendet werden.

#### Beispiel `conanfile.py` für `meincms_parser`

```python
from conan import ConanFile
from conan.tools.files import copy
import os

class MeinCmsParserConan(ConanFile):
    name = "meincms_parser"
    version = "0.1.0"
    license = "AGPL-3.0"
    author = "Thorsten Klöhn"
    url = "https://github.com/thorstenkloehn/wissen-ahrensburg.de"
    description = "Rust-based Markdown & MediaWiki Parser C-FFI Library"
    topics = ("markdown", "wikitext", "parser", "rust", "c-ffi")
    settings = "os", "compiler", "build_type", "arch"

    def build(self):
        # Baut die Rust-Bibliothek via Cargo
        self.run("cargo build --release -p meincms_parser")

    def package(self):
        # Kopiert C-Header und Binärdateien (.so / .dylib / .dll / .a)
        copy(self, "meincms_parser.h", src=self.source_folder, dst=os.path.join(self.package_folder, "include"))
        copy(self, "*.so", src=os.path.join(self.source_folder, "target", "release"), dst=os.path.join(self.package_folder, "lib"))
        copy(self, "*.dylib", src=os.path.join(self.source_folder, "target", "release"), dst=os.path.join(self.package_folder, "lib"))
        copy(self, "*.dll", src=os.path.join(self.source_folder, "target", "release"), dst=os.path.join(self.package_folder, "lib"))

    def package_info(self):
        self.cpp_info.libs = ["meincms_parser"]
```

#### Einbindung in C++ CMake-Projekten
```cmake
cmake_minimum_required(VERSION 3.15)
project(MeinApp CXX)

find_package(meincms_parser REQUIRED)

add_executable(mein_app main.cpp)
target_link_libraries(mein_app PRIVATE meincms_parser::meincms_parser)
```

---

### 2.2 Alternativen zu Conan auf Linux Ubuntu

Falls Conan auf Ubuntu Linux nicht verwendet werden soll, stehen folgende professionelle Alternativen zur Verfügung:

#### 1. Nativer Ubuntu System-Paketmanager (APT / `.deb` Paket via `cargo-deb`)
Unter Ubuntu kann aus dem Rust-Projekt direkt ein nativ installierbares Debian/Ubuntu-Paket (`.deb`) gebaut werden:

```bash
# Cargo DEB Tool installieren
cargo install cargo-deb

# .deb Paket für meincms_parser bauen
cargo deb -p meincms_parser

# Unter Ubuntu installieren
sudo dpkg -i target/debian/meincms_parser_0.1.0_amd64.deb
```
Dabei wird `libmeincms_parser.so` nach `/usr/lib/` und `meincms_parser.h` nach `/usr/include/` installiert. Jeder C/C++ Compiler unter Ubuntu (GCC/Clang) findet die Bibliothek nun systemweit ohne Konfiguration.

#### 2. `pkg-config` Integration (`meincms_parser.pc`)
`pkg-config` ist der Standard auf Ubuntu Linux, um C/C++-Bibliothekspfade automatisch aufzulösen.

Erstelle die Datei `/usr/lib/pkgconfig/meincms_parser.pc`:
```ini
prefix=/usr
exec_prefix=${prefix}
libdir=${exec_prefix}/lib
includedir=${prefix}/include

Name: meincms_parser
Description: High-Performance Rust Markdown & MediaWiki Parser C-FFI
Version: 0.1.0
Libs: -L${libdir} -lmeincms_parser
Cflags: -I${includedir}
```
Kompilieren unter Ubuntu mit GCC / Clang:
```bash
gcc main.c $(pkg-config --cflags --libs meincms_parser) -o mein_programm
```

#### 3. vcpkg (Microsoft C/C++ Package Manager for Linux/Ubuntu)
[vcpkg](https://vcpkg.io/) ist eine weit verbreitete Conan-Alternative für Ubuntu Linux.
Es lässt sich nahtlos in CMake integrieren:
```bash
# vcpkg unter Ubuntu installieren
git clone https://github.com/microsoft/vcpkg.git
./vcpkg/bootstrap-vcpkg.sh

# Einbinden in CMake
cmake -B build -S . -DCMAKE_TOOLCHAIN_FILE=/pfad/zu/vcpkg/scripts/buildsystems/vcpkg.cmake
```

#### 4. CMake `ExternalProject` / `FetchContent` (Ohne externen Paketmanager)
CMake kann unter Ubuntu direkt beim Konfigurieren den Rust Cargo Build anstoßen:

```cmake
include(ExternalProject)
ExternalProject_Add(
    meincms_parser_target
    SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/path/to/meincms_parser
    BUILD_COMMAND cargo build --release
    INSTALL_COMMAND ""
)
```

---

### 2.3 Ubuntu Security Portal & Prüfung einzelner C/C++ Pakete

Für die Verifizierung und Prüfung der Sicherheit einzelner C-Bibliotheken und Pakete unter Ubuntu stehen folgende offizielle Ressourcen und Werkzeuge bereit:

#### 1. Offizielle Ubuntu Security Webseiten & CVE-Datenbanken

- **Ubuntu Security Notices (USN):**  
  🔗 [https://ubuntu.com/security/notices](https://ubuntu.com/security/notices)  
  *Hier veröffentlicht Canonical alle offiziellen Sicherheits-Patches, Updates und Warnungen für einzelne C-Pakete und Systembibliotheken.*

- **Ubuntu CVE Tracker & Paketsuche:**  
  🔗 [https://ubuntu.com/security/cve](https://ubuntu.com/security/cve)  
  *Ermöglicht die gezielte Suche nach Sicherheitslücken (CVEs) für einzelne C-Pakete (z. B. `glibc`, `openssl`, `zlib`, `curl`, `libssl`).*

- **Ubuntu Package Directory:**  
  🔗 [https://packages.ubuntu.com](https://packages.ubuntu.com)  
  *Informationen zu allen Quell- und Binärpaketen und deren Zugehörigkeit zu Repositories (`main`, `universe`, `security`).*

#### 2. Befehle zur Sicherheitsprüfung auf lokalen Ubuntu-Systemen

```bash
# 1. Sicherheitsstatus von Ubuntu Pro & ESM (Security Patches) prüfen
pro status

# 2. Prüfen, aus welchem Security-Repository ein C-Paket bezogen wurde (z.B. noble-security)
apt-cache policy libssl3 libc6

# 3. Bekannte CVEs auf dem eigenen Ubuntu-System auswerten (benötigt debsecan)
sudo apt install debsecan
debsecan
```

#### 3. Compiler-Härtung für C/C++ Pakete unter Ubuntu
Beim Kompilieren von C/C++ Anwendungen, die `libmeincms_parser.so` einbinden, sollten unter Ubuntu stets die offiziellen Canonical Hardening Flags verwendet werden:

```bash
gcc -O2 -fstack-protector-strong -D_FORTIFY_SOURCE=2 -fPIE -pie -Wl,-z,relro,-z,now main.c -lmeincms_parser -o main_app
```

---

## 3. NPM Paketmanager & Scripts

Für die Verwaltung, Generierung und Veröffentlichung der Handbuch-Dokumentation wird Node.js und **npm** eingesetzt.

### `package.json` Übersicht

```json
{
  "name": "wissen-ahrensburg.de",
  "version": "1.0.0",
  "description": "Ein hochperformantes, mandantenfähiges (Multi-Tenancy) Wiki-CMS System, vollständig in Rust entwickelt.",
  "directories": {
    "doc": "docs"
  },
  "scripts": {
    "build:docs": "node scripts/build_docs.js",
    "ver": "npm run build:docs && npx -y gh-pages -d docs/book --nojekyll --cname handbuch.wissen-ahrensburg.de"
  },
  "dependencies": {
    "github-pages": "^0.1.0"
  }
}
```

### Verwendete NPM Pakete

1. **`gh-pages`**: 
   - Ein Utility-Paket zur automatischen Veröffentlichung des vom **mdBook** erzeugten HTML-Ordners (`docs/book`) auf den `gh-pages`-Branch in GitHub.
   - Flag `--nojekyll`: Deaktiviert Jekyll-Processing auf GitHub Pages für korrekte Auslieferung aller Unterordner.
   - Flag `--cname handbuch.wissen-ahrensburg.de`: Setzt automatisch die Custom-Domain der Dokumentation.

2. **`github-pages`**:
   - Unterstützendes Abhängigkeitspaket für GitHub-Pages Hilfsklassen und Konfiguration.

---

## 4. Python & PIP Ökosystem (Python FFI Bindings & PIP Paketierung)

Neben C und C++ kann der `meincms_parser` auch direkt in Python mittels des Standard-Moduls `ctypes` oder über ein mit `pip` installierbares Python-Paket eingebunden werden.

### Einbindung in Python mit `ctypes`

Da `meincms_parser` eine C-ABI dynamische Bibliothek erzeugt (`libmeincms_parser.so`), kann Python sie ohne zusätzliche C-Extensions laden:

```python
import ctypes
import json
import os

# Pfad zur kompilierten Shared Library (.so / .dylib / .dll)
lib_path = os.path.abspath("target/release/libmeincms_parser.so")
lib = ctypes.CDLL(lib_path)

# Signaturen definieren
lib.meincms_markdown_to_html.argtypes = [ctypes.c_char_p]
lib.meincms_markdown_to_html.restype = ctypes.c_char_p

lib.meincms_markdown_get_categories.argtypes = [ctypes.c_char_p]
lib.meincms_markdown_get_categories.restype = ctypes.c_char_p

lib.meincms_free_string.argtypes = [ctypes.c_char_p]
lib.meincms_free_string.restype = None

def render_markdown(text: str) -> str:
    encoded = text.encode("utf-8")
    raw_ptr = lib.meincms_markdown_to_html(encoded)
    result = ctypes.cast(raw_ptr, ctypes.c_char_p).value.decode("utf-8")
    lib.meincms_free_string(raw_ptr) # Speichergabe
    return result

def get_categories(text: str) -> list:
    encoded = text.encode("utf-8")
    raw_ptr = lib.meincms_markdown_get_categories(encoded)
    result_json = ctypes.cast(raw_ptr, ctypes.c_char_p).value.decode("utf-8")
    lib.meincms_free_string(raw_ptr) # Speichergabe
    return json.loads(result_json)

if __name__ == "__main__":
    sample = "# Hallo aus Python\nInhalt mit [[kategorie:PythonTest]]."
    print("HTML:", render_markdown(sample))
    print("Kategorien:", get_categories(sample))
```

### Python-Paketierung (`pyproject.toml` / `pip install`)

Um den Parser als pip-Paket zu verteilen, kann ein `pyproject.toml` in Kombination mit `setuptools` oder `maturin` (für native Rust-Python Bindings) genutzt werden:

```toml
[build-system]
requires = ["setuptools>=61.0", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "meincms_parser"
version = "0.1.0"
description = "Python Bindings for MeinCMS Rust Markdown & MediaWiki Parser"
authors = [{ name = "Thorsten Klöhn" }]
license = { text = "AGPL-3.0" }
dependencies = []
```

### PIP Installation & Verwendung
- Installation im Entwicklungsmodus: `pip install -e .`
- Erstellen von Distribution-Wheels: `pip wheel .`
