
# Analytischer Vergleich zwischen C und Rust im Linux-Kernel
\textauthor{Amadeo Wieser, Moritz Zugaj}

Nachdem wir sowohl ein Kernelmodul in C als auch ein Kernelmodul in Rust implementiert haben, kann die praktische Erfahrungen der beiden Ansätze miteinander verglichen werden. Ziel dieses Kapitels ist es, Unterschiede im Entwicklungsprozess, in der Toolchain, im Aufbau der Treiber sowie in der praktischen Umsetzung zu analysieren.

## Vergleich der Implementierung

Sowohl der C Treiber als auch der Rust Treiber implementieren die gleiche grundlegende Funktionalität. In beiden Fällen handelt es sich um einen einfachen Character Device Treiber mit folgenden Funktionen:

- open
- release
- read
- write

Das Ziel war es, eine möglichst vergleichbare Implementierung zu erstellen, damit Unterschiede zwischen den beiden Programmiersprachen besser analysiert werden können.

### Moduldefinition

Der erste Unterschied zeigt sich bereits bei der Definition des Kernelmoduls.

C verwendet klassische Makros zur Registrierung des Moduls welche alle genau eine Funktion haben.

```c
MODULE_LICENSE("GPL");
MODULE_AUTHOR("Zugaj");
MODULE_DESCRIPTION("Simple char device");

module_init(device_init);
module_exit(device_exit);
```

Rust verwendet stattdessen ein spezielles Makro, welches ähnliche Funktionen übernimmt.

```rust
module! {
    type: CharTestRustModule,
    name: "chartest_rust",
    author: "Wieser",
    description: "Char device module in Rust",
    license: "GPL",
}
```

Während C einzelne Makros für verschiedene Eigenschaften nutzt, bündelt Rust diese Informationen innerhalb eines einzigen Makros. Dieses registriert das Modul automatisch beim Kernel.

### Dateioperationen

In C wird dazu eine Struktur definiert, welche Funktionspointer enthält.

```c
struct file_operations fops = {
    .open = device_open,
    .read = device_read,
    .write = device_write,
    .release = device_release
};
```

In Rust wird ein Trait implementiert, der die benötigten Funktionen definiert.

```rust
impl FileOperations for CharTestFile {
    type Data = ();

    fn open(_context: &(), _file: &File) -> Result<Self::Data> {
        pr_info!("device opened\n");
        Ok(())
    }
}
```

Statt Funktionspointer in einer Struktur zu speichern, implementiert Rust also eine Schnittstelle (Trait), welche die benötigten Funktionen vorgibt.

### Zugriff auf Userspace Speicher

In C erfolgt dies über spezielle Funktionen wie `copy_from_user`.

```c
copy_from_user(kernel_buffer, user_buffer, length);
```

Rust verwendet stattdessen abstrahierte Kernel APIs.

```rust
reader.read_slice(&mut buf[..n])?;
```

Diese abstrahierten Funktionen reduzieren das Risiko von typischen Speicherfehlern.

## Entwicklungsaufwand

Der größte Unterschied zwischen beiden Ansätzen zeigt sich im Entwicklungsaufwand.

Die Entwicklung eines einfachen Kernelmoduls in C ist relativ unkompliziert. Die notwendigen Werkzeuge sind in den meisten Linux Distributionen bereits vorhanden und die Dokumentation ist sehr umfangreich.

Rust hingegen erfordert eine deutlich komplexere Entwicklungsumgebung. Neben dem Rust Compiler müssen zusätzliche Komponenten installiert und korrekt konfiguriert werden.

Zusätzlich musste der Linux Kernel mit aktivierter Rust Unterstützung selbst kompiliert werden.

Ein großer Teil der praktischen Arbeit bestand daher zunächst darin, eine funktionierende Entwicklungsumgebung einzurichten.

## Toolchain und Buildsystem

Die Toolchain stellt einen weiteren großen Unterschied zwischen C und Rust dar.

C Kernelmodule werden direkt über das etablierte Kernel Buildsystem kompiliert. Der Buildprozess basiert auf dem Kbuild System und verwendet typischerweise den GNU Compiler (gcc).

Rust Module hingegen benötigen zusätzliche Schritte, da Rust nicht direkt Teil der ursprünglichen Kernel Toolchain war. Auch wenn Rust mittlerweile offiziell in den Kernel integriert wurde, basiert ein großer Teil der Infrastruktur weiterhin auf C.

