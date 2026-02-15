## Arbeitsanweisung

Andernfalls funktioniert es nicht richtig.

1. Kopieren Sie die Konfigurationsdateien:
    ```bash
    cp mvc/_appsettings.Development.json mvc/appsettings.Development.json
    cp mvc/_appsettings.json mvc/appsettings.json
    ```

2. Erstellen Sie die Datenbank und vergeben Sie die Rechte:
    ```bash
    sudo -u postgres -i
    createdb -E UTF8 -O thorsten mvc
    psql -d mvc -c "GRANT ALL PRIVILEGES ON DATABASE mvc TO thorsten"
    exit
    ```
