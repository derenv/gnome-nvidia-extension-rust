<!--
SPDX-FileCopyrightText: 2024 Deren Vural
SPDX-License-Identifier: GPL-3.0-or-later
-->

### Tasks

#### 1 - Base functionality
- [x] Update crates in Cargo.toml
- [x] Refactor to use updated crates

#### 2 - Working

- [gtk inspector](https://developer.gnome.org/documentation/tools/inspector.html)
- [ ] flatpak deployment
    - [docs](https://developer.gnome.org/documentation/introduction/flatpak.html)
- [ ] Lots of seg faults
    - [ ] on exit
        - 
    - [ ] on 'refresh_cards'
        - caused by
            ```
            self.gpu_stack.get().remove(&valid_child);
            ```
- [ ] breaks on settings exit
    - runs 'refresh_cards'! this works here
    - errors:
        ```
        (main:3056): Adwaita-CRITICAL **: 15:07:59.348: adw_view_stack_add_titled: assertion 'gtk_widget_get_parent (child) == NULL' failed
        thread 'main' panicked at /home/deren/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libadwaita-0.6.0/src/auto/view_stack_page.rs:13:1: assertion failed: !ptr.is_null()
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        ```
- [ ] Nothing displayed
    - probably related to 'refresh_cards' not working

#### 3 - New

- [ ] Use nvidia-smi only, for wayland compat
- [ ] Refactor to simplify
- [ ] Add about popup
