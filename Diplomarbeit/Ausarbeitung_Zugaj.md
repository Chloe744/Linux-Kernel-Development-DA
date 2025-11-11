# Diplomarbeit Linux Treiber Mitschrift
*Mo Jo*  
August 2025

## Vorwort
Das Ziel dieses Dokuments ist die Aufzeichnung meiner Erfahrungen und anderen erwähnenswerten Notizen, welche für die Ausarbeitung der Diplomarbeit zum Thema Linux Treiber notwendig sind. Zudem werden mit der Verwendung von LaTeX wichtige Erfahrungen gesammelt, die später bei der eigentlichen Ausarbeitung von Nutzen sein werden.

## Erkenntnisse

### Grundlagen zur Kernelprogrammierung in C
Die 5 wichtigsten Aspekte, die man vor dem Starten der Entwicklung verstehen muss.

#### Linux Kernel
Der Kernel eines Linux-Betriebssystems ist für die Speicher- und Prozessverwaltung zuständig. Er bildet eine unabhängige Schnittstelle für Software, welche auf die Schnittstelle zugreifen kann, ohne eine Ahnung von der Hardware zu haben. Der Linux Kernel ist ein modularer monolithischer Kernel, was so viel bedeutet, dass der Kernel nicht nur Funktionen für die Kommunikation zwischen Prozessen, sondern auch Treiber von Hardware bietet. Noch dazu kommt aber, dass der Kernel wie erwähnt auch modular ist, bedeutet, er kann durch jegliche Module verbessert werden.

#### Module
Module sind Codeteile welche auf dem Linux Kernel geladen und entfernt werden und dienen dazu, die Fähigkeiten des Kernels zu erweitern, ohne dabei einen Systemneustart zu verlangen. Ein Beispiel für ein wichtiges Modul im Linux Kernel ist der Geräte Treiber, der dafür dient, einen Schnittpunkt zwischen Hardware- und Kernelinteraktion herzustellen. Damit ein Modul funktioniert und richtig geladen werden kann, muss es mindestens aus einer Startfunktion (init_module) und einer Endfunktion (cleanup_module) bestehen, damit der Code richtig geladen und komplett entfern werden kann, so als hätte er niemals existiert.

#### Makefiles
Um die Kompilierung von den Quelltextdateien zu Programmmodulen bzw. Objekten zu steuern und nach Belieben anzupassen, verwendet man ein Makefile. Mit Makefile kann eine beliebige Anzahl an Quelltextdateien Kompilieren aber auch miteinander zu einem einzelnen Programm Linken, das alles funktioniert, mit den sogenannten Targets, welche mit einem Doppelpunkt sichtbar markiert sind sie zeigen, welche Dateien kompiliert und miteinander verbunden werden sollen, da man auch ein Objekt aus mehreren Quelltextdateien erstellen kann. 

```
    obj_name := sourcefile_name.o

    [target]: #dependent   
        #Command
```

Wichtig bei Makefiles zu beachten ist, dass statt Abständen Tabs verwendet werden müssen!

#### Header Files
Damit die Module fehlerfrei funktionieren, muss man die benötigten Header Files für den Kernel installieren. Die Header Files sind im Grunde Interfaces, welche Funktionen definieren, damit der Compiler weiß, ob diese richtig auf Basis der Signatur benutzt werden. Diese Header Files werden dann direkt als Erweiterung für den Kernel installiert, auf Ubuntu funktioniert das mit den Commands: 

```
    sudo apt-get update 
    apt-cache search linux-headers-`uname -r`
    sudo apt-get install linux-headers-`uname -r`
```

**Wichtig!** Ich hatte anfangs Probleme mit Packages wie  **printk.h** da meine Konfigurationsdatei (c_cpp_properties.json) nicht im Projektordner war.

#### Risiken
Die Stärke der Modulprogrammierung ist der Einfluss welchen das Modul auf den Kernel haben kann, was jedoch schnell auch zur Schwäche werden 
kann, da eine Zugriffsverletzung bei dem Modul auch zu einer Zugriffsverletzung am Kernel führen kann, da Module nicht ihren eigenen Code Bereich haben, sondern sie den des Kernels teilen. Dazu kommt noch, wenn der Code des Moduls in den Kernel geladen wird, dann kann es passieren, dass Variablen Namen gleich sind, kann es zu "Namespacepollution" führen. Deshalb ist es empfehlenswert, Module auf einer virtuellen Maschine oder anderen sicheren Umgebung zu testen, damit der mögliche Schaden keine Rolle spielt.

### Aufbau eines Treiber Moduls
In Linux teilen die meisten Treibermodule einen vorgesehenen Aufbau an Funktionen und anderen wichtigen Merkmalen, die man erlernen kann, jedoch unterscheiden sie sich auch in gewissen Punkten.

#### Device Files
Treiber, Dateien oder wie in Linux genannt Driver Files repräsentieren je eine Art von Hardware, die mit dem Betriebssystem interagieren will bzw. kann, diese Driver Files stellen die Mittel zur Verfügung, damit mit der Hardware kommuniziert werden kann. Driver Files unter Linux befinden sich im /dev Folder und sind wie folgt aufgebaut:

