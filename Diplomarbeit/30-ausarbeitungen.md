# Aufgabenstellung

## Auftraggeber

Die Diplomarbeit entstand in Zusammenarbeit mit der Fachhochschule, an der der Vater des Hernn Zugaj aus unserem Team tätig ist. Über diesen Kontakt ergab sich die Möglichkeit, ein technisches Thema aus dem Bereich der Systemprogrammierung als Diplomarbeit zu bearbeiten. Die Fachhochschule fungierte dabei als externer Auftraggeber und unterstützte das Projekt vor allem in organisatorischer Hinsicht.

Der thematische Schwerpunkt wurde jedoch in erster Linie durch das persönliche Interesse von uns beiden. Beide von uns beschäftigen sich privat mit Open-Source-Software und insbesondere mit dem Linux-Betriebssystem. Aus diesem Interesse heraus entstand die Idee, sich im Rahmen der Diplomarbeit mit der Entwicklung von Linux-Kernel-Treibern zu beschäftigen.

Der Auftraggeber versprach sich von der Arbeit vor allem eine strukturierte Aufarbeitung eines aktuellen technischen Themas. Durch die praktische Umsetzung und Analyse der Treiberentwicklung sollte ein nachvollziehbarer Einblick in die Arbeitsweise der Linux-Kernelentwicklung entstehen. Die Ergebnisse der Arbeit können darüber hinaus als Beispiel für zukünftige Projekte oder als Einstieg in die Kernelprogrammierung dienen.

## Ausgangssituation

Linux ist eines der weltweit am häufigsten eingesetzten Betriebssysteme und bildet die Grundlage für zahlreiche Server, Embedded-Systeme sowie mobile Geräte. Die Kommunikation zwischen Hardware und Betriebssystem erfolgt dabei über sogenannte Gerätetreiber, die direkt im Linux Kernel implementiert werden.

Traditionell werden Linux-Treiber in der Programmiersprache C entwickelt. C ermöglicht direkten Zugriff auf Speicher und Hardware und bietet damit die notwendige Kontrolle für systemnahe Programmierung. Gleichzeitig bringt diese Freiheit jedoch auch Risiken mit sich, da Fehler in der Speicherverwaltung oder Nebenläufigkeit schwerwiegende Auswirkungen auf die Stabilität des gesamten Systems haben können.

In den letzten Jahren wurde daher begonnen, die Programmiersprache Rust schrittweise in den Linux Kernel zu integrieren. Rust verfolgt einen anderen Ansatz als C und versucht durch verschiedene Sprachmechanismen viele typische Fehler der Systemprogrammierung bereits während der Kompilierung zu verhindern.

Die Integration von Rust in den Linux Kernel stellt daher eine interessante Entwicklung im Bereich der Betriebssystementwicklung dar. Ziel dieser Diplomarbeit war es, diese Entwicklung näher zu untersuchen und die Unterschiede zwischen der klassischen Treiberentwicklung in C und einer Implementierung in Rust praktisch zu analysieren.