# TUI Layout

## Overview

The Torpor TUI is divided into three main regions: a sidebar for navigation
(Phase 2), a main pane split between request and response, and a status bar
at the bottom.

## Current Layout (Phase 1)

```
┌────────────────────────────────────────────────────┐
│  [METHOD]  url                          Request    │
├────────────────────────────────────────────────────┤
│  Body | Headers                                    │
│                                                    │
│         Request Editor / Headers Table             │
│                                                    │
├────────────────────────────────────────────────────┤
│  200  233ms  514b                      Response    │
│                                                    │
│         Response Viewer (scrollable)               │
│                                                    │
├────────────────────────────────────────────────────┤
│  context-sensitive keybind hints                   │
└────────────────────────────────────────────────────┘
```

## Planned Layout (Phase 2+)

```
┌──────────────┬─────────────────────────────────────┐
│              │  Method  URL                        │
│  Sidebar     ├─────────────────────────────────────┤
│              │  Body | Headers                     │
│  Workspaces  ├─────────────────────────────────────┤
│  Collections │                                     │
│  Requests    │         Request Editor              │
│              │                                     │
│              ├─────────────────────────────────────┤
│              │  Status  Time  Size                 │
│              ├─────────────────────────────────────┤
│              │                                     │
│              │         Response Viewer             │
│              │                                     │
├──────────────┴─────────────────────────────────────┤
│  [ENV: dev]  workspace/collection/request   [?]    │
└────────────────────────────────────────────────────┘
```

## Regions

### URL Bar
- Displays and edits the request URL
- Method selector cycles with `Ctrl+↑` / `Ctrl+↓`
- Full cursor movement with `←` / `→`
- `Ctrl+D` clears the bar

### Request Pane
- Two tabs: **Body** and **Headers**, switched with `Ctrl+←` / `Ctrl+→` (or `Ctrl+H` / `Ctrl+L`)
- **Body tab**: full cursor-aware text editor — arrow keys move, `Enter` splits lines, `Backspace` merges
- **Headers tab**: key/value table — `↑↓` to select, `a` add, `d` delete, `Enter` edit, `Esc` cancel

### Response Pane
- Status code with color coding (2xx green, 3xx cyan, 4xx yellow, 5xx red)
- Response time and size on the status line
- JSON pretty-printed with syntax highlighting (keys cyan, strings green, numbers yellow, booleans magenta, null dim)
- Scrollable with `j`/`↓` and `k`/`↑`

### Status Bar
- Context-sensitive keybind hints that update based on the focused pane and active tab
- Status messages (send errors, save/load confirmations) shown in yellow

## Focus Model

Focus cycles through URL bar → request pane → response pane via `Tab` / `Shift+Tab`.
All navigation is keyboard-driven.

## Keybinds

| Key | Context | Action |
|-----|---------|--------|
| `Tab` / `Shift+Tab` | Anywhere | Cycle focus forward / backward |
| `Ctrl+R` | Anywhere | Send request |
| `Ctrl+S` | Anywhere | Save request to `request.yaml` |
| `Ctrl+O` | Anywhere | Load request from `request.yaml` |
| `Ctrl+Q` | Anywhere | Quit |
| `q` | Response pane | Quit |
| `Ctrl+↑` / `Ctrl+↓` | URL bar | Cycle HTTP method |
| `←` / `→` | URL bar | Move cursor |
| `Ctrl+D` | URL bar | Clear URL |
| `Ctrl+←` / `Ctrl+→` | Request pane | Switch Body / Headers tab |
| `Ctrl+H` / `Ctrl+L` | Request pane | Switch Body / Headers tab (vim aliases) |
| `↑↓←→` | Body tab | Move cursor |
| `Enter` | Body tab | Insert newline at cursor |
| `Backspace` | Body tab | Delete character / merge with previous line |
| `↑` / `↓` | Headers tab | Select row |
| `a` | Headers tab | Add new header |
| `d` | Headers tab | Delete selected header |
| `Enter` | Headers tab | Edit selected header (key then value) |
| `Esc` | Headers tab (editing) | Cancel edit |
| `j` / `↓` | Response pane | Scroll down |
| `k` / `↑` | Response pane | Scroll up |
