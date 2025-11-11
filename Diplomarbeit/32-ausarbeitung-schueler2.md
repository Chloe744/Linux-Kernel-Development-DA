# Teilaufgabe Schüler 1  
\textauthor{Wieser}

## Theoretische Grundlagen der Kernel-Treiberentwicklung

### Einführung  
In modernen Betriebssystemen nimmt der Kernel eine zentrale Rolle ein: Er verwaltet Ressourcen, steuert Hardwarezugriff, stellt Dienste für Prozesse bereit und bildet damit die systemnahe Schicht zwischen Applikationen und der Hardware. Ein „Treiber“ (englisch *driver*) ist dabei eine Softwarekomponente, die dem Kernel ermöglicht, mit einem bestimmten Gerät oder einer Gerätekategorie zu kommunizieren – sei es eine Netzwerkkarte, ein Speichergerät oder ein Sensor.  
Ziel dieser Ausarbeitung ist die Untersuchung und Analyse der Treiberentwicklung im Kernel-Kontext, insbesondere im Hinblick auf einen Vergleich zwischen traditioneller C-Implementierung und moderner Rust-Entwicklung.

### Kernelspace vs. Userspace  
Ein wesentliches Unterscheidungsmerkmal bei der Treiberentwicklung ist die Frage: Wird der Code im *Userspace* oder im *Kernelspace* ausgeführt?  
* **Userspace**: Dienste oder Bibliotheken laufen im Kontext eines Benutzerprozesses mit begrenzten Rechten. Fehler hier wirken meist isoliert.  
* **Kernelspace**: Der Treiber läuft mit hohen Rechten im Kernel-Adressraum. Fehler können das gesamte System destabilisieren oder zum Absturz führen.  
Die Entwicklung von Treibern im Kernelspace erfordert daher besondere Sorgfalt bezüglich Sicherheit, Stabilität und Ressourcenkonflikten. Aspekte wie Race-Conditions, direkte Hardwarezugriffe, Speicherverwaltung und Interrupt-Handling sind kritisch.

### Aufbau eines Kernel-Moduls  
Ein Treiber oder einfaches Modul im Kernel lässt sich typischerweise in folgenden Schritten strukturieren:  
1. **Initialisierung**: Beim Laden des Moduls werden Initialisierungsfunktionen ausgeführt (z. B. `module_init`).  
2. **Registrierung/Schnittstellen**: Anbindung an Subsysteme (z. B. Char-Device, `procfs`, `sysfs`).  
3. **Operationen**: Implementierung der Treiberfunktionen (open, read, write, ioctl, Interrupt-Handler etc.).  
4. **Aufräumarbeiten**: Beim Entfernen (`module_exit`) erfolgen Deregistrierungen und Freigaben.  
In C zählen dazu u. a. `MODULE_LICENSE`, `MODULE_AUTHOR`, `obj-m`/`obj-y` im Makefile und das KBuild-System.

### Rust im Linux-Kernel (Einordnung)  
Seit Kernel 6.1 ist Rust als **experimentelle** Zweitsprache im Mainline-Kernel vorhanden. Ziel ist es, Speicher- und Thread-Sicherheit (Ownership, Borrowing) in sicherheitskritischen Kernelkomponenten zu nutzen. Derzeit ist die API-Abdeckung noch begrenzt; viele Subsysteme sind teils verfügbar, die Integration wird aber kontinuierlich ausgebaut.

#### Vorteile
- **Speicher-/Typsicherheit** (z. B. weniger Use-after-free, Buffer Overflows)  
- **Moderne Sprachfeatures** (Zero-cost Abstractions, Traits, Pattern Matching)  
- **Wartbarkeit/Lesbarkeit**  

#### Herausforderungen  
- **Unvollständige Abdeckung** aller Kernel-Subsysteme  
- **Toolchain-Voraussetzungen** (`CONFIG_RUST=y`, `rustc` nightly, `#![no_std]`)  
- **Ökosystem/Prozess**: Integration in C-dominierte Maintainer-Workflows

