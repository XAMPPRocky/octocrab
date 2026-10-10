# Octocrab Playground

A small interactive, schema-driven, hierarchical GitHub API query explorer built with [Dioxus 0.7](https://dioxuslabs.com) and [Octocrab](https://github.com/XAMPPRocky/octocrab).

## Features

- **Schema-Driven Dynamic Forms**: GitHub entities define a declarative `FieldSchema` which renders typed inputs (text, numbers, booleans, enums, secrets) with descriptions and validations.
- **Hierarchical Query Merging**: Construct trees of entities (e.g. `Repository -> Issues & Pull Requests`, `Organization -> Repos & Members`, `User -> Repos & Followers`). Child queries should inherit context (such as repository `owner` and `repo` names) and merge into an enveloped JSON result:
  ```json
  {
    "_meta": { "entity": "repo", "status": "success", "duration_ms": 115 },
    "data": { ... },
    "children": {
      "repo_issues": {
        "_meta": { "entity": "repo_issues", "count": 5, "status": "success" },
        "data": [ ... ]
      },
      "repo_pulls": {
        "_meta": { "entity": "repo_pulls", "count": 5, "status": "success" },
        "data": [ ... ]
      }
    }
  }
  ```
- **Cross-Platform**:
  - **Native Desktop**: Native desktop GUI powered by `dioxus-desktop` / Wry.
  - **WebAssembly / Browser**: Client-side web SPA compiled to `wasm32-unknown-unknown` executing requests with browser fetch directly against GitHub API with personal access tokens or public access.
- **Presets**: example query trees:
  - Repository + Issues + Pull Requests
  - Organization + Repositories + Members
  - User Profile + Repositories + Followers
  - Search Repositories

## Dioxus Components Management

You can list or add any additional components directly using the `dx` CLI:
```bash
cd playground
# List available components from the registry
dx components list

# Add more components (e.g. tooltip, select, dialog)
dx components add select tooltip dialog
```

## How to Run

### 1. Run as Native Desktop App
```bash
cargo run -p octocrab-playground
```

### 2. Run in Web Browser
Using the Dioxus CLI:
```bash
cd playground
dx serve --web
```
Or build the WASM bundle:
```bash
cargo build -p octocrab-playground --target wasm32-unknown-unknown --no-default-features --features web
```

### 3. Run Tests
```bash
cargo test -p octocrab-playground
```
