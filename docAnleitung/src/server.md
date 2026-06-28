# Produktions-Server Installation

Diese Anleitung beschreibt das Vorgehen zur Erstinstallation von MeinCMS auf einem produktiven Linux-Server (z. B. Ubuntu oder Debian).

## Voraussetzungen auf dem Server

Stellen Sie sicher, dass auf dem Zielserver folgende Pakete installiert sind:

- **PostgreSQL-Server**
- **ASP.NET Core 10.0 Runtime** (z. B. `aspnetcore-runtime-10.0` über das Microsoft-Repository)
- **Nginx** (als Reverse Proxy)
- **Certbot** (für kostenlose Let's Encrypt SSL-Zertifikate)

---

## 1. Veröffentlichung auf dem Entwicklungsrechner

Führen Sie folgende Befehle auf Ihrem Entwicklungsrechner aus, um die Anwendung zu kompilieren, ein Release zu erstellen und die SQL-Migrationsskripte vorzubereiten:

```bash
# 1. Optionale Datensicherung exportieren
dotnet run --project backup -- export mein_umzug.xml --full

# 2. Release-Build für Linux 64-Bit erstellen
dotnet publish -c Release -r linux-x64 --self-contained false -o /home/thorsten/publis/wissen-ahrensburg-de/

# 3. SQL-Migrationsskript für die Produktionsdatenbank generieren
dotnet ef migrations script -o migration.sql
```

## 2. Dateien auf den Server übertragen

Übertragen Sie die generierten Build-Dateien und das Migrationsskript via `rsync` und `scp` auf den Server:

```bash
# Migrationsskript hochladen
scp migration.sql thorsten@ttt.de:/tmp/migration.sql

# Anwendungscode synchronisieren
rsync -avz --exclude 'bin' --exclude 'obj' /home/thorsten/publis/wissen-ahrensburg-de/ tt@ah.city:/var/www/wissen-ahrensburg-de/
```

## 3. Datenbankmigrationen auf dem Server ausführen

Melden Sie sich per SSH auf dem Server an und wenden Sie das Migrationsskript auf die PostgreSQL-Produktionsdatenbank an:

```bash
psql -h localhost -U dein_db_user -d deine_datenbank_name -f /tmp/migration.sql
```

## 4. Systemd-Service konfigurieren

Um die Anwendung als Hintergrunddienst auszuführen und automatisch bei Systemstarts zu laden, erstellen Sie eine Systemd-Service-Datei:

```bash
sudo nano /etc/systemd/system/wissen-ahrensburg-de.service
```

Fügen Sie folgende Konfiguration ein (angepasst an Ihre Pfade und Benutzer):

```ini
[Unit]
Description=MeinCMS Wiki System (Unix Socket)
After=network.target postgresql.service

[Service]
WorkingDirectory=/var/www/wissen-ahrensburg-de/
ExecStart=/usr/bin/dotnet /var/www/wissen-ahrensburg-de/mvc.dll
Restart=always
RestartSec=10
KillSignal=SIGINT
SyslogIdentifier=meincms
User=www-data
Group=www-data
Environment=ASPNETCORE_ENVIRONMENT=Production
RuntimeDirectory=meincms

[Install]
WantedBy=multi-user.target
```

Aktivieren und starten Sie den Dienst:

```bash
sudo systemctl daemon-reload
sudo systemctl enable wissen-ahrensburg-de.service
sudo systemctl start wissen-ahrensburg-de.service
```

Prüfen Sie den Status, um sicherzustellen, dass die Anwendung läuft:

```bash
sudo systemctl status wissen-ahrensburg-de.service
```

## 5. Nginx als Reverse Proxy einrichten

Da die Anwendung so konfiguriert ist, dass sie über einen Unix Domain Socket kommuniziert ( standardmäßig unter `/var/www/wissen-ahrensburg-de/meincms.sock` oder ähnlich, entsprechend der Konfiguration in der `appsettings.json` unter `Kestrel:UnixSocket`), konfigurieren Sie Nginx wie folgt:

Erstellen Sie eine Konfigurationsdatei für Nginx:

```bash
sudo nano /etc/nginx/sites-available/wissen-ahrensburg-de.conf
```

Inhalt der Konfigurationsdatei:

```nginx
server {
    listen 80;
    listen [::]:80;
    server_name doc.wissen-ahrensburg.de wissen-ahrensburg.de;

    # Weiterleitung auf HTTPS (wird durch Certbot automatisch aktualisiert, oder hier manuell eintragen)
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name doc.wissen-ahrensburg.de wissen-ahrensburg.de;

    ssl_certificate /etc/letsencrypt/live/wissen-ahrensburg.de/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/wissen-ahrensburg.de/privkey.pem;

    location / {
        proxy_pass         http://unix:/var/www/wissen-ahrensburg-de/meincms.sock;
        proxy_http_version 1.1;
        proxy_set_header   Upgrade $http_upgrade;
        proxy_set_header   Connection keep-alive;
        proxy_set_header   Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header   X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header   X-Forwarded-Proto $scheme;
    }
}
```

Aktivieren Sie die Konfiguration und starten Sie Nginx neu:

```bash
sudo ln -s /etc/nginx/sites-available/wissen-ahrensburg-de.conf /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

## 6. SSL-Zertifikat mit Certbot generieren

Falls noch kein SSL-Zertifikat vorliegt, können Sie es mit Certbot erzeugen lassen:

```bash
sudo certbot --nginx -d wissen-ahrensburg.de -d doc.wissen-ahrensburg.de
```