---

## Kernelarchitektur und Funktionsweise
- Linux ist **monolithisch, aber modular**: Funktionalität kann via **Loadable Kernel Modules (LKM)** zur Laufzeit ergänzt/entfernt werden.  
- Treiber sind **integraler Bestandteil** des Kernel-Ökosystems: Systemaufrufe (z. B. `open()`) gehen über die System-Call-Schnittstelle in den Kernel, wo Treiber/Subsysteme den Aufruf verarbeiten und Ergebnisse an den Userspace zurückgeben.  
- Diese Architektur bedingt **hohe Sicherheitsanforderungen** an Treibercode (Memory-Safety, Synchronisation) – ein Hauptmotiv für die Rust-Integration.

---

## Kernel-Buildsystem und Modulkompilierung

### KBuild (Out-of-Tree Module)  
Das Kernel-Buildsystem basiert auf `make`/KBuild. Externe (Out-of-Tree) Module werden gegen die aktuell installierte Kernel-Version gebaut.

```makefile
obj-m += hello_rust.o

all:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean
```

### Buildprozess eines Moduls
1. Präprozessor  
2. Kompilation  
3. Verlinkung → `.ko` (Kernel Object)  
4. (optional) Signierung  
5. Laden/Entladen mit `insmod`/`modprobe`, Status via `lsmod`, `modinfo`, Logs via `dmesg`

### Besonderheiten beim Rust-Build  
- Kein `cargo` im Kernel; stattdessen **KBuild ruft `rustc` (nightly)** auf  
- **`#![no_std]`**, Nutzung von `core`/`alloc` (Kernel-Allocator)  
- Rust-Module über **Makros/Bindings** (z. B. `module!`, `kernel::printk!`)  
- Templates/Guides im *Rust-for-Linux*-Projekt

### Signierung/Kompatibilität  
Kernel-Version und Signatur müssen passen; Abweichungen führen zu `Invalid module format`.  
`modinfo` zeigt Metadaten (Lizenz, Autor, vermutete Kernelversion). Signaturprüfung via `CONFIG_MODULE_SIG=y`.

---

## Praktische Arbeit (konzeptionell): Rust im Linux-Kernel

### Überblick: Rust-for-Linux‐Projekt  
Ziel: Kernel-Module/Treiber **sicherer** implementieren, ohne Performance einzubüßen.  
Stand (high-level): **experimentell stabil**; Kerninfrastruktur vorhanden, API-Abdeckung wächst. Unterstützt werden u. a. Modul-/Build-Makros (`module!`, `kernel::printk!`), Primitiven (`Mutex`, `SpinLock`, `Arc`) und einfache Subsysteme (z. B. Char-Devices).

### Aufbau eines Rust-Kernelmoduls  
Rust nutzt **kein Header-System**, sondern **Module/Sichtbarkeit** (`mod`, `pub`). Kernpunkte:
- Einstieg in `lib.rs`/`mod.rs`  
- Registrierung über `module!{ … }` (ersetzt in C: `MODULE_*`, `module_init/exit`)  
- Initialisierung via `KernelModule::init()`, Aufräumen via `Drop`‐Impl

_Schematischer Aufbau (nicht zum 'builden' gedacht):_
```rust
#![no_std]
#![no_main]
use kernel::prelude::*;

module! {
    type: HelloRust,
    name: "hello_rust",
    author: "Wieser",
    description: "Rust-Modul (schematisch)",
    license: "GPL",
}

struct HelloRust;

impl KernelModule for HelloRust {
    fn init() -> Result<Self> {
        pr_info!("Hello from Rust kernel module!\n");
        Ok(HelloRust)
    }
}

impl Drop for HelloRust {
    fn drop(&mut self) {
        pr_info!("Goodbye from Rust kernel module!\n");
    }
}
```