Besonders wichtig ist dabei das Tool **bindgen**. Dieses Tool analysiert C Headerdateien und erzeugt daraus Rust Bindings, damit Rust Code auf bestehende C Funktionen zugreifen kann. 

Dadurch entsteht eine enge Verbindung zwischen Rust und der bestehenden C Infrastruktur des Kernels.

## Sicherheit und Speicherverwaltung

Das zentrales Ziel der Integration von Rust in den Linux Kernel ist die Verbesserung der Speichersicherheit.

C bietet sehr große Freiheiten beim Umgang mit Speicher. Diese Freiheit kann jedoch auch zu Fehlern führen. Typische Probleme sind beispielsweise:

- Buffer Overflows
- Use-After-Free Fehler
- Nullpointer Dereferenzen
- Race Conditions

Bei Rust können viele dieser Fehler bereits zur Compilezeit verhindert werden. Durch das sogenannte Ownership System und den Borrow Checker wird sichergestellt, dass Speicher korrekt verwaltet wird.

Dadurch können eine große von Speicherfehlern vermieden werden, bevor der Code überhaupt ausgeführt wird.

Gerade im Kernelspace, wo Fehler das gesamte Betriebssystem zum Absturz bringen können, stellt das einen erheblichen Vorteil dar.

## Vergleich der beiden Ansätze

| Kategorie | C Kernelmodul | Rust Kernelmodul |
|:--|:--|:--|
|Entwicklungsaufwand|relativ gering|deutlich höher|
|Toolchain|gcc, Kernel Buildsystem|rustc, bindgen, LLVM, clang|
|Speicherverwaltung|manuell|Ownership System|
|Fehlersicherheit|gering|deutlich höher|
|Integration in Kernel|vollständig etabliert|noch in Entwicklung|

: Vergleich zwischen C und Rust Kernelmodulen

## Praktische Erfahrungen

Die praktische Umsetzung zeigt deutlich, dass Rust im Linux Kernel bereits nutzbar ist, sich jedoch noch in einer relativ frühen Entwicklungsphase befindet.

Während der C Treiber ohne größere Probleme kompiliert und getestet werden konnte, traten beim Rust Modul mehrfach Kompatibilitätsprobleme mit der Toolchain auf.

Besonders Versionsunterschiede führten wiederholt zu Buildfehlern während der Entwicklung.

## Fazit

Wir beide sind fest der Überzeugung, dass Rust im Linux Kernel großes Potenzial besitzt und langfristig eine immer wichtigere Rolle in der Kernelentwicklung spielen wird.

Die Sprache bietet durch ihr Speichersicherheitsmodell klare Vorteile gegenüber C und kann helfen, viele typische Fehler bereits zur Compilezeit zu verhindern.

Beim Schreiben des eigentlichen Codes bemerkt man eine Sache schnell, C in Linux hat zwar starke Programmierguidelines gibt dem Entwickler aber immernoch viel Freiheit was zu vielen vermeidbaren Fehlern führen kann. Durch das Ownership-System, den Borrow-Checker und das Typsystem zwingt Rust den Entwickler dazu, viele potenzielle Fehler bereits während der Implementierung zu berücksichtigen. Dies kann den Entwicklungsprozess zwar verlangsamen, führt jedoch langfristig zu robusterem und besser überprüfbarem Code.

Gleichzeitig zeigt die praktische Umsetzung jedoch, dass die Entwicklung von Rust Kernelmodulen derzeit noch mit einem deutlich höheren Aufwand verbunden ist. Besonders die Einrichtung der Toolchain und die Kompatibilität zwischen verschiedenen Komponenten stellt aktuell eine riesige Herausforderung dar.

Eines der größten Probleme ist, dass Rust im Kernel weiterhin stark mit C verbunden ist. Durch die Verwendung von FFI Schnittstellen greift Rust häufig auf bestehende C Implementierungen zurück. Rust ersetzt den bestehenden C Code im Kernel daher momentan nicht vollständig, sondern ergänzt diesen nur.

Seit Dezember 2025 ist Rust offiziell Teil des Linux Kernels und wird nicht mehr als experimentelles Feature betrachtet. Dennoch wird es vermutlich noch einige Zeit dauern, bis Rust in größerem Umfang für Kernelentwicklung eingesetzt wird.

Langfristig wird Rust sicher einen wichtigen Beitrag dazu leisten, die Stabilität und Sicherheit des Linux Kernels weiter zu verbessern.