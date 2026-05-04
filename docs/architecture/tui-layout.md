# TUI Layout

## Overview

The Torpor TUI is divided into three main regions: a sidebar for navigation, a main pane split between request and response, and a status bar at the bottom.

## Layout

```
┌──────────────┬─────────────────────────────────────┐
│              │  Method  URL                [Send]  │
│  Sidebar     ├─────────────────────────────────────┤
│              │  Headers │ Body │ Auth │ Params     │
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

### Sidebar
- Displays the workspace tree: collections and their requests
- Fuzzy search to filter requests
- Keybinds to create, rename, and delete requests and collections
- Collapsible collection groups

### Request Pane
- Method selector (GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS)
- URL bar with environment variable interpolation preview
- Tab bar: Headers | Body | Auth | Params
- Each tab shows a key/value editor or body text area as appropriate
- Send button / keybind

### Response Pane
- Status code with color coding (2xx green, 4xx yellow, 5xx red)
- Response time and size
- Tab bar: Body | Headers
- Syntax highlighted body (JSON, XML, HTML, plain text)
- Pretty print / raw toggle
- Scrollable with search

### Status Bar
- Active environment name with color indicator
- Current workspace / collection / request breadcrumb
- Keybind hint

## Focus Model

Focus cycles through the sidebar, request pane, and response pane. Within each pane, tab navigation moves between elements. All navigation is keyboard driven. Mouse is supported but optional.

## Keybinds (Planned)

| Key | Action |
|-----|--------|
| `Tab` | Cycle focus between panes |
| `Enter` | Send request (when URL bar focused) |
| `Ctrl+Enter` | Send request (from anywhere) |
| `Ctrl+e` | Switch environment |
| `Ctrl+n` | New request |
| `Ctrl+s` | Save request |
| `/` | Search sidebar |
| `q` | Quit |
| `?` | Help |
| `j/k` | Navigate lists (vim style) |
| `gg/G` | Jump to top/bottom of list |
