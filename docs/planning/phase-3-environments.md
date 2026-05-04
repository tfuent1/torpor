# Phase 3 — Environments & Variable Interpolation

## Goal
Make it practical to work across multiple environments without editing request files.

## Deliverables

- [ ] Environment file support (load/save environments/*.yaml)
- [ ] Variable interpolation in URLs, headers, body, and auth values
- [ ] Secret storage via system keyring
- [ ] `torpor secret set <variable>` command to populate keyring
- [ ] Environment switcher keybind in TUI
- [ ] Visual indicator of active environment in status bar
- [ ] Warning when a variable reference cannot be resolved

## Definition of Done

A developer can switch between dev, staging, and prod environments with a single keybind and send the same request collection against each without modifying any request files.
