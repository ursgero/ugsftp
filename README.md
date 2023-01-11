# Projekt ugsftp

** Rust ein SFTP Backup client für ein Databackup für  Swissbackup @ Informaniak.com **

* Usage: ugsftp [OPTIONS]

Options:
  -s, --server <SERVER>          [env: SERVER=] [default: localhost]
  -u, --user <USER>              [env: USER=root] [default: user]
  -p, --password <PASSWORD>      [env: PASSWORD=] [default: password]
  -c, --configfile <CONFIGFILE>  [env: CONFIGFILE=] [default: control.txt]
  -h, --help                     Print help information*


*** Beschreibung ***

Das Programm liest, wenn vorhanden, die vorgegebene Konfigurationsdatei.
Der Dateiname wird dabei angegeben mit dem Parameter -c, ist keiner angegeben
sucht das Programm nach der Datei /etc/ugftp/control.txt

Die Konfigurationsdatei liegt immer im Ordner /etc/ugftp/
und darf nur für den root (0x600) Benutzer lesbar sein.

Beispiel einer Konfigurationsdatei: ( Standard: /etc/ugftp/control.txt ):

locdir=/backup/
remdir=/remotedir/
rmhost=sftp.xyz.infomaniak.com
kaewor=Passwort
person=Benutzername
