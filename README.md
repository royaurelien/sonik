# sonik

**sonik** is a small Rust utility that synchronizes a local music directory to a mounted device.  
It supports manual sync and automatic sync when the target mountpoint appears.

Configuration follows the XDG standard:

```
~/.config/sonik/config.toml
```

Example:

```toml
source = "/home/user/Music"
target = "/media/user/MyDevice/Music"
```

## Requirements

- Rust (stable)
- A target device that mounts as a real filesystem (no MTP)
- Linux system with inotify support

## Build

Build in debug:

```bash
cargo build
```

Build optimized:

```bash
cargo build --release
```

The binary ends up in:

```
target/release/sonik
```

Install locally:

```bash
cargo install --path .
```

## Configuration

```bash
mkdir -p ~/.config/sonik
nano ~/.config/sonik/config.toml
```

Example:

```toml
source = "/home/user/Music"
target = "/media/user/MyDevice/Music"
```

## Usage

Manual sync:

```bash
sonik run
```

Automatic sync:

```bash
sonik watch
```

Verbose mode:

```bash
RUST_LOG=info sonik watch
```

## Development

```bash
cargo install cargo-watch
cargo watch -x run
cargo test
```

## Optional: systemd user service

Create:

```
~/.config/systemd/user/sonik.service
```

```ini
[Unit]
Description=Sonik auto-sync

[Service]
ExecStart=%h/.cargo/bin/sonik watch
Restart=always

[Install]
WantedBy=default.target
```

Enable:

```bash
systemctl --user enable --now sonik.service
```

## License

MIT
