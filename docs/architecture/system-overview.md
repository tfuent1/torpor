# System Overview

## High-Level Architecture

Torpor is a single-binary TUI application. There is no daemon, no background process, and no network service. Everything runs in the foreground as a single Tokio async runtime.

```
┌─────────────────────────────────────────────────┐
│                   Torpor Binary                  │
│                                                  │
│  ┌─────────────┐        ┌─────────────────────┐ │
│  │  TUI Layer  │◄──────►│   App State / Store  │ │
│  │  (Ratatui)  │        │                     │ │
│  └─────────────┘        └──────────┬──────────┘ │
│                                    │             │
│              ┌─────────────────────┼──────────┐  │
│              │                     │          │  │
│     ┌────────▼──────┐   ┌──────────▼───────┐  │  │
│     │ Storage Layer │   │  Request Engine  │  │  │
│     │  YAML + SQLite│   │    (reqwest)     │  │  │
│     └───────────────┘   └──────────────────┘  │  │
│                                                │  │
└────────────────────────────────────────────────┘  │
```

## Key Components

### TUI Layer
Built on Ratatui with crossterm as the backend. Responsible for rendering the interface and handling keyboard input. The TUI layer is purely a view — it reads from app state and dispatches events, but contains no business logic.

### App State
A central state struct that owns all runtime data — the active workspace, loaded collections, current request, last response, active environment, and UI focus state. All mutations go through the app state.

### Storage Layer
Handles reading and writing YAML files for workspaces, collections, environments, and requests. Also manages the SQLite database for request history via sqlx. Secrets are read and written via the system keyring.

### Request Engine
Wraps reqwest to execute HTTP requests. Handles environment variable interpolation before sending, and response parsing after receiving. Runs on the Tokio async runtime so the TUI remains responsive during in-flight requests.

## Data Flow

```
User Input
    │
    ▼
TUI Event Handler
    │
    ▼
App State Mutation
    │
    ├──► Storage Layer (save request, load collection)
    │
    ├──► Request Engine (send request)
    │         │
    │         ▼
    │    Response + History Write
    │
    ▼
TUI Re-render
```

## Technology Choices

| Concern | Choice | Rationale |
|---|---|---|
| TUI framework | Ratatui | Most mature Rust TUI library |
| Terminal backend | crossterm | Cross-platform, works on Linux/macOS |
| HTTP client | reqwest | Async, feature-complete, widely used |
| TLS | rustls | Pure Rust, no system OpenSSL dependency |
| Serialization | serde + serde_yaml | Idiomatic Rust, YAML support |
| Database | sqlx + SQLite | Async, compile-time checked queries, embedded |
| Secret storage | keyring | Cross-platform system keyring abstraction |
| Error handling | anyhow | Ergonomic error propagation at application layer |
| Async runtime | Tokio | Industry standard for Rust async |
