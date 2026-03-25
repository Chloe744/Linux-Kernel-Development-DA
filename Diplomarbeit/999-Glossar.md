
# Glossar

*LKM*  
Loadable Kernel Module. Zur Laufzeit ladbares Kernelmodul zur Erweiterung der Kernel-Funktionalität.

*KBuild*  
Buildsystem des Linux-Kernels, das unter anderem das Bauen externer Module ermöglicht [@docs_kbuild_external_modules].

*VFS*  
Virtual File System. Abstraktionsschicht über verschiedene Dateisysteme.

*MMU*  
Memory Management Unit. Hardwareeinheit für virtuelle und physische Adressierung.

*Interrupt*  
Asynchrones Hardwareereignis, das vom Kernel über Handler verarbeitet wird.

*Ownership, Borrowing, Lifetimes*  
Rust Konzepte zur Kontrolle von Besitz, Referenzen und Lebensdauern, die Speicherfehler reduzieren.

*no std*  
Kompilation ohne Rust-Standardbibliothek. Nutzung von `core` und bei Heap-Bedarf von `alloc` [@rust_alloc_docs].

*Out-of-Tree-Modul*  
Externes Kernelmodul, das getrennt vom Kernel-Quellcode gebaut wird und über KBuild gegen Kernel-Header kompiliert [@docs_kbuild_external_modules].

*Use after free*  
Fehler, bei dem auf Speicher zugegriffen wird, der zuvor freigegeben wurde. Das kann zu Abstürzen, Datenkorruption oder Sicherheitslücken führen.

*Ungültiger Zeigerzugriff*  
Zugriff über einen Zeiger, der auf keine gültige Speicheradresse zeigt, zum Beispiel durch Nullzeiger, nicht initialisierte Zeiger oder bereits freigegebene Speicherbereiche.

*Datenrennen*  
Fehler in nebenläufigen Programmen, bei dem mehrere Threads gleichzeitig auf dieselben Daten zugreifen und mindestens ein Zugriff schreibend ist, ohne ausreichende Synchronisation. Das Ergebnis ist oft nicht deterministisch und schwer zu debuggen.

*Adressraum*  
Bereich von Speicheradressen, den ein Programm oder der Kernel verwenden kann. Im Userspace hat jeder Prozess einen eigenen virtuellen Adressraum. Im Kernelspace teilen sich Kernel und Treiber einen gemeinsamen Adressraum mit privilegiertem Zugriff.

LLVM (*Low Level Virtual Machine*) ist eine modulare Compilerinfrastruktur, die aus verschiedenen Komponenten besteht und von vielen modernen Programmiersprachen verwendet wird. LLVM stellt unter anderem Backend-Technologien bereit, die für die Codeoptimierung und die Generierung von Maschinencode zuständig sind.

*Clang* ist ein Frontend für die Programmiersprachen C, C++ und Objective-C, das auf LLVM aufbaut. Es übernimmt das Parsen und Analysieren von Quellcode und übersetzt diesen in eine Zwischendarstellung, die anschließend von LLVM weiterverarbeitet wird.

*libclang* ist eine Bibliothek, die Funktionen von Clang als Programmierschnittstelle bereitstellt. Werkzeuge wie *bindgen* können damit C-Headerdateien analysieren und deren Strukturen automatisiert in andere Formate – in diesem Fall Rust-Bindings – übersetzen.

*Buildtree* bezeichnet im Kontext des Linux-Kernels das Verzeichnis, das alle für den Kompilierungsprozess benötigten und während des Builds erzeugten Dateien enthält. Dazu gehören beispielsweise generierte Headerdateien, Objektdateien, Konfigurationsdateien sowie Makefiles und weitere Buildskripte. Der Buildtree stellt somit die Arbeitsumgebung des Kernel-Buildsystems dar. Bei der Entwicklung externer Kernelmodule wird in der Regel der Buildtree des aktuell laufenden Kernels verwendet, um sicherzustellen, dass das Modul mit derselben Kernelkonfiguration und denselben Headerdateien kompiliert wird wie der Kernel selbst. Dieser befindet sich üblicherweise im Verzeichnis `/lib/modules/<kernelversion>/build`.