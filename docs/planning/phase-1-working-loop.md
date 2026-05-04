# Phase 1 — Working Loop

## Goal
Deliver a TUI that a developer can actually use for basic API work. The definition of done for Phase 1 is: a developer can open Torpor, send a GET or POST request, see the response, and save the request to a YAML file.

Everything else is deferred. No collections, no environments, no assertions. Just a tight request/response loop.

## Deliverables

- [ ] Basic TUI skeleton (Ratatui app with event loop)
- [ ] Request pane: method selector, URL bar, headers editor, body editor
- [ ] Send request via reqwest
- [ ] Response pane: status code, response time, headers, body
- [ ] JSON syntax highlighting in response body
- [ ] Save request to YAML file
- [ ] Load request from YAML file
- [ ] Basic keybinds (send, save, quit)

## Out of Scope for Phase 1

- Collections and workspaces
- Environment variables
- Auth beyond manual header entry
- Assertions
- History
- Sidebar/navigation

## Definition of Done

A developer can:
1. Launch Torpor
2. Enter a URL and select a method
3. Add headers and a JSON body
4. Send the request
5. See the response with syntax highlighting
6. Save the request to a `.yaml` file
7. Reopen Torpor and load the saved request
