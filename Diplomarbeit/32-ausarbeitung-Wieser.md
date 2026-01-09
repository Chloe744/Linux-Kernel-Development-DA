# Teilaufgabe Wieser
\textauthor{Wieser}

## Theoretische Grundlagen der Kernel Treiberentwicklung

### Einführung

Der Kernel bildet das zentrale Element eines Betriebssystems. Er verwaltet Hardware Ressourcen, steuert Prozesse und stellt grundlegende Systemdienste bereit. Als systemnahe Schicht sorgt er dafür, dass Anwendungen kontrolliert und sicher auf Ressourcen wie Speicher, Geräte und Dateien zugreifen können.

Ein Gerätetreiber ist eine Softwarekomponente, die dem Kernel die Kommunikation mit konkreter Hardware oder einer Gerätekategorie ermöglicht. Typische Beispiele sind Netzwerkschnittstellen oder USB Geräte. Im Linux Kernel wird dabei zwischen Kernel Subsystemen und Gerätetreibern unterschieden. Kernel Subsysteme stellen eine allgemeine Infrastruktur für bestimmte Aufgabenbereiche bereit, etwa für Geräteverwaltung oder Netzwerke, und definieren dabei Schnittstellen sowie grundlegende Abläufe. Gerätetreiber nutzen diese Subsysteme, um die hardwareabhängige Logik umzusetzen und ein konkretes Gerät anzubinden.

Da Treiber im Kernelspace ausgeführt werden, haben sie einen direkten Einfluss auf Stabilität, Sicherheit und Leistung des gesamten Systems. Grundlagen wie Initialisierung, Zugriffsschnittstellen und Aufräumlogik sind daher zentrale Bestandteile der Kernel Entwicklung [@docs_driver_basics].


Ziel dieses Kapitels ist es, die wichtigsten Konzepte der Linux Treiberentwicklung verständlich darzustellen. Darauf aufbauend soll der spätere Vergleich einer Implementierung in C mit einer Implementierung in Rust nachvollziehbar werden.

### Kernelspace und Userspace

Ein wesentliches Konzept moderner Betriebssysteme ist die Trennung zwischen Userspace und Kernelspace. Anwendungen im Userspace werden mit eingeschränkten Rechten ausgeführt. Fehler in einer Anwendung betreffen normalerweise nur den eigenen Prozess. Der Kernelspace ist dagegen privilegiert. Code im Kernelspace hat direkten Zugriff auf Hardware und Speicher. Fehler können daher das gesamte System destabilisieren oder sicherheitsrelevante Schwachstellen verursachen.

Treiber und Kernelmodule werden im Linux System typischerweise im Kernelspace ausgeführt. Aus diesem Grund sind Themen wie Speicherverwaltung, Synchronisation nebenläufiger Abläufe, der Umgang mit Interrupts und saubere Fehlerbehandlung besonders wichtig [@docs_driver_basics].

### Aufbau eines Kernelmoduls

Linux unterstützt Erweiterungen durch *Loadable Kernel Modules*. Solche Module können zur Laufzeit geladen und entladen werden. Dadurch lassen sich Funktionen nachrüsten, ohne das System neu zu starten.

Ein Kernelmodul folgt meist einem klaren Ablauf. Beim Laden wird eine Initialisierung ausgeführt, in der Ressourcen angefordert und Schnittstellen registriert werden. Danach stellt das Modul seine Funktionalität bereit, zum Beispiel über Dateioperationen oder über die Anbindung an ein Kernel Subsystem. Beim Entladen muss die gesamte Registrierung zurückgenommen und belegter Speicher wieder freigegeben werden. Für die Modul Einbindung werden in C unter anderem `module_init` und `module_exit` verwendet [@docs_driver_basics]. Das Erstellen und Bauen von Modulen erfolgt über *KBuild* [@docs_kbuild_external_modules].

### Rust im Linux Kernel

Rust wurde entwickelt, um systemnahe Programmierung mit erhöhten Sicherheitsgarantien zu ermöglichen. Konzepte wie *Ownership*, *Borrowing* und *Lifetimes* sollen typische Fehlerklassen wie *Use after free*, *ungültige Zeigerzugriffe* oder *Datenrennen* bereits zur Compile Zeit reduzieren.

Rust ist seit Linux Kernel Version 6.1 im Mainline Kernel enthalten. Nach einer mehrjährigen Integrationsphase wurde Rust im Dezember 2025 als stabil unterstützte Sprache im Kernel akzeptiert. Rust soll C nicht ersetzen, sondern als zusätzliche Option für neue Komponenten und Treiber dienen [@thenewstack_rust_2025] [@heise_rust_kernel_2025].

