
# Projekthandbuch
\textauthor{Amadeo Wieser}

## Entwicklungsplan

### Projektauftrag

Im Linux-Kernel werden Treiber traditionell in der Programmiersprache C entwickelt. C bietet dabei direkten Zugriff auf Hardware und Speicher, bringt jedoch auch Risiken mit sich, da Fehler wie ungültige Speicherzugriffe oder Datenrennen schwer zu erkennen sein können und das gesamte System beeinträchtigen können.

Mit der Integration von Rust in den Linux-Kernel wurde ein neuer Ansatz eingeführt, der darauf abzielt, die Entwicklung von Kernelcode sicherer zu machen. Rust besitzt Mechanismen zur Speichersicherheit und zur Kontrolle von Nebenläufigkeit, die viele typische Fehler bereits während der Kompilierung verhindern können.

Im Rahmen dieser Diplomarbeit wurde untersucht, wie sich die Entwicklung eines Linux-Kernel-Treibers in C im Vergleich zu einer Implementierung in Rust gestaltet. Dazu wurden zwei funktional identische Kernelmodule entwickelt und praktisch getestet. Ziel war es, Unterschiede im Entwicklungsprozess, im Aufbau des Codes sowie in der Toolchain und den benötigten Entwicklungsumgebungen zu analysieren.

Die Arbeit konzentriert sich dabei nicht auf die Entwicklung eines komplexen Hardwaretreibers, sondern auf einen didaktischen Vergleich zweier Implementierungen eines einfachen Character-Device-Treibers. Dadurch soll nachvollziehbar werden, welche Vorteile und Herausforderungen sich beim Einsatz von Rust im Linux-Kernel ergeben und wie sich die Entwicklung im Vergleich zur klassischen C-Implementierung gestaltet.


### Projektziele

Das zentrale Ziel dieser Diplomarbeit ist die praktische Untersuchung der Linux-Treiberentwicklung und der Vergleich zweier Programmiersprachen im Kernelkontext. Dazu sollen zwei funktional vergleichbare Kernelmodule entwickelt werden: ein Treiber in der Programmiersprache C sowie ein äquivalenter Treiber in Rust.

Ein wichtiger Teil der Arbeit besteht darin, ein grundlegendes Verständnis der Linux-Kernelarchitektur und der Mechanismen zur Treiberintegration zu erlangen. Dazu gehören insbesondere Konzepte wie Kernelspace und Userspace, Speicherverwaltung, Geräte-Registrierung sowie typische Schnittstellen zwischen Hardware und Betriebssystem.

Darüber hinaus soll untersucht werden, wie sich die Entwicklung eines Kernelmoduls in Rust im Vergleich zur klassischen Implementierung in C gestaltet. Dabei werden sowohl der Entwicklungsprozess als auch Unterschiede in der Speicherverwaltung, der Codequalität, der Fehlersicherheit sowie der verwendeten Toolchain analysiert.

Ein weiteres Ziel ist die systematische Dokumentation des Entwicklungsprozesses. Dazu zählen typische Fehlerquellen, Debugging-Strategien sowie praktische Erfahrungen beim Kompilieren und Testen von Kernelmodulen. Die gewonnenen Erkenntnisse sollen anschließend in der schriftlichen Arbeit zusammengeführt und in Form eines strukturierten Vergleichs dargestellt werden.

Langfristig soll die Arbeit aufzeigen, welche Rolle Rust künftig in der Linux-Kernelentwicklung spielen könnte und welche Vorteile beziehungsweise Herausforderungen sich bei der Verwendung dieser Sprache im Kernelkontext ergeben.

### Nicht-Ziele bzw. Nichtinhalte

Diese Diplomarbeit verfolgt nicht das Ziel, einen vollständig produktionsreifen Hardwaretreiber für ein reales Gerät zu entwickeln. Stattdessen wird ein bewusst vereinfachter Character-Device-Treiber implementiert, der primär als Demonstrations- und Vergleichsbeispiel dient.

Ebenso liegt der Fokus der Arbeit nicht auf der Entwicklung komplexer Hardwarekommunikation oder spezifischer Treiber für bestimmte Geräteklassen. Die Implementierungen dienen vielmehr dazu, grundlegende Mechanismen der Kernelmodulentwicklung verständlich darzustellen.

Darüber hinaus ist es nicht Ziel dieser Arbeit, den Linux-Kernel selbst zu verändern oder bestehende Kernelkomponenten zu ersetzen. Rust wird im Rahmen des Projekts ausschließlich verwendet, um einen vergleichbaren Treiber zu implementieren und dessen Entwicklung mit einer klassischen C-Implementierung zu vergleichen.