### Rust-spezifische Eigenschaften im Kernelkontext  
- **Ownership/Borrowing/Lifetimes:** Compiler-gestützte Vermeidung von Use-after-free/Datenrennen; für Hardware-nahes Arbeiten sind gezielte `unsafe {}`-Blöcke möglich.  
- **Fehlerbehandlung:** `Result<T,E>`/`Option<T>` erzwingen explizites Error-Handling statt stiller Fehlercodes.  
- **Synchronisation:** `SpinLock<T>`, `Mutex<T>`, `Arc<T>` sind *thread-safe by design*; reduzieren typische Concurrency-Fehler.  
- **Speicherverwaltung:** `alloc` + Kernel-Allocator; sichere Container (z. B. `Box<T>`, `Arc<T>`) statt manuellem `kmalloc/kfree`.

---

## Begriffs- und Abkürzungsverzeichnis (Glossar)
  
- **LKM (Loadable Kernel Module):** Zur Laufzeit ladbares Kernelmodul, erweitert Kernel-Funktionalität ohne Reboot.  
- **VFS (Virtual File System):** Abstraktionsschicht über verschiedenen Dateisystemen (z. B. ext4, tmpfs).  
- **MMU (Memory Management Unit):** Hardwareeinheit zur Verwaltung virtueller/physischer Adressen; Basis für Speicherschutz.  
- **Scheduler:** Kernelkomponente, die CPU-Zeit an Threads/Prozesse zuteilt.  
- **Interrupt (IRQ):** Asynchrones Ereignis der Hardware, wird durch Interrupt-Handler im Kernel bedient.  
- **`procfs` / `sysfs` / `debugfs`:** Pseudo-Dateisysteme zur Sicht-/Steuerung von Kernelzuständen aus dem Userspace.  
- **Char-/Block-Device:** Zeichengerät (stromorientiert) vs. Blockgerät (blockweise I/O); erscheinen unter `/dev`.  
- **Major/Minor:** Nummern, die Gerätetreibern (Major) und deren Instanzen (Minor) zugeordnet sind.  
- **inode:** Dateisystem-Struktur mit Metadaten (ohne Dateiname/Pfad).  
- **`struct file_operations` (fops):** C-Struktur mit Funktionszeigern für Dateioperationen eines Treibers.  
- **`ioctl`:** Schnittstelle für gerätespezifische Steuerbefehle über File-Deskriptoren.  
- **KBuild / Kconfig:** Kernel-Buildsystem / Konfigurationssprache zur Steuerung von Features/Abhängigkeiten.  
- **`obj-m` / Out-of-Tree:** Kennzeichnung externer Module; Bau gegen installierte Kernel-Headers.  
- **`insmod` / `modprobe` / `lsmod` / `modinfo`:** Werkzeuge zum Laden/Entladen/Inspektieren von Modulen.  
- **`dmesg` / `printk` / `pr_info!`:** Kernel-Logausgabe und Nutzerwerkzeuge zur Einsicht in Logs.  
- **Slab-Allocator:** Kernel-Speicherallokator für häufig genutzte, gleich große Objekte.  
- **Spinlock / Mutex / RCU:** Synchronisationsmechanismen für nebenläufigen Kernelcode.  
- **Ownership / Borrowing / Lifetimes (Rust):** Regeln, die Speicher-/Aliasierungsfehler zur Compile-Zeit vermeiden.  
- **`unsafe` (Rust):** Markierung für Operationen außerhalb der Sprachgarantien (z. B. rohe Pointer, HW-Zugriff).  
- **`#![no_std]` (Rust):** Kompilieren ohne Rust-Standardbibliothek; Nutzung von `core`/`alloc`.  
- **`Arc<T>` / `Box<T>` (Rust):** Referenzgezählte bzw. besitzende Zeigerarten mit definierten Lebensdauern.  
- **`Result<T,E>` / `Option<T>` (Rust):** Typisierte Fehler-/Optionalitätsbehandlung statt globaler Errorcodes.  
- **Netlink:** Socket-basierte Schnittstelle für Kernel↔Userspace-Kommunikation (z. B. Netzwerk-Konfiguration).

---

