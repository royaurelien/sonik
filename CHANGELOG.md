# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-11-16

### Added

- Automatic synchronization of local folders to USB devices and external drives
- Binary index system for fast incremental sync (only changed files)
- Hot-plug device detection via `/proc/self/mountinfo` monitoring
- Real-time file watching with inotify and configurable debouncing
- Two operation modes: CLI (`plainsync run`) for manual sync and daemon (`plainsyncd`) for continuous monitoring
- YAML configuration supporting multiple devices with multiple folder mappings
- Progress bars showing file counts, transfer sizes and completion percentage
- Desktop notifications for sync events (start, completion, errors)
- Comprehensive path validation ensuring source readability and target writability
- Atomic index updates to prevent data corruption
- systemd user service support for automatic daemon startup