Auch eine vollständige Performanceanalyse oder Benchmark-Messung der beiden Implementierungen ist nicht Bestandteil dieser Arbeit. Der Schwerpunkt liegt stattdessen auf dem Entwicklungsprozess, der Struktur des Codes sowie den praktischen Erfahrungen bei der Umsetzung.

### Projektnutzen

Der Nutzen dieser Diplomarbeit liegt vor allem in der praktischen Untersuchung eines aktuellen Themas der Systemprogrammierung. Mit der Integration von Rust in den Linux-Kernel ist erstmals eine neue Programmiersprache offiziell Teil des Kernels geworden. Dadurch entsteht ein relevantes Forschungs- und Entwicklungsfeld, das sowohl für die Open-Source-Community als auch für zukünftige Entwickler von großer Bedeutung ist.

Durch die Entwicklung zweier funktional vergleichbarer Kernelmodule in C und Rust wird ein direkter Vergleich der beiden Ansätze möglich. Die Arbeit zeigt dabei nicht nur theoretische Unterschiede zwischen den Programmiersprachen, sondern dokumentiert auch die praktischen Herausforderungen beim Aufbau der Entwicklungsumgebung, beim Kompilieren des Kernels sowie bei der Implementierung eines Treibers.

Für die Schule entsteht dadurch eine nachvollziehbare Dokumentation der Linux-Treiberentwicklung, die als Lernmaterial für zukünftige Schüler oder Projekte dienen kann. Besonders im Bereich der Systemprogrammierung ist praxisnahes Beispielmaterial selten, weshalb eine verständliche Aufarbeitung dieses Themas einen zusätzlichen Mehrwert bietet.

Darüber hinaus liefert die Arbeit einen Einblick in moderne Entwicklungen der Kernelprogrammierung und zeigt, inwiefern Rust als sichere Systemprogrammiersprache langfristig zur Verbesserung der Stabilität und Sicherheit des Linux-Kernels beitragen könnte.

## Projektauftraggeber/in

Die Diplomarbeit wird in Zusammenarbeit mit der FH Joanneum durchgeführt. Der externe Betreuer der Arbeit ist Thomas Strametz, während die Betreuer der HTL Leoben Ing. DI Dr. Christian Schindler und Mag. Anja Lube sind.

#### Projekttermine

| Termin     | Inhalt                          |
|-----------:|:--------------------------------|
| 12.09.2025 | DA-Portal befüllt |
| 10.11.2025 | 1. DA-Präsentation |
| 09.01.2026 | DA-Erstversion elektronisch an Betreuer übermittelt |
| 26.02.2026 | 2. DA-Präsentation |
| 06.03.2026 | DA-Abgabe |
| 23.03.2026 | DA-Durchsicht mit Betreuer |
| 27.03.2026 | DA-Portal mit Hr. Messner abgeschlossen |
| 07.04.2026 | Abgabe – Bibliotheksversion der DA |
| 08.04.2026 | 3. DA-Präsentation |

: Projektterminübersicht


### Projektkosten

| Meilenstein  | Kostenart | Menge  | Preis   | Gesamtkosten | Deckung durch |
|:-------------|:---------:|:------:|--------:|-------------:|---------------|
| Gebundene DA-Abgabe | Druck | 3 | 27.90€ | 83,70€ | Schüler |

 : Geplante Projektkosten
 
Tatsächlich angefallene Kosten: 0

### Projektrisiken

| Risiko | EW | Auswirkungen | Maßnahmen |
|:--------------:|:---:|:----------------|:--------------|
| Toolchain- oder Versionskonflikte (Rust, Kernel, Compiler) | 35% | Kernel oder Treiber lassen sich nicht kompilieren | Verwendung dokumentierter Versionen und schrittweise Anpassung der Toolchain |
| Fehler im Kernelmodul | 25% | Systemabstürze oder Kernel Panics während Tests | Entwicklung und Tests ausschließlich in einer virtuellen Maschine |
| Hoher Zeitaufwand beim Debugging | 30% | Verzögerungen im Projektfortschritt | Frühzeitiges Testen einzelner Komponenten und regelmäßige Zwischentests |
| Unterschiede zwischen Linux-Distributionen | 20% | Unterschiedliches Verhalten von Kernel oder Toolchain | Nutzung dokumentierter Kernelversionen und Anpassung der Entwicklungsumgebung |
| Verzögerungen im Projektablauf | 15% | Projektabschnitte können nicht rechtzeitig abgeschlossen werden | Regelmäßige Abstimmung im Team und frühzeitige Planung der Arbeitsschritte |

: Projektrisiken

### Projektorganisation

### Projektbeteiligte

