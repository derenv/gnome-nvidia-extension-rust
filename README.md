<!--[![dependency status](https://deps.rs/repo/github/derenv/gtk4-nvidia-monitor-rust/status.svg)](https://deps.rs/repo/github/derenv/gtk4-nvidia-monitor-rust)
SPDX-FileCopyrightText: 2024 Deren Vural
SPDX-License-Identifier: GPL-3.0-or-later
-->

[![wakatime](https://wakatime.com/badge/user/8ce81774-1d46-4c72-8a15-b5cf4032138f/project/64d5b278-0a3f-4b93-9a34-5cf88c7a8a3a.svg)](https://wakatime.com/badge/user/8ce81774-1d46-4c72-8a15-b5cf4032138f/project/64d5b278-0a3f-4b93-9a34-5cf88c7a8a3a) ![CI](https://github.com/derenv/gtk4-nvidia-monitor-rust/actions/workflows/rust.yml/badge.svg) [![Test Coverage](https://codecov.io/gh/derenv/gtk4-nvidia-monitor-rust/branch/main/graph/badge.svg?token=PHDU5O3VFZ)](https://codecov.io/gh/derenv/gtk4-nvidia-monitor-rust)[![dependency status](https://deps.rs/repo/github/derenv/gtk4-nvidia-monitor-rust/status.svg)](https://deps.rs/repo/github/derenv/gtk4-nvidia-monitor-rust)[![License: GPL v3.0+](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

# Description
Intended as a rewrite of [gnome-nvidia-extension](https://github.com/ethanwharris/gnome-nvidia-extension) as a practise Rust project, this app has developed from a system tray style app into a gtk-rs app for interacting with GPU management interfaces like Nvidia's [nvidia-settings](https://github.com/NVIDIA/nvidia-settings), [nvidia-smi](https://developer.nvidia.com/nvidia-system-management-interface) and [optimus](https://www.nvidia.com/en-gb/geforce/technologies/optimus/). This includes viewing statistics like temperature, clock speeds, VRAM usage but also setting fan-profiles and GPU overclocks (like [GreenWithEnvy](https://gitlab.com/leinardi/gwe/), a major inspiration for this app).

The intention is to provide an alternative to both GPU monitoring and overclocking software like [GreenWithEnvy](https://gitlab.com/leinardi/gwe/) and system tray monitoring programs/extensions like [gnome-nvidia-extension](https://github.com/ethanwharris/gnome-nvidia-extension). Hopefully the app can be extended to AMD's equivalent GPU management interfaces (if they exist) at some point (see the [Roadmap](docs/ROADMAP.md) for the rest of the plan for this app).

The app is written entirely in [gtk-rs](https://gtk-rs.org/), using [gtk4](https://github.com/gtk-rs/gtk4-rs) and [libadwaita](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/).

# Usage
To first install/update the settings schema, use the following:
```bash
bash install_schemas.sh
```

Then, use the following to compile the project:
```bash
cargo clean; cargo build
```

Finally, run the app:
```bash
cargo run
```

# Resources
## Applications
- [Original version, written in GTK-JS](https://github.com/ethanwharris/gnome-nvidia-extension)
- [nvidia-settings](https://github.com/NVIDIA/nvidia-settings)
- [nvidia-smi](https://developer.nvidia.com/nvidia-system-management-interface)
- [optimus](https://www.nvidia.com/en-gb/geforce/technologies/optimus/)
- [GreenWithEnvy (GWE)]()
## Documentation
- [GTK-RS Project website](https://gtk-rs.org/)
- [GTK-RS Book](https://gtk-rs.org/gtk4-rs/stable/latest/book/introduction.html)
- [GTK-RS Documentation](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/)
- [GLib Documentation](https://docs.gtk.org/glib/index.html)
- [GObject Documentation](https://docs.gtk.org/gobject/index.html)
- [Gio Documentation](https://docs.gtk.org/gio/index.html)
- [LibAdwaita-rs Documentation](https://relm4.org/docs/stable/libadwaita/index.html)
- [Gnome HIG](https://developer.gnome.org/hig/index.html)
## Git
- [GTK-RS Organisation on GitHub](https://github.com/gtk-rs)
- [GTK-RS Core](https://github.com/gtk-rs/gtk-rs-core)
- [GTK4-RS](https://github.com/gtk-rs/gtk4-rs)
- [LibAdwaita-rs](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/)

## Crates
### GTK libraries
- [gtk4](https://crates.io/crates/gtk4)
- [gdk4](https://crates.io/crates/gdk4)
- [gtk4-macros](https://crates.io/crates/gtk4-macros)
- [LibAdwaita](https://crates.io/crates/libadwaita)

## Examples & Discussion
These are intended as pointers for others creating gtk-rs projects, as they took a while to wrap my head around (or i haven't yet).
### System Tray Library
(see [System Tray](docs/system_tray.md) for discussion)
### Subclassing
(see [Subclassing](docs/subclassing.md) for discussion)
### Async
(see [Async](docs/async.md) for discussion)
### Signals
(see [Signals](docs/signals.md) for discussion)

# License
The app is licensed under [GPL3.0-or-later](https://spdx.org/licenses/GPL-3.0-or-later.html)
