# Use Cases

## Primary Use Cases

### 1. Day-to-Day API Development
A developer building or consuming a REST API wants to send requests, inspect responses, and iterate quickly without leaving the terminal. Torpor provides a persistent collection of saved requests that can be opened, modified, and re-executed with minimal keystrokes.

### 2. Team-Shared Collections
A development team wants to share a set of API requests alongside their codebase. Because Torpor collections are plain YAML files, they can be committed to the same repository as the application code. Teammates clone the repo and immediately have access to the full request collection. Secrets are never committed — they are stored in each developer's system keyring.

### 3. Multi-Environment Workflows
A developer needs to test the same requests against local, staging, and production environments. Torpor environment files define named variable sets (base_url, auth tokens, etc.) that can be switched with a single keybind. Sensitive values like API keys are stored in the system keyring rather than in the environment file.

### 4. Request Chaining
A developer needs to authenticate before making subsequent requests. Torpor's extract block allows values from a response (such as a bearer token from a login endpoint) to be automatically written into the current environment, making them available to subsequent requests without manual copy-paste.

### 5. API Regression Testing
A developer wants to verify that an API behaves correctly after a change. Torpor's assertion system allows expected status codes, response times, headers, and JSON field values to be defined directly in the request file. The collection runner executes all requests in sequence and reports pass/fail per assertion.

## Secondary Use Cases

### Scripted/CI Use
Because Torpor collections are YAML files, a future CLI mode could execute collections non-interactively in a CI pipeline, producing structured output for test reporting.

### Onboarding
New team members can clone a repository and immediately have a working set of API requests with correct environment variable names. The only setup required is populating their local keyring with the appropriate secret values.
