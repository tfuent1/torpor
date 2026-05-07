# torpor

A lightweight TUI-based REST API client for the terminal, written in Rust.

Torpor is a keyboard-driven alternative to Insomnia and Postman. No Electron,
no cloud sync, no subscription. Just a single binary that sends HTTP requests
and gets out of your way.

## Why

Insomnia and Postman are built on Electron and consume 300–600MB of RAM at
idle. For developers who live in the terminal, that's an unreasonable trade-off
for a tool that fundamentally just sends HTTP requests.

Torpor aims to deliver the core workflows developers actually use — persistent
collections, environment variable management, request chaining, and assertions
— in a single compiled binary with a fraction of the memory footprint.

## Status

Phase 1 (working request/response loop) is complete. Torpor can be used today
for basic GET/POST/PUT/PATCH/DELETE workflows. Phase 2 (collections and
workspaces) is next.

See the [roadmap](docs/vision/roadmap.md) for what's planned.

## Features

- Send HTTP requests from a keyboard-driven TUI
- Full cursor-aware JSON body editor
- Headers editor with inline key/value editing
- JSON syntax highlighting in the response pane with scrolling
- Save and load requests as plain YAML files — git-friendly by design
- Single binary, no runtime dependencies

## Planned

- Organise requests into collections and workspaces
- Multiple environments (dev, staging, prod) with a single keybind to switch
- Secrets stored in the system keyring, never committed to version control
- Assert on status codes, response times, headers, and JSON fields
- Chain requests — extract values from responses and use them in subsequent requests

## Installation

Not yet published. Build from source:

```bash
git clone git@github.com:tfuent1/torpor.git
cd torpor
cargo build --release
./target/release/torpor
```

## Keybinds

| Key | Context | Action |
|-----|---------|--------|
| `Tab` / `Shift+Tab` | Anywhere | Cycle focus forward / backward |
| `Ctrl+R` | Anywhere | Send request |
| `Ctrl+S` | Anywhere | Save request to `request.yaml` |
| `Ctrl+O` | Anywhere | Load request from `request.yaml` |
| `Ctrl+Q` | Anywhere | Quit |
| `Ctrl+↑` / `Ctrl+↓` | URL bar focused | Cycle HTTP method |
| `←` / `→` | URL bar focused | Move cursor |
| `Ctrl+D` | URL bar focused | Clear URL |
| `Ctrl+←` / `Ctrl+→` | Request pane focused | Switch Body / Headers tab |
| `Ctrl+H` / `Ctrl+L` | Request pane focused | Switch Body / Headers tab (vim aliases) |
| `↑` / `↓` / `←` / `→` | Body tab | Move cursor |
| `Enter` | Body tab | Insert newline |
| `Backspace` | Body tab | Delete character / merge lines |
| `↑` / `↓` | Headers tab | Select row |
| `a` | Headers tab | Add new header row |
| `d` | Headers tab | Delete selected row |
| `Enter` | Headers tab | Edit selected row |
| `Esc` | Headers tab (editing) | Cancel edit |
| `j` / `↓` | Response pane focused | Scroll down |
| `k` / `↑` | Response pane focused | Scroll up |
| `q` | Response pane focused | Quit |

## Documentation

Full documentation lives in [`docs/`](docs/README.md).

- [Problem Statement](docs/vision/problem-statement.md)
- [Roadmap](docs/vision/roadmap.md)
- [Architecture Overview](docs/architecture/system-overview.md)
- [Contributing](docs/engineering/contributing.md)

## Contributing

See [docs/engineering/contributing.md](docs/engineering/contributing.md) for
setup instructions, code standards, and how to open a pull request.

## License

Licensed under either of MIT or Apache-2.0 at your option.

## AI Assistance

This project is developed with the assistance of AI tools, primarily
[Claude](https://claude.ai) by Anthropic. AI is used for architecture
discussion, code review, documentation, and pair programming.

Claude has no write access to this repository. All code and content is
reviewed and committed by the human author. AI-generated suggestions may
be used as-is, modified, or rejected — but nothing reaches the repository
without human review and approval.