Die Rust Schnittstellen im Kernel sind vorhanden und werden weiter ausgebaut. Gleichzeitig sind nicht alle Subsysteme vollständig über Rust erreichbar. Die offizielle Kernel Dokumentation enthält alle Grundlegenden Information für den Einstieg in die Rust Kernel Entwicklung und alle Programmiernormen welche befolgt werden müssen.[@docs_kernel_rust_index] [@docs_kernel_rust_quickstart].

### Vorteile von Rust im Kernelkontext

Ein zentraler Vorteil von Rust liegt in der strikten Typ und Speichersicherheit. Viele Fehler, die in C erst zur Laufzeit auftreten, werden durch den Rust Compiler verhindert oder zumindest schwerer möglich gemacht. Gerade im Kernelspace ist das besonders hilfreich.

Rust bietet zudem eine klare Strukturierung durch Module, Traits und explizite Fehlerbehandlung mit `Result` und `Option`. Was Wartbarkeit und Lesbarkeit verbessert.

### Herausforderungen bei der Nutzung von Rust

Der Einsatz von Rust im Linux Kernel ist mit Voraussetzungen verbunden. Die Kernel Konfiguration muss Rust Unterstützung aktivieren, und die Toolchain muss zu Kernel und Buildsystem passen. Informationen dazu findet man wieder in den Rust Dokumentationen [@docs_kernel_rust_quickstart]. Zusätzlich ist zu beachten, dass C und Rust Code im Kernel koexistieren. Das beeinflusst Workflows, Reviews und Schnittstellen, da große Teile der Kernel Infrastruktur historisch auf C ausgerichtet.

## Kernelarchitektur und Funktionsweise

Der Linux Kernel ist monolithisch aufgebaut, bietet aber durch Module eine modulare Erweiterbarkeit. Zentrale Komponenten wie Prozessverwaltung, Speicherverwaltung, Dateisysteme, Netzwerk und Gerätetreiber laufen im selben *Adressraum*.

Treiber sind ein integraler Bestandteil des Systems. Systemaufrufe aus dem Userspace gelangen über definierte Schnittstellen in den Kernel und werden dort von Subsystemen oder Treibern verarbeitet. Da Kernel Subsysteme und Treiber eng zusammenarbeiten, wirken sich Fehler in einem Teil schnell auf das gesamte System aus. Deshalb sind Codequalität, Speichersicherheit und korrekte Synchronisation in der Kernel Entwicklung besonders wichtig.


## Kernel Buildsystem und Modulkompilierung

### KBuild und externe Module

Das Linux Kernel Buildsystem basiert auf `make` und dem KBuild System. Externe Module, die nicht direkt in den Kernel Quellcode integriert sind, werden als Out of Tree Module gegen die installierten Kernel Header und die passende Kernel Konfiguration gebaut [@docs_kbuild_external_modules].

```makefile
obj-m += hello_rust.o

all:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean
```

### Buildprozess eines Moduls

Der Buildprozess umfasst Vorverarbeitung, Kompilation und das Erzeugen einer Kernel Objekt Datei. Danach kann ein Modul geladen und entladen werden. Für die Praxis sind Werkzeuge wie `insmod`, `modprobe`, `lsmod`, `modinfo` und `dmesg` relevant, da sie Laden, Status und Logs sichtbar machen [@docs_kbuild_external_modules].

### Besonderheiten beim Rust Build

Rust Module werden im Kernel nicht mit Cargo gebaut, sondern über KBuild. Der Rust Compiler wird dabei vom Kernel Buildsystem angesteuert. Rust Code im Kernel wird ohne die Rust Standardbibliothek kompiliert. Stattdessen werden `core` und, falls Heap genutzt wird, `alloc` verwendet [@rust_alloc_docs] [@docs_kernel_rust_index] [@docs_kernel_rust_general_info].

## Praktische Arbeit in Rust

### Rust for Linux Projekt

Das Rust for Linux Projekt verfolgt das Ziel, Kernelmodule und Treiber sicherer zu implementieren, ohne die Kontrolle und Performance systemnaher Entwicklung zu verlieren [@docs_kernel_rust_index] [@docs_kernel_rust_general_info] [@docs_kernel_rust_quickstart]. Rust war innerhalb des Kernels eine Lange Zeit im Experimentellen Zustand, wie es bereits C++ vorher war. Mit Ende 2025 hat Rust jedoch offiziellen Support im Linux Kernel erhalten und ist damit auch die erste weitere Programmiersprache der dies gelungen ist [thenewstack_rust_2025] [@heise_rust_kernel_2025] [@lwn_rust_debate_2025].

### Aufbau eines Rust Kernelmoduls

Im Gegensatz zu C nutzt Rust kein Header System. Die Strukturierung erfolgt über Module. Die Registrierung geschieht über ein zentrales Makro, während Initialisierung und Aufräumen über definierte Schnittstellen abgebildet werden[@docs_kernel_rust_index] [@docs_kernel_rust_quickstart].

