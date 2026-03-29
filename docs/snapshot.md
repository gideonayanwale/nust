# NUST Build Snapshot

This snapshot summarizes what is currently implemented and runnable.

## What is already built

- Rust workspace with 19 subsystem crates organized by architecture layers.
- End-to-end minimal rendering pipeline:
  - fetch URL (placeholder HTTP1 client)
  - tokenize + parse HTML text
  - build DOM document
  - compute block layout
  - emit text paint commands
- Browser shell interfaces:
  - pinned/muted tabs state management
  - bookmark folders
  - searchable + recent history APIs
  - incognito session mode control
  - design system tokens (colors, spacing, typography, radius) for consistent shell skinning
  - Home surface (`--home`)
  - New Tab surface (`--new-tab <query>`)
  - multi-engine one-page search planning and rendering (Google, Bing, DuckDuckGo, Brave, Perplexity)
  - address input parser (URL vs search query)
  - route resolver (home/new-tab/external)
  - in-memory tab manager for new tab sessions

## Current limitations

- Networking client is still placeholder output, not full HTTP/TLS stack.
- UI is currently HTML string rendering via CLI output, not yet native window composited UI.
- Most non-shell subsystems are scaffolded and not fully implemented.

## Validation commands

- `cargo fmt --all`
- `cargo check`
- `cargo test`
- `cargo run -p browser_shell -- --home`
- `cargo run -p browser_shell -- --new-tab "multi engine search"`
- `cargo run -p browser_shell -- --showcase-modern-features`

## Visual references

![Architecture snapshot](images/nust-architecture-snapshot.svg)

![New tab UI mock](images/new-tab-ui-mock.svg)


## Skill server connection status

- Attempted to connect to curated skill source using the skill installer workflow.
- Environment currently blocks GitHub curated index access with HTTP 403 via proxy tunnel, so remote skill installation is pending.
