# Wartung: Backup, Update & Reparatur

Dieses Kapitel behandelt wiederkehrende administrative Aufgaben im laufenden Betrieb von MeinCMS.

---

## 1. System-Updates einspielen

Wenn Sie Codeänderungen oder Updates einspielen möchten, führen Sie folgende Schritte aus:

1. Navigieren Sie lokal in Ihr Projektverzeichnis:
   ```bash
   cd /home/thorsten/wissen-ahrensburg.de
   ```
2. Kompilieren Sie das Projekt im Release-Modus neu:
   ```bash
   dotnet publish -c Release -r linux-x64 --self-contained false -o /home/thorsten/publis/wissen-ahrensburg-de/
   ```
3. Synchronisieren Sie die geänderten Dateien zum Server (Konfigurationsdateien ausschließen, um Produktionsdaten nicht zu überschreiben):
   ```bash
   rsync -avz --exclude 'bin' --exclude 'obj' --exclude 'config' --exclude 'appsettings.json' /home/thorsten/publis/wissen-ahrensburg-de/ tt@ah.city:/var/www/wissen-ahrensburg-de/
   ```
4. Führen Sie auf dem Server ausstehende Migrationen aus:
   ```bash
   dotnet mvc.dll --migrate
   ```
5. Starten Sie ggf. den Dienst neu:
   ```bash
   sudo systemctl restart wissen-ahrensburg-de.service
   ```

---

## 2. Backup & Restore (Daten-Import / Export)

Das System verfügt über ein CLI-Tool namens `backup`, um Artikeldaten speichereffizient zu sichern. Das gerenderte HTML wird beim Export ausgeschlossen (spart ca. 70% Dateigröße) und beim Import automatisch neu generiert.

### Daten exportieren

Sichern Sie die gesamte Datenbank (inklusive aller Mandanten):

```bash
# Auf dem Server im Installationsverzeichnis:
sudo ./backup export meine_sicherung.xml --full

# Oder über das Projekt-Verzeichnis mittels dotnet:
dotnet run --project backup -- export meine_sicherung.xml --full
```

### Daten importieren

Spielen Sie ein Backup wieder in die Datenbank ein:

```bash
# Importieren einer spezifischen Datei:
sudo ./backup import thomas.xml

# Oder aus einem Unterverzeichnis:
sudo ./backup import thorsten/thomas.xml

# Über dotnet:
dotnet run --project backup -- import thomas.xml
```

### Remote-Backup per SSH & SCP erstellen

Sie können Backups auch direkt von Ihrem lokalen Entwicklungsrechner über eine SSH/SCP-Pipeline initiieren und herunterladen:

```bash
# 1. Backup auf dem Server über SSH anstoßen
ssh hhhh@wissen-ahrensburg.de "cd /var/www/wissen-ahrensburg-de && sudo ./backup export /root/meine_sicherung.xml --full"

# 2. Die erstellte Sicherung auf den lokalen PC herunterladen
scp jjjjj@wissen-ahrensburg.de:/root/meine_sicherung.xml /home/thorsten/Downloads/meine_sicherung.xml
```

---

## 3. HTML-Reparatur (Database Repair)

Wenn Sie Änderungen am **MediaWiki-** oder **Markdown-Parser** vornehmen, stimmt das in der Datenbank gespeicherte HTML der existierenden Artikel möglicherweise nicht mehr mit den neuen Parser-Regeln überein. 

In diesem Fall können Sie das HTML aller Artikel automatisch neu generieren lassen:

```bash
dotnet run --project backup -- repair
```
*Dieses Kommando liest alle Wiki-Inhalte ein, parst sie erneut mit der aktuellen Engine und speichert das frisch generierte HTML zurück in die Datenbank.*

---

## 4. Benutzer- und Rollenverwaltung

Das Projekt `UserAdmin` bietet ein Konsolenmenü zur schnellen Erstellung oder Anpassung von Administratorenkonten direkt in der Datenbank:

```bash
dotnet run --project UserAdmin
```
Folgen Sie den Anweisungen im interaktiven Menü der Konsolenanwendung.
