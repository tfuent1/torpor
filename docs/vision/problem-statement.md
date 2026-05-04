# Problem Statement

## The Problem

Modern REST API clients like Insomnia and Postman are built on Electron — a framework that bundles a full Chromium browser instance to render a web application as a desktop app. This approach has a significant cost:

- **Memory usage**: Insomnia and Postman routinely consume 300–600MB of RAM at idle, doing nothing more than displaying text and waiting for user input.
- **Startup time**: Electron apps take several seconds to launch due to the overhead of initializing a browser runtime.
- **Complexity**: The tools are feature-bloated, with cloud sync, team collaboration, and subscription paywalls layered on top of what is fundamentally a tool for sending HTTP requests.

For developers who work primarily in the terminal, running a 500MB GUI application to test an API endpoint is an unreasonable trade-off. The underlying task — constructing an HTTP request and inspecting the response — is not inherently a graphical problem.

## Why Existing Alternatives Fall Short

TUI alternatives to Insomnia and Postman exist but are incomplete:

- **posting** is the closest equivalent, written in Python. Python's distribution story (pip, pipx, virtual environments) adds friction compared to a single compiled binary. Performance headroom is also limited.
- **hurl** is excellent for scripted/CI use cases but is not designed as an interactive client with persistent collections.
- **httpie** and **curlie** improve on curl's ergonomics but do not provide persistent request storage.
- **curl** itself requires reconstructing the full command on every invocation.

None of these tools provide the combination of persistent collections, environment variable management, request chaining, and an interactive TUI in a single compiled binary.

## The Opportunity

A Rust-based TUI API client can deliver:

- A single binary with no runtime dependencies
- Memory usage an order of magnitude lower than Electron-based alternatives
- Git-friendly YAML files that treat collections as code
- A keyboard-driven workflow that integrates naturally with a terminal-first development environment
- Feature parity with the core workflows developers actually use in Insomnia and Postman