| Vorname     | Nachname     | Organisation | Kontaktinfos      |
|:------------|:-------------|:-------------|:------------------|
| Amadeo    | Wieser  | HTL Leoben   | wieser1806@gmail.com  |
| Moritz    | Zugaj  | HTL Leoben   | zugajmor@gmail.com  |
| Christian | Schindler| HTL Leoben | schr@O365.htl-leoben.at |
| Anja    | Lube  | HTL Leoben   | Lan@O365.htl-leoben.at  |
| Thomas  | Strametz  | FH Joanneum  | thomas.strametz2@fh-joanneum.at
 |

: Projektbeteiligte

### Projektrollen

| Projektrolle           | Rollenbeschreibung     | Name              |
|------------------------|------------------------|-------------------|
| Projektleiter | Verantwortlicher für Einhaltung des Projektrahmens | Amadeo Wieser  |
| Auftraggeber | Auftraggeber der externen Diplomarbeit | Thomas Strametz |
| Betreuer | Schulischer Betreuer | Ing. DI Dr. C. Schindler |
| Betreuer | Schulischer Betreuer | Mag. A. Lube |

: Projektrollen

### Vorgehen bei Änderungen

Damit Änderungen im Projekt strukturiert erfolgen können, wurde ein einheitliches Vorgehen definiert. Änderungen können beispielsweise Anpassungen am Zeitplan, an einzelnen Meilensteinen oder an der technischen Umsetzung betreffen.

* Wer wird informiert
    * Alle Projektbeteiligten
    * Schulische Betreuer

* Wer muss zustimmen
    * Projektteam gemeinsam
    * Hauptbetreuer der Diplomarbeit

* Wo werden Änderungen dokumentiert
    * Änderungen am Quellcode werden im Github Repository über Commits dokumentiert
    * Größere Änderungen am Projektumfang oder an der technischen Umsetzung werden zusätzlich in der schriftlichen Arbeit festgehalten
    * Anpassungen am Zeitplan werden im Projektdokument entsprechend aktualisiert

Dieses Vorgehen stellt sicher, dass alle Projektmitglieder über Änderungen informiert sind und Entscheidungen nachvollziehbar dokumentiert werden.

## Meilensteine

### 12.09.2025: Eintragung im DA-Portal abgeschlossen

- Projektidee und Themenbeschreibung wurden im Diplomarbeitsportal eingetragen
- Projektteam und Betreuer wurden im System erfasst
- Grundlegende Projektdaten wurden dokumentiert

### 10.11.2025: Erste Projektpräsentation

- Vorstellung des aktuellen Projektstandes vor Schülern und Lehrpersonen
- Präsentation der Projektidee und der geplanten technischen Umsetzung
- Rückmeldungen und Verbesserungsvorschläge des Betreuers wurden aufgenommen

### 09.01.2026: Erste Version der Diplomarbeit fertiggestellt

- Erste vollständige Version der schriftlichen Arbeit wurde erstellt
- Dokument wurde elektronisch an den Betreuer übermittelt
- Feedback für weitere Überarbeitungsschritte wurde eingeholt

### 26.02.2026: Zweite Projektpräsentation

- Präsentation des aktuellen Fortschritts der Diplomarbeit
- Vorstellung der praktischen Umsetzung und der bisherigen Ergebnisse
- Präsentation wurde auf Basis des Feedbacks der ersten Präsentation verbessert

### 06.03.2026: Fertigstellung der Diplomarbeit

- Schriftliche Arbeit wurde final überarbeitet
- Korrekturen und Verbesserungen wurden eingearbeitet
- Diplomarbeit liegt in abgabefertiger Form vor

### 23.03.2026: Durchsicht mit dem Betreuer

- Gemeinsame Durchsicht der Diplomarbeit mit dem Betreuer
- Letzte inhaltliche und formale Anpassungen wurden besprochen
- Freigabe für die endgültige Abgabe wurde erteilt

### 27.03.2026: Abschluss im DA-Portal

- Alle notwendigen Einträge im Diplomarbeitsportal wurden finalisiert
- Abschluss des Projekts im Portal gemeinsam mit dem Betreuer durchgeführt

### 07.04.2026: Abgabe der Bibliotheksversion

- Gebundene Version der Diplomarbeit wurde abgegeben
- Bibliotheksversion wurde offiziell eingereicht

### 08.04.2026: Abschlusspräsentation

- Präsentation der finalen Diplomarbeit vor Lehrpersonen und Mitschülern
- Vorstellung der Ergebnisse und Erkenntnisse des Projekts
- Letzte Rückmeldungen zur Diplomarbeit erhalten
    

## Anwendungsfälle

Da es sich bei dieser Diplomarbeit nicht um eine klassische Benutzeranwendung handelt, sondern um eine technische Untersuchung der Linux-Kernel-Treiberentwicklung, beziehen sich die Anwendungsfälle auf typische Interaktionen mit Kernelmodulen und deren Entwicklungsprozess.

