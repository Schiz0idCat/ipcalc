# IPCALC
Minimal IP network calculator. Shows quick and important information about a network.

---

Index:
- [Usage](#Usage)
- [Build](#Build)

---

## Usage

```bash
ipcalc --ip <IP> [--mask <MASK>]

#=====> EXAMPLE <=====#
❯ ipcalc -i 192.168.0.1/24

Address:   192.168.0.1
NetMask:   255.255.255.0
Hosts:     254
Network:   192.168.0.0
Broadcast: 192.168.0.255

❯ ipcalc -i 192.168.0.1 -m 255.255.255.0

Address:   192.168.0.1
NetMask:   255.255.255.0
Hosts:     254
Network:   192.168.0.0
Broadcast: 192.168.0.255
```

## Build
```bash
git clone --depth 1 https://github.com/Schiz0idCat/ipcalc.git
cd ipcalc

cargo build --release
```
