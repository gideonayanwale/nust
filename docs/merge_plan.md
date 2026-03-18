# NUST Merge Plan (Subsystem Consolidation)

This document begins the requested "merge them" effort by consolidating overlapping parser/render/paint APIs.

## Completed in this step

- `rendering_engine` now delegates tokenizer/parser/render-tree internals to dedicated crates:
  - `html_tokenizer`
  - `html_parser`
  - `render_tree`
- `painting_engine` now delegates paint command generation to `paint_engine`.
- `browser_shell` pipeline now consumes merged facades (`rendering_engine`, `painting_engine`) while internals stay modular.

## Why this matters

- keeps public subsystem boundaries stable while internal crates evolve
- avoids duplicate logic drift between old/new modules
- enables incremental migration without breaking integration

## Next merge tasks

1. Merge CSS path by routing future `css_parser/css_selector_engine/css_style_system` through `rendering_engine` facades.
2. Merge old/new paint command types into one canonical command model.
3. Merge networking + resource loading behind a single `networking_stack` public façade.
4. Deprecate duplicated modules after adapters are fully adopted.