Die beschriebenen Anwendungsfälle stellen daher typische Szenarien dar, die während der Entwicklung, dem Laden und der Verwendung eines Kernel-Treibers auftreten.

\newpage

### Kernelmodul kompilieren

#### Kurzbeschreibung
Der Entwickler kompiliert ein Kernelmodul (in C oder Rust) mithilfe des Linux Kernel Buildsystems.

#### Trigger
Der Entwickler startet den Buildprozess über ein Makefile.

#### Vorbedingung
* Linux-Kernel-Quellen sind vorhanden
* Kernel-Buildtree ist verfügbar
* Compiler und Toolchain sind korrekt installiert

#### Nachbedingung
Eine kompilierte Kernelmodul-Datei (.ko) wurde erzeugt.

#### Akteure
* Entwickler

#### Standardablauf

1. Entwickler wechselt in das Verzeichnis des Kernelmoduls
2. Der Buildbefehl wird ausgeführt
3. Das Kernel-Buildsystem kompiliert den Quellcode
4. Das fertige Kernelmodul wird erzeugt

#### Fehlersituationen

* Fehlende Abhängigkeiten
* Fehler im Quellcode
* Inkompatible Kernelversion

#### Systemzustand im Fehlerfall
Das Kernelmodul wird nicht erstellt und der Buildprozess bricht mit einer Fehlermeldung ab.

\newpage

### Kernelmodul laden

#### Kurzbeschreibung
Ein zuvor kompiliertes Kernelmodul wird in den laufenden Linux-Kernel geladen.

#### Trigger
Der Entwickler oder Administrator lädt das Modul manuell.

#### Vorbedingung
* Kernelmodul wurde erfolgreich kompiliert
* Benutzer besitzt Administratorrechte

#### Nachbedingung
Das Kernelmodul ist im Kernel aktiv und kann verwendet werden.

#### Akteure
* Administrator
* Entwickler

#### Standardablauf

1. Das Kernelmodul wird mit einem entsprechenden Systembefehl geladen
2. Der Kernel initialisiert das Modul
3. Die Modul-Initialisierungsfunktion wird ausgeführt
4. Das Modul registriert seine Funktionalität im Kernel

#### Fehlersituationen

* Modul ist nicht kompatibel mit der Kernelversion
* Modul enthält Fehler im Initialisierungscode

#### Systemzustand im Fehlerfall
Das Modul wird nicht geladen und der Kernel bleibt unverändert.

\newpage

### Zugriff auf das Gerät über den Treiber

#### Kurzbeschreibung
Ein Programm im Userspace greift über eine Gerätedatei auf den implementierten Kernel-Treiber zu.

#### Trigger
Ein Userspace-Programm öffnet die entsprechende Gerätedatei.

#### Vorbedingung
* Kernelmodul ist geladen
* Gerätedatei wurde erstellt
* Treiber ist korrekt registriert

#### Nachbedingung
Der Treiber verarbeitet die Anfrage und führt die entsprechende Operation aus.

#### Akteure
* Userspace-Programm
* Linux-Kernel
* Kernel-Treiber

#### Standardablauf

1. Ein Programm öffnet die Gerätedatei
2. Der Kernel leitet die Anfrage an den Treiber weiter
3. Der Treiber verarbeitet die Operation
4. Das Ergebnis wird an das Userspace-Programm zurückgegeben

#### Fehlersituationen

* Gerätedatei existiert nicht
* Treiber reagiert mit einem Fehlercode

#### Systemzustand im Fehlerfall
Die Operation wird abgebrochen und eine Fehlermeldung wird zurückgegeben.

\newpage

### Vergleich der Implementierungen

#### Kurzbeschreibung
Die Implementierungen des Treibers in C und Rust werden hinsichtlich Struktur, Speicherverwaltung und Sicherheitsmechanismen analysiert.

#### Trigger
Durchführung der Analysephase im Rahmen der Diplomarbeit.

#### Vorbedingung
* Beide Treiberimplementierungen existieren
* Die Treiber sind kompilierbar und testbar

#### Nachbedingung
Unterschiede und Gemeinsamkeiten der Implementierungen werden dokumentiert.

#### Akteure
* Entwickler
* Diplomarbeitsteam

#### Standardablauf

1. Analyse des C-Treibers
2. Analyse des Rust-Treibers
3. Vergleich der Implementierungsansätze
4. Dokumentation der Unterschiede und Erkenntnisse

#### Fehlersituationen

* Eine Implementierung kann nicht erfolgreich getestet werden

#### Systemzustand im Fehlerfall
Die Analyse wird angepasst oder auf theoretischer Ebene fortgeführt.