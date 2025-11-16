# Plainsync

![Love](https://img.shields.io/badge/Made%20with%20%E2%9D%A4-Rust-orange)
![License](https://img.shields.io/badge/License-MIT-green.svg)
[![Build & Release Plainsync](https://github.com/toolsforfolk/plainsync/actions/workflows/release.yml/badge.svg)](https://github.com/toolsforfolk/plainsync/actions/workflows/release.yml)

## Fast diff-based sync for devices

Originally built for the Innoasis Y1 MP3 player, Plainsync focuses on **syncing folders**, not managing a music library (see [beets](https://github.com/beetbox/beets) for that).  
It performs **diff-based transfers** with **no unnecessary writes**, preserving your device’s lifespan.  
**Fast**, **minimal**, and **adaptable**, it can easily extend to other targets beyond music players.

## Features

- Incremental sync (only changed files)
- Real-time device detection via mount monitoring
- Live file watching with debouncing
- Binary index for fast comparison
- Multi-device support with a simple YAML config
- Progress bars and detailed statistics

## Installation

Download pre-built binaries from the **[Releases page](https://github.com/toolsforfolk/plainsync/releases)**

You will get:

- `plainsync` — CLI tool  
- `plainsyncd` — background daemon  

## Configuration

Run:

```bash
plainsync config edit
```

The config file is typically located at:

```
~/.config/plainsync/config.yaml
```

Example:

```yaml
device:
  - name: MY_DEVICE
    label: "My Device"
    mount: "/media/{user}/{device}"
    mountinfo: true
    folders:
      - source: "~/Music/Library"
        target: "Music"
        enabled: true
```

### Fields

| Field        | Description |
|--------------|-------------|
| **name**     | Unique identifier. Must match the final directory name in the mount path. |
| **label**    | Optional friendly name. |
| **mount**    | Mount path template. Usually `/media/{user}/` (Ubuntu/GNOME) or `/run/media/{user}/` (KDE/Fedora/Arch). |
| **mountinfo**| Use `/proc/self/mountinfo` for detection (**recommended**). |
| **source**   | Local folder. `~` and relative paths are expanded from user home. |
| **target**   | Folder on device (relative to mount). |
| **enabled**  | Enable/disable sync for this folder. |

### Supported placeholders

- `{user}`: username  
- `{uid}`: user ID  
- `{device}`: device directory name  

You may define **multiple devices and folders**.

> If in doubt, use the `plainsync config show` command to check the interpreted values.

## Usage

Manual sync:

```bash
plainsync sync run
```

Daemon:

```bash
plainsyncd
```

### Run in background (systemd user)

If you installed manually:

```bash
mkdir -p ~/.config/systemd/user
cp plainsync.service ~/.config/systemd/user/plainsync.service
systemctl --user daemon-reload
systemctl --user enable --now plainsync.service
```

View logs:

```bash
journalctl --user -u plainsync -f
```

## How It Works

Plainsync keeps a local binary index for each device/folder pairing.  
During sync, it compares:

- current filesystem state  
- previous index  

…to determine exactly what to upload or delete — avoiding unnecessary writes.

The daemon (`plainsyncd`) handles:

- device mount detection  
- directory watching (via inotify)  
- debounced event processing  

### File Change Detection

```mermaid
stateDiagram-v2
    [*] --> CheckPath
    CheckPath --> New : Not in index
    CheckPath --> CheckSize : In index
    
    CheckSize --> Modified : Size differs
    CheckSize --> CheckMtime : Size same
    
    CheckMtime --> Modified : Mtime differs
    CheckMtime --> Unchanged : Mtime same
    
    New --> [*] : Upload
    Modified --> [*] : Upload
    Unchanged --> [*] : Skip
    
    note right of CheckMtime
        mtime = modification time
        from filesystem metadata
    end note
```

## Requirements

- Linux with **inotify** (most modern distros include this by default)
- Target devices must expose a regular file system (**no MTP**)  

## Development

Rust 1.70+

```bash
cargo build
cargo test
cargo clippy
```

## Troubleshooting

**Device not detected**  
Check real mount directory:

```bash
ls -l /media/$USER/
```

**Permission denied**  
Verify write permissions:

```bash
touch /media/$USER/MyUSB/.test && rm /media/$USER/MyUSB/.test
```

**Daemon not syncing**  
Check process:

```bash
ps aux | grep plainsyncd
```

**Want verbose logs?**

```bash
RUST_LOG=debug plainsync sync run
```

## License

MIT
