# Request Execution

## Overview

When a user sends a request, Torpor performs a pipeline of steps before the HTTP request goes out and after the response comes back.

## Execution Pipeline

```
User triggers send
        │
        ▼
1. Load active environment variables
        │
        ▼
2. Resolve collection-level auth and headers
        │
        ▼
3. Merge request-level auth and headers (request wins on conflict)
        │
        ▼
4. Interpolate environment variables in URL, headers, body, auth
        │
        ▼
5. Execute pre_request script (if defined)
        │
        ▼
6. Send HTTP request via reqwest
        │
        ▼
7. Receive response
        │
        ▼
8. Run assertions (if defined)
        │
        ▼
9. Run extract block — write values into active environment
        │
        ▼
10. Execute post_request script (if defined)
        │
        ▼
11. Write to history (SQLite)
        │
        ▼
12. Update TUI with response
```

## Variable Interpolation

All `{{variable}}` references are resolved against the active environment's variable map before the request is sent. Interpolation applies to:

- The URL
- All header values
- The request body
- Auth values (token, username, password, api key)
- Query parameter values

Variables that cannot be resolved (not present in the active environment) are left as-is and a warning is surfaced in the TUI status bar.

## Auth Resolution

Auth is resolved in the following priority order:

1. Request-level auth (highest priority)
2. Collection-level auth
3. No auth

If request-level auth is set to `none` explicitly, it overrides and suppresses collection-level auth. If request-level auth is not defined, collection-level auth is used.

## Async Execution

The HTTP request is executed on the Tokio async runtime. The TUI remains responsive during in-flight requests — a loading indicator is shown and the user can cancel the request with `Escape`.

## Error Handling

Network errors, TLS errors, and timeout errors are caught and displayed in the response pane rather than crashing the application. The history entry is still written with an error status.

## Assertions

Assertions are evaluated after the response is received. Each assertion produces a pass or fail result. Results are displayed in the response pane. A failed assertion does not stop subsequent requests in a collection run — all requests execute and a summary is shown at the end.

## Extract

The extract block runs after assertions. Each extract entry reads a value from the response (via JSONPath or header name) and writes it into the active environment's in-memory variable map. This updated map is used for subsequent requests in the same session without modifying the environment YAML file on disk.
