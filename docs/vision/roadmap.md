# Roadmap

## Phase 1 — Working Loop (Current)
The goal of Phase 1 is to produce a TUI that a developer can actually use for day-to-day API work. Everything else is secondary to getting a tight request/response loop working.

**Deliverables:**
- Basic TUI layout with request builder and response viewer panes
- All HTTP methods
- Headers, query params, and JSON body editing
- Send request and display response (status, headers, body)
- Syntax highlighted JSON response
- Save and load requests as YAML files

**Definition of done:** A developer can replace Insomnia for basic GET/POST/PUT/PATCH/DELETE workflows.

---

## Phase 2 — Collections & Workspaces
Organise requests into collections and workspaces. Enable the git-friendly sharing workflow.

**Deliverables:**
- Workspace and collection file support
- Collection browser in the TUI sidebar
- Request ordering within collections
- Collection-level auth and header inheritance
- Import from Postman/Insomnia collection format

---

## Phase 3 — Environments & Variable Interpolation
Make it practical to work across multiple environments without editing request files.

**Deliverables:**
- Environment file support (dev, staging, prod)
- Variable interpolation in URLs, headers, and body (`{{variable}}`)
- Secret storage via system keyring
- Environment switcher keybind
- Visual indicator of active environment

---

## Phase 4 — Assertions & Test Runner
Make Torpor useful for API regression testing.

**Deliverables:**
- Assertion definitions in request YAML files
- Status code, response time, header, and JSONPath assertions
- Per-request test results in the TUI
- Collection runner with sequential execution and pass/fail summary

---

## Phase 5 — Request Chaining & Advanced Features
Enable complex multi-step workflows.

**Deliverables:**
- Extract block — pull values from responses into environment variables
- Pre/post request script hooks (environment manipulation only)
- Response history with diff view
- Code generation (curl, PHP Guzzle, Python requests, JS fetch)
- WebSocket support (stretch goal)
