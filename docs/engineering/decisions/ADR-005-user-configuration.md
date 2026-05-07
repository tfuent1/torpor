# ADR-005 — User Configuration System (Keybinds and Themes)

**Status:** Accepted

## Context

As Torpor matures, two user-facing customization needs emerged before Phase 2:

1. **Keybind remapping** — power users want to change default key assignments,
   particularly those with vim-style workflows or terminal keybind conflicts.
2. **Themes** — developers work in various terminal colour schemes; the
   hardcoded colours in Phase 1 conflict with popular schemes like Solarized
   Dark, Nord, Catppuccin, and Dracula.

Decisions were needed on: config file format, config location, and how themes
are represented and extended.

## Options Considered

### File format: TOML vs YAML vs Lua

**YAML** is already used for workspace/collection/request files (ADR-001), so
consistency was an argument for it. However, user config has a fundamentally
different shape — flat key/value pairs: a theme name, a handful of string
arrays. YAML's indent sensitivity and verbose structure add no value here and
make parse errors harder to diagnose for users who just want to remap a key.

**Lua** would give programmable config (like Neovim's `init.lua`) — you could
compute keybinds conditionally, define themes programmatically, and so on.
However, this requires embedding a Lua runtime, which adds significant binary
weight and build complexity. Torpor's entire value proposition is being lean and
dependency-light. Scripting belongs in the pre/post-request hook system (Phase
5), not in a config file for key remapping.

**TOML** is the right fit for flat user config: it's what `Cargo.toml` uses, so
Rust developers are immediately comfortable with it; it has excellent error
messages for malformed input; it has native array syntax that maps cleanly to
multi-bind definitions like `quit = ["ctrl+q", "q"]`; and it's a pure
key/value format that doesn't scale awkwardly the way YAML does for this shape.

**Decision: TOML for user config.** The two formats serve different purposes and
coexist cleanly in the project.

### Config location: workspace-local vs XDG global vs both

Workspace-local config would mean each project carries its own keybind and
theme config. This creates duplication and means developers re-configure every
project. Preferences like keybinds and colour themes are personal, not
project-specific.

XDG global (`~/.config/torpor/`) means configure once, apply everywhere —
the correct model for personal preferences.

A "both with override" model adds flexibility but also complexity. The
per-workspace layer can be added in a future ADR if there is real demand.

**Decision: XDG standard location (`~/.config/torpor/config.toml`).**

### Theme representation

Three approaches were considered:

1. **Enum variants** — `theme = "nord"` maps to a compiled-in function.
   No user extensibility without recompiling.
2. **Named TOML files** — each theme is `~/.config/torpor/themes/<name>.toml`.
   Fully extensible; users can write and share themes.
3. **Inline in config.toml** — the full theme definition lives in the main
   config. Monolithic; editing one colour means touching the whole file.

The implemented approach combines 1 and 2: built-in themes are compiled in and
reachable by name; custom themes live in `~/.config/torpor/themes/<name>.toml`
and shadow built-in names if they share one. This gives zero-config defaults
while remaining fully extensible.

**Decision: named TOML files for custom themes, compiled-in built-ins as fallback.**

## Decision

- Config file: `~/.config/torpor/config.toml` (TOML)
- Keybinds: human-readable strings (`"ctrl+r"`, `"shift+tab"`) with multi-bind
  support (`Vec<KeyBind>` per action)
- Built-in themes: `default-dark`, `nord`, `catppuccin-mocha`, `dracula`,
  `solarized-dark`
- Custom themes: `~/.config/torpor/themes/<name>.toml` (full `Theme` struct)
- Config is written on first launch if absent; safe to re-generate
- Theme selector overlay accessible via `Ctrl+T` from any focus — `Up`/`Down`
  to navigate, `Enter` to apply and persist, `Esc` to cancel

## Consequences

- Two new dependencies: `toml = "0.8"` and `dirs = "5"`. Both MIT/Apache-2.0
  licensed, pure Rust, and small. `dirs` resolves `~/.config/` correctly across
  Linux, macOS, and Windows.
- Keybind strings are parsed at startup; malformed strings produce a clear error
  and fall back to the compiled-in default for that action.
- On systems where `dirs::config_dir()` returns `None` (unusual headless
  environments), Torpor uses compiled-in defaults — no panic.
- Custom theme TOML files can be shared via dotfiles repos. They are not
  workspace files and are not listed in `.gitignore`.
- Selecting a theme via `Ctrl+T` persists immediately to `config.toml` and
  swaps the live theme without restarting.
- The status bar hints are currently hardcoded strings. A future improvement
  could generate hints dynamically from the active `KeyBinds` so remapped
  bindings show correctly — deferred to avoid complexity before Phase 2.