```rust
#![no_std]
#![no_main]
use kernel::prelude::*;

module! {
    type: HelloRust,
    name: "hello_rust",
    author: "Wieser",
    description: "Beispiel Rust Kernelmodul",
    license: "GPL",
}

struct HelloRust;

impl KernelModule for HelloRust {
    fn init() -> Result<Self> {
        pr_info!("Hello from Rust kernel module\n");
        Ok(HelloRust)
    }
}

impl Drop for HelloRust {
    fn drop(&mut self) {
        pr_info!("Goodbye from Rust kernel module\n");
    }
}
```

#### Erklärung des Beispielmoduls

Das Modul besteht aus mehreren klar getrennten Teilen.

1. Die Attribute `#![no_std]` und `#![no_main]` signalisieren, dass der Code ohne Rust Standardbibliothek kompiliert wird und keine Main Funktion besitzt.

2. `use kernel::prelude::*;` bindet grundlegende Typen und Traits ein, die für Rust Kernel Entwicklung benötigt werden.

3. Das `module!` Makro definiert Metadaten wie Name, Autor und Lizenz. Zusätzlich legt es den Rust Typ fest, der als Modulinstanz verwendet wird.

4. Die Struktur `HelloRust` repräsentiert den Modul Zustand. In diesem minimalen Beispiel enthält sie keine Daten.

5. `KernelModule::init()` wird beim Laden des Moduls ausgeführt. Hier kann Initialisierung erfolgen. Im Beispiel wird lediglich eine Log Meldung ausgegeben.

6. `Drop` wird beim Entladen des Moduls ausgeführt. Hier können Ressourcen freigegeben werden. Im Beispiel wird ebenfalls nur geloggt.

### Rust spezifische Eigenschaften im Kernel

Rust erzwingt eine explizite Behandlung von Besitz und Lebensdauern. Das reduziert typische Speicherfehler, die in C durch rohe Pointer entstehen können. Für hardwarenahe Operationen ist weiterhin `unsafe` möglich, wird aber bewusst markiert und kann dadurch in Reviews leichter geprüft werden.

Fehlerbehandlung erfolgt über typisierte Rückgabewerte, wodurch Fehlersituationen sichtbar bleiben. Synchronisationsmechanismen sind so gestaltet, dass fehlerhafte Nebenläufigkeit erschwert wird [@docs_kernel_rust_general_info; @lwn_rust_debate_2025].

## Begriffs und Abkürzungsverzeichnis

*LKM*  
Loadable Kernel Module. Zur Laufzeit ladbares Kernelmodul zur Erweiterung der Kernel Funktionalität.

*KBuild*  
Buildsystem des Linux Kernels, das unter anderem das Bauen externer Module ermöglicht [@docs_kbuild_external_modules].

*VFS*  
Virtual File System. Abstraktionsschicht über verschiedene Dateisysteme.

*MMU*  
Memory Management Unit. Hardwareeinheit für virtuelle und physische Adressierung.

*Interrupt*  
Asynchrones Hardware Ereignis, das vom Kernel über Handler verarbeitet wird.

*Ownership, Borrowing, Lifetimes*  
Rust Konzepte zur Kontrolle von Besitz, Referenzen und Lebensdauern, die Speicherfehler reduzieren.

*no std*  
Kompilation ohne Rust Standardbibliothek. Nutzung von `core` und bei Heap Bedarf von `alloc` [@rust_alloc_docs].

*Out of Tree Modul*  
Externes Kernelmodul, das getrennt vom Kernel Quellcode gebaut wird und über KBuild gegen Kernel Header kompiliert [@docs_kbuild_external_modules].

*Use after free*  
Fehler, bei dem auf Speicher zugegriffen wird, der zuvor freigegeben wurde. Das kann zu Abstürzen, Datenkorruption oder Sicherheitslücken führen.

*Ungültiger Zeigerzugriff*  
Zugriff über einen Zeiger, der auf keine gültige Speicheradresse zeigt, zum Beispiel durch Null Zeiger, nicht initialisierte Zeiger oder bereits freigegebene Speicherbereiche.

*Datenrennen*  
Fehler in nebenläufigen Programmen, bei dem mehrere Threads gleichzeitig auf dieselben Daten zugreifen und mindestens ein Zugriff schreibend ist, ohne ausreichende Synchronisation. Das Ergebnis ist oft nicht deterministisch und schwer zu debuggen.

*Adressraum*  
Bereich von Speicheradressen, den ein Programm oder der Kernel verwenden kann. Im Userspace hat jeder Prozess einen eigenen virtuellen Adressraum. Im Kernelspace teilen sich Kernel und Treiber einen gemeinsamen Adressraum mit privilegiertem Zugriff.


