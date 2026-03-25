\newpage

# Projektabschlussbericht
\textauthor{Wieser}

## Erfolgsmessung

### Erreichung Leistungs-/Qualitätsziele

Das grundlegende Ziel dieser Diplomarbeit bestand darin, die Entwicklung von Linux-Kernel-Treibern zu untersuchen und zwei funktional vergleichbare Implementierungen in den Programmiersprachen C und Rust zu erstellen. Zusätzlich sollte der Entwicklungsprozess dokumentiert und ein Vergleich zwischen den beiden Ansätzen durchgeführt werden.

Dieses Ziel konnte im Wesentlichen erreicht werden. Der Treiber in der Programmiersprache C wurde erfolgreich implementiert, kompiliert und getestet. Dadurch konnten grundlegende Mechanismen der Kernelmodulentwicklung praktisch nachvollzogen werden.

Bei der Rust-Implementierung zeigte sich jedoch, dass die Integration von Rust in den Linux-Kernel aktuell noch mit erheblichen technischen Herausforderungen verbunden ist. Besonders die Einrichtung der Entwicklungsumgebung sowie die Kompatibilität zwischen Kernelversion, Rust-Compiler und der notwendigen Buildtools erwiesen sich als komplex.

Obwohl der Rust-Treiber nicht in allen Fällen vollständig getestet werden konnte, lieferte gerade dieser Umstand wichtige Erkenntnisse über den aktuellen Stand der Rust-Integration im Linux-Kernel. Die praktischen Schwierigkeiten konnten in der Arbeit dokumentiert und analysiert werden, wodurch ein realistischer Einblick in die aktuelle Entwicklungssituation gegeben werden kann.

Aus diesem Grund kann das ursprüngliche Projektziel – die praktische Untersuchung und der Vergleich der beiden Ansätze – insgesamt als erreicht betrachtet werden.

### Erreichung Terminziele

Die geplanten Meilensteine des Projekts konnten größtenteils eingehalten werden. Die einzelnen Projektphasen, insbesondere die Recherche, die Entwicklung der Treiber sowie die Erstellung der schriftlichen Dokumentation, wurden innerhalb des vorgesehenen Zeitrahmens durchgeführt.

In einigen Projektabschnitten kam es jedoch zu einem erhöhten Zeitaufwand, insbesondere während der Einrichtung der Entwicklungsumgebung für die Rust-Implementierung. Mehrere Kompatibilitätsprobleme zwischen Kernelversion, Rust-Compiler und Rust-Toolchain führten dazu, dass zusätzliche Zeit für Fehlersuche und Anpassungen aufgewendet werden musste.

Trotz dieser Schwierigkeiten konnte der Gesamtzeitplan eingehalten werden und die Diplomarbeit wurde fristgerecht fertiggestellt.

### Erreichung Kosten-/Aufwandsziele

Für die Durchführung dieses Projekts waren keine nennenswerten finanziellen Kosten erforderlich. Die gesamte Entwicklung erfolgte auf vorhandener Hardware sowie mit frei verfügbarer Open-Source-Software.

Sowohl der Linux-Kernel als auch die benötigten Entwicklungswerkzeuge wie Compiler, Rust-Toolchain und weitere Tools stehen kostenlos zur Verfügung. Daher konnten die ursprünglich geplanten Kosten vollständig eingehalten werden.

Der tatsächliche Aufwand des Projekts lag hauptsächlich im Zeitaufwand für Recherche, Entwicklung und Dokumentation.

## Reflexion / Lessons Learned

### Teamarbeit

Die Zusammenarbeit im Projektteam verlief insgesamt sehr positiv. Die Aufgaben wurden zwischen den Projektmitgliedern aufgeteilt, wobei sich einer von uns stärker auf die Implementierung des C-Treibers konzentrierte, während der andere den Rust-Treiber entwickelte.

Durch diese Aufgabenteilung konnte eine parallele Bearbeitung der beiden Implementierungen erfolgen. Gleichzeitig fand ein regelmäßiger Austausch über technische Probleme und Lösungsansätze statt.

Diese Zusammenarbeit ermöglichte es, unterschiedliche Perspektiven auf die Treiberentwicklung zu gewinnen und die Ergebnisse gemeinsam zu analysieren.

### Projektmanagement

Im Verlauf des Projekts zeigte sich, wie wichtig eine strukturierte Planung und klare Aufgabenteilung ist. Besonders bei technischen Projekten kann es immer wieder zu unerwarteten Problemen kommen, beispielsweise durch inkompatible Softwareversionen oder unzureichend dokumentierte Schnittstellen.

Ein wichtiger Teil des Projektmanagements bestand daher darin, flexibel auf solche Probleme zu reagieren und alternative Lösungswege zu finden. In einigen Fällen musste der ursprüngliche Plan angepasst werden, um technische Hindernisse zu überwinden.

Die Dokumentation der einzelnen Arbeitsschritte stellte dabei sicher, dass der Fortschritt des Projekts jederzeit nachvollziehbar blieb.

### Sonstige Lernerfahrungen

Während der Arbeit an diesem Projekt konnten umfangreiche Kenntnisse im Bereich der Systemprogrammierung und der Linux-Kernelentwicklung gewonnen werden.

Dazu gehören unter anderem:

* grundlegendes Verständnis der Linux-Kernelarchitektur
* Entwicklung und Kompilierung von Kernelmodulen
* Arbeit mit dem Linux-Kernel-Buildsystem
* Einrichtung komplexer Entwicklungsumgebungen
* Umgang mit Compiler- und Toolchain-Problemen
* praktische Erfahrungen mit Rust im Kernelkontext

Besonders deutlich wurde, wie komplex die Entwicklung von Kernelsoftware sein kann und wie wichtig eine stabile und kompatible Entwicklungsumgebung ist.

### Nachhaltigkeitsanalyse

Im Rahmen der Nachhaltigkeitsanalyse wird untersucht, inwiefern diese Diplomarbeit einen Bezug zu den Sustainable Development Goals (SDGs) der Vereinten Nationen aufweist.

Die Arbeit leistet insbesondere einen indirekten Beitrag zu folgenden Zielen:

**SDG 9 – Industrie, Innovation und Infrastruktur**

Die Untersuchung moderner Programmiersprachen im Bereich der Betriebssystementwicklung trägt zur Weiterentwicklung von Softwaretechnologien bei. Die Integration von Rust in den Linux-Kernel stellt einen innovativen Ansatz dar, um die Sicherheit und Stabilität von Systemsoftware zu verbessern.

**SDG 4 – Hochwertige Bildung**

Die Diplomarbeit dient als Lern- und Forschungsprojekt im Bereich der Systemprogrammierung. Die dokumentierten Ergebnisse können auch für zukünftige Schüler oder Studierende als Grundlage dienen, um sich mit Linux-Kernelentwicklung und moderner Systemsprache auseinanderzusetzen.

Darüber hinaus basiert die gesamte Arbeit auf Open-Source-Technologien. Linux, Rust sowie die verwendeten Entwicklungswerkzeuge stehen frei zur Verfügung und fördern damit eine offene und nachhaltige Softwareentwicklung.

Ein negativer Einfluss auf ökologische oder soziale Nachhaltigkeitsaspekte ist im Rahmen dieses Projekts nicht zu erwarten, da keine zusätzliche Hardware produziert oder Ressourcen in größerem Umfang verbraucht wurden.