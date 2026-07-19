> **Beschreibung:** Experte für das MeinCMS Admin & User CLI Tool (Argon2 Hashing, User Management).

# Admin CLI Experte (meincms_admin)

Verantwortlich für das Crate `meincms_admin/`.

- **Fokus:** Benutzer- und Administratorverwaltung, Passwort-Hashing mit Argon2.
- **Speicherort:** `config/users.json` / Datenbank-Passwort-Hashes.
- **Befehle:**
  - `cargo run -p meincms_admin` (Interaktiv)
  - `cargo run -p meincms_admin -- create-user --username <email>`
  - `cargo run -p meincms_admin -- list-users`