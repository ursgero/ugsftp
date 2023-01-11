# Projekt ugsftp

## Linux basiertes Rust Programm

**Rust ein SFTP Backup client für ein Databackup für  Swissbackup @ Informaniak.com**

*Usage: ugsftp [OPTIONS]*

*Options:*
  | kurz | lang | Parameter | |
  | :--- | :----------- | :------------- | :------------------------------------------ |
  | -s | --server | <SERVER> |         [env: SERVER=] [default: localhost]|
  | -u | --user |<USER>      |        [env: USER=root] [default: user]|
  | -p | --password |<PASSWORD> |     [env: PASSWORD=] [default: password]|
  | -c | --configfile| <CONFIGFILE> |  [env: CONFIGFILE=] [default: control.txt]|
  | -h | --help      | |              Print help information|


***Beschreibung***

Das Programm liest, wenn vorhanden, die vorgegebene Konfigurationsdatei. Ermittelt aus dieser
einen Quellpfad (lokales Filesystem) und Zielpfad (Remotefilesystem),
vergleicht die jeweils in den Ordner liegenden Dateien auf Basis des Dateialters und kopiert/überträgt
geänderte und neue Dateien mit dem sftp Protokoll auf das Remotefilesystem.

Es werden dabei keine Dateien gelöscht, sollte also eine Datei auf dem lokalen System gelöscht worden sein,
bleibt diese auf dem remote System bestehen.

Der Konfigurationsdateiname wird dabei angegeben mit dem Parameter -c, ist keiner angegeben
sucht das Programm nach der Datei /etc/ugftp/control.txt

Die Konfigurationsdatei liegt immer im Ordner /etc/ugftp/
und darf nur für den root Benutzer lesbar sein.

## Beispiel Parameter einer Konfigurationsdatei:

**( Standard: /etc/ugftp/control.txt ):**

*locdir=/backup/*

*remdir=/remotedir/*

*rmhost=sftp.xyz.infomaniak.com*

*kaewor=Passwort*

*person=Benutzername*




## Aufruf des Programmes

wechseln sie in den Projektordner */src*

starten Sie

*sudo [RUST_LOG=trace] cargo run -v [-- [--user=<user>] [-s <destination_url>] [-p <passwort>] [-c <congfigfile>]]*



***ende des Readme***
