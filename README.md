# IPCALC
Minimal IP network calculator. Shows quick and important information about a network.

---

Index:
    - [Features](#Features)
    - [Usage](#Usage)
    - [Build](#Build)

---

## Features
Main features:
    - Parse both IPv4 and IPv6 addresses
    - Accepts addresses in multiple formats:
        - IP/CIDR
        - IP + mask (CIDR)
        - IP + mask (dotted mask)
    - Output information:
        - Address.
        - NetMask.
        - Number of avaliable Hosts.

## Usage

```bash
ipcalc --ip <IP> [--mask <MASK>]

#=====> EXAMPLE <=====#
❯ ipcalc -i 192.168.0.1/24

Address: 192.168.0.1
NetMask: 255.255.255.0
Hosts:   254

❯ ipcalc -i 192.168.0.1 -m 255.255.255.0

Address: 192.168.0.1
NetMask: 255.255.255.0
Hosts:   254
```

## Build
```bash
git clone --depth 1 https://github.com/Schiz0idCat/ipcalc.git
cd ipcalc

cargo build --release
```
