<h1 align="center"> WiFi Analyzer (Rust) </h1>
<p align="center">
  <img alt="Version" src="https://img.shields.io/badge/version-1.0-blue.svg?cacheSeconds=2592000" />
</p>

> Analyse des trames WiFi (Beacon) et détection de DroneID à partir de fichiers PCAP

---

### 🏠 [Homepage](https://github.com/EtudiantEnsea)

# TP1_rust

Un analyseur de trames WiFi développé en Rust dans le cadre du TP réseau.

---

## Description

`TP1_rust` est un outil permettant d’analyser des fichiers PCAP contenant des trames WiFi (IEEE 802.11).

Le programme :

* Lit les paquets via la bibliothèque `pcap`
* Détecte les trames **Beacon**
* Extrait les informations principales :

  * Adresse MAC
  * SSID
* Détecte les trames **DroneID** via les champs Vendor Specific

---

## Fonctionnalités

* Analyse de fichiers PCAP
* Parsing des trames Radiotap et 802.11
* Extraction des champs TLV
* Détection de réseaux WiFi
* Détection de drones (DroneID)
* Export des résultats en **JSON** ou **CSV**

---

## Architecture

```text
src/
├── main.rs        # Interface CLI
├── lib.rs         # Bibliothèque principale
├── parser.rs      # Analyse des paquets
├── tlv.rs         # Parsing TLV
└── output.rs      # Export JSON / CSV
```

---

## Compilation

```sh
cargo build
```

---

## Utilisation

```sh
cargo run -- --pcap data/capture.pcap --output-format json --output-file results.json
```

---

## Exemple de sortie

```json
[
  {
    "mac": "34:27:92:37:ff:2a",
    "ssid": "Freebox-37FF29",
    "is_drone": false
  }
]
```

---

## Fonctionnement

1. Lecture du fichier PCAP
2. Extraction de l’en-tête Radiotap
3. Analyse des trames 802.11
4. Filtrage des trames Beacon (type 0, subtype 8)
5. Parsing TLV :

   * `0x00` → SSID
   * `0xdd` → Vendor Specific (DroneID)
6. Export des résultats

---

## Prérequis

* Rust installé
* Npcap (Windows)

---

## Documentation

```sh
cargo doc --no-deps --open
```

---

## Auteur

**Loick GOMES GRANCHO & Abdoul Nouroudine SANA**