```
brw-rw----  1 root  disk  8, 1 Apr  9  2025 /dev/sda1
```

Die drei wichtigsten Merkmale dieser File sind der erste Buchstabe, welcher die Art des Device repräsentiert (B für block und C für Character), die erste Ziffer nach Disk in dem fall 8 ist die Major Nummer und besagt welcher Treiber für die Kommunikation dieses Gerätes zuständig ist und die Nummer danach, die sogenannte "minor" Nummer die für den Treiber wichtig ist um zwischen seinen zuständigen Geräten zu differenzieren. Die Art des Gerätes ist wie gesagt in Block und Charakter geteilt, wobei Block einen Buffer für Lese und Schreibmethode zur Verfügung stellt, was für Speichergeräte von Vorteil ist, während character die Möglichkeit die Anzahl der Bytes, die sie nutzen, nach Belieben anzupassen was Flexibilität fördert und daher der Typ der meisten Geräte ist. 

\\  
\\
Wie schon erwähnt, befinden sich alle Driver Files im Ordner /dev. Sobald man also mit seiner eigenen Driver File fertig ist, muss man zum Schluss seine File vom Arbeitsordner in /dev geben.

#### Datei Operatoren
Die Struktur der Datei Operatoren (File operators) ist definiert unter (include/linux/fs.h) und beinhaltet Pointer für Funktionen, die im Treiber definiert sind und verschiedene Funktionen auf dem Treiber ausführen. Einer der wichtigsten Operatoren ist das einlesen vom Gerät, welches in jedem Character Gerätetreiber definiert ist, da es ein Must-have ist, im Gegensatz zu gewissen Block-Treibern wo die Funktion einfach mit Null supplementiert wird. 
\\
\\
Durch gcc Erweiterungen ist es heutzutage deutlich einfacher, etwas zu der Struktur zuzuweisen:

```
    struct file_operation fops = {
      read: device_read, 

      write: device_write, 

      open: device_open, 

      release: device_release 
    };
```

\\
\\
Der Kompatibilität halber ist es empfohlen, dass die "fops" Instanz so implementiert werden soll:

```
    struct file_operation fops = {
      .read = device_read, 

      .write = device_write, 

      .open = device_open, 

      .release = device_release 
    };
```

#### Gerät-Registrierung
Wenn ein Character Gerät erreicht werden will, muss eine Geräte File in /dev sein, diese Dateien sind jedoch abstrakt, offen und operieren in Kernel Space. Um einen fertigen Treiber ins System einzufügen, muss es erstmal in den Kernel registriert werden:

```
    int register_chrdev(unsigned int major, const char *name, 
    struct file_operations *fops);
```

Damit die erstellte Geräte Datei alle "Minor" Nummern verwendet gibt es zwei bessere Interfaces die sich nur darin unterscheiden, ob man die "Major" Nummer kennt oder eine dynamisch zugewiesene haben will:

```
    int register_chrdev_region(dev_t from, unsigned count,
    const char *name); 

    int alloc_chrdev_region(dev_t *dev, unsigned baseminor,
    unsigned count, const char *name);
```

Diese abstrakten Dateien sind nicht im "Disk-Space" vorhanden, werden aber von der Datenstruktur Inode (Index Node) repräsentiert. Inode enthält mithilfe von Metadaten alle Informationen über die File, mit zwei ausnahmen, der Name der Datei und der Pfad jedoch dient die Inode Nummer als Pointer zu dem richtigen Inode damit die Metadaten leicht gefunden werden können. 
\\
\\
Wichtig bei der Registrierung ist noch das wir bei der Major Nummer 0 setzten müssen, damit der Kernel uns eine noch nicht zugewiesene Major number zur Verfügung gibt und erst dann können wir folgendermaßen die Gerätedatei (Device File) erstellen.

#### Gerät-Entregistrierung
Es sollte nicht möglich sein, dass Root ein Treiber Modul, was gerade im Linux Kernel einen Prozess durchläuft, mit rmmod entfernt, da es dann zu großen Problemen im Kernel führen kann, da Code von einem anderen Modul inmitten einer Funktion ausgeführt werden kann. Deshalb gibt es einen Zähler, der darauf achtet, wie oft das Modul gerade verwendet wird, wenn dieser Counter auf 0 ist also gerade wird das Modul nicht in Betrieb, dann ist es gestattet auch ein rmmod anzuwenden.

#### Datei-Systeme
Ein verwandtes Thema der erwähnten Inodes sind Dateisysteme. Dateisysteme wie proc erlauben eine weitere Möglichkeit für den Kernel und die Kernel Module, Informationen zu senden und zu Verarbeiten. Aber Proc gibt auch von sich aus wichtige Informationen über den Prozess, wie zum Beispiel Informationen über alle vorhandenen Module oder eine Statistik über die speicher Verwendung. Methode Proc zu erstellen und auszuführen ist sehr ähnlich wie bei den Modulen, da wir eine Struktur erstellen müssen mit allen Informationen der /proc Datei, sowie Pointern zu allen Funktionen und zu guter Letzt haben wir wieder die Init Funktion zum Registrieren und die cleanup zum Endregistrieren. Im simpelsten Fall haben wir dann noch mindestens eine Read-Methode, damit etwas zurückgegeben wird, wenn wir lesen möchten.

### Best Practices im Code
Wenn man sich die auf Github vorhanden Treiber Module ansieht, stoßt man immer wieder auf kleine Tricks im code, die quality of life verbessern. Diese Tricks sind leicht zu implementieren und sollten deshalb, wenn möglich, immer angewandt werden.

#### Macros
Wie schon früher erwähnt braucht ein jedes Modul mindestens eine Init und Cleanup Funktion, damit sie richtig vom System registriert und unregistriert werden können. Der __init und __Exit Macro erlauben den Wegfall der init und cleanup Funktion nach der Verwendung bzw. wenn die Funktion nicht gebraucht wird um RAM speicher freizuräumen. Dies ist natürlich nur bei sogenannten "built-in" Modulen möglich, da bei ladbaren die Funktionen nicht einfach weggeworfen werden dürfen.
\\
\\
Ein weiterer Fall wo Macros hilfreich sind ist, wenn Daten vom Userspace (Prozess) zum Kernelspace (Linux Kernel) transportiert werden müssen. Dies wird zum Beispiel bei der Schreibfunktion von Datei-Systemen gebraucht und dafür gibt es die Macros put_user und get_user für einzelne Zeichen, sowie copy_to_user und copy_from_user. Natürlich war das nur ein Beispiel, da es noch hunderte weitere hilfreiche Macros gibt.
\\
\\
#### Debugging
Für die Fehlersuche und Vermeidung können ebenfalls Macros hilfreich sein. Vor allem der Tracepoint Macro **ftrace** kann Profilabschnitte erstellen, welche benutzt werden können, um komplexe Treiber verstehen und eigenen debuggen zu können. Eine Möglichkeit ist es auch den Kernel neu zu kompilieren um hilfreiche Funktionen wie  MODULE_FORCE_UNLOAD, dies gibt dir die Möglichkeit jegliches Modul mit dem  sudo rmmod -f module Befehl zu entladen, selbst wenn der Kernel es als unsicher ansieht.

\\
\\
#### Coding Stil
Damit man Code, auf den Linux Kernel bekommt, muss man auf den vorgesehen Coding Stil achten, der perfekt einzuhalten ist oder konsequenterweise die Push-Request abgelehnt bekommt. Wenn man es nicht beabsichtigt seinen Code in den Linux Kernel zu committen ist es zwar nicht zwingend, aber sehr empfohlen auf den Stil der Autoren zu wechseln. Dazu zählt:
\\
\\
 **Statements** Über 80 Zeichen sollten geteilt werden, damit man die Lesbarkeit verbessert.
```
    if (device->config_table[CONFIG_INDEX]->subsystem->handler->ops->init(device, CONFIG_MODE_HIGH, true) < 0)
    return -EINVAL;

    //stattdessen:

    if (device->config_table[CONFIG_INDEX]->subsystem->handler->
    ops->init(device, CONFIG_MODE_HIGH, true) < 0)
```
\\
\\
 **Einrückungen** Sind 8 Zeichen lang damit man sie gut erkennen kann.
```
    if(i == 1){
            printf("i ist 1\n"); <-- 8 Zeichen abstand statt 4.  
    }
```
\\
\\
 **Funktionen** Haben die Öffnungsklammer am Anfang der nächsten Linie und alle nicht-Funktionen haben sie am Ende der gleichen Zeile nach dem Vorbild von Kernighan und Ritchie.
```
    static int myIntFunkton(int var)
    { <--
            //code 
    }

    if(var <= 2){ <--
            //code
    }
```
\\
\\
**Typedefs** sollten zur Verständlichkeit des Codes nicht verwendet werden!
```
    vps_t a; //schlecht

    struct virtual_container *a; //in ordnung

```
\\
\\
Dazu kommen noch andere bereits erwähnte Aspekte wie das Bewusstsein bei der Namensgebung der Variablen.

### Schritt für Schritt Vorführung einer Linux Treiber Entwicklung in C
Jetzt wird anhand des bereits besprochenen Aufbaus eines Treibermoduls Theorie in die Praxis umgesetzt und anhand der gesammelten Erfahrung schrittweise ein einfaches Treibermodul erstellt werden.

#### Zielsetzung
Das Treibermodul soll folgende funktionen haben:
- Eine Initial Funktion um vom Kernel geladen werden zu können.
- Eine Öffnungsfunktion, damit man weiß, ob die Gerätedatei bereits geöfnet ist.
- Eine Schließfunktion, damit man weiß, wenn die geöffnete Gerätedatei wieder geschlossen wurde
- Eine schreib Funktion
- Eine Lesefunktion
- Zum Schluss noch eine cleanup Funktion, damit es auch entladen werden kann.

#### 

```
```

