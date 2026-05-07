# Phase 1 — Working Loop

## Goal
Deliver a TUI that a developer can actually use for basic API work. The definition of done for Phase 1 is: a developer can open Torpor, send a GET or POST request, see the response, and save the request to a YAML file.

Everything else is deferred. No collections, no environments, no assertions. Just a tight request/response loop.

## Deliverables

- [x] Basic TUI skeleton (Ratatui app with event loop)
- [x] Request pane: method selector, URL bar, headers editor, body editor
- [x] Send request via reqwest
- [x] Response pane: status code, response time, headers, body
- [x] JSON syntax highlighting in response body
- [x] Response pane scrolling
- [x] Save request to YAML file
- [x] Load request from YAML file
- [x] Basic keybinds (send, save, quit)

## Definition of Done

A developer can:
1. Launch Torpor ✓
2. Enter a URL and select a method ✓
3. Add headers and a JSON body ✓
4. Send the request ✓
5. See the response with syntax highlighting ✓
6. Save the request to a `.yaml` file ✓
7. Reopen Torpor and load the saved request ✓

## Status: Complete

Phase 1 is done. See the [roadmap](../vision/roadmap.md) for what's next.

## Keybinds Implemented

| Key | Context | Action |
|-----|---------|--------|
| `Tab` / `Shift+Tab` | Anywhere | Cycle focus |
| `Ctrl+R` | Anywhere | Send request |
| `Ctrl+S` | Anywhere | Save to `request.yaml` |
| `Ctrl+O` | Anywhere | Load from `request.yaml` |
| `Ctrl+Q` | Anywhere | Quit |
| `Ctrl+↑` / `Ctrl+↓` | URL bar | Cycle HTTP method |
| `←` / `→` | URL bar | Move cursor |
| `Ctrl+D` | URL bar | Clear URL |
| `Ctrl+←` / `Ctrl+→` | Request pane | Switch Body / Headers tab |
| `Ctrl+H` / `Ctrl+L` | Request pane | Switch tabs (vim aliases) |
| `↑↓←→` | Body tab | Move cursor |
| `Enter` | Body tab | New line |
| `Backspace` | Body tab | Delete / merge lines |
| `↑` / `↓` | Headers tab | Select row |
| `a` / `d` | Headers tab | Add / delete header |
| `Enter` / `Esc` | Headers tab | Edit / cancel |
| `j` / `↓` | Response pane | Scroll down |
| `k` / `↑` | Response pane | Scroll up |
| `q` | Response pane | Quit |
