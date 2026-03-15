# NUST Browser – Repository Structure

This document defines the canonical repository layout for the NUST browser project.

The repository must be structured as a **Rust workspace** containing multiple independent crates representing subsystems.

---

## Root Workspace

nust/

Cargo.toml
README.md
LICENSE

docs/
scripts/
configs/

crates/

---

## Crates Directory

Each subsystem must exist as its own crate to enforce modular boundaries.

crates/

browser_shell/
core_engine/
networking_stack/
rendering_engine/
layout_engine/
painting_engine/
gpu_compositor/
javascript_runtime/
dom_engine/
event_system/
storage_system/
security_layer/
automation_platform/
data_extraction_engine/
ai_platform/
distributed_cluster/
extension_platform/
devtools_platform/
observability_system/

---

## Internal Structure of Each Crate

Each crate should follow the same internal pattern.

crate_name/

src/

lib.rs
mod.rs

modules/

component_one.rs
component_two.rs
component_three.rs

tests/

integration_tests.rs

---

## Example Crate Layout

rendering_engine/

src/

lib.rs

modules/

html_tokenizer.rs
html_parser.rs
dom_tree_builder.rs
css_tokenizer.rs
css_parser.rs
cssom.rs
selector_matcher.rs
style_engine.rs
render_tree_builder.rs

tests/

rendering_tests.rs

---

## Binary Entry Point

The main browser binary is located in:

crates/browser_shell/src/main.rs

Responsibilities:

initialize subsystems
start event loop
open browser window
initialize tab manager

---

## Documentation Directory

docs/

architecture/
design/
api/
development_guides/

---

## Scripts Directory

scripts/

build.sh
run_dev.sh
test_all.sh

These scripts simplify development workflows.

---

## Configuration Directory

configs/

build_config.toml
browser_settings.toml
feature_flags.toml

These configuration files control runtime behavior.

---

## Workspace Principles

The repository must enforce:

modular development
clear subsystem separation
independent crate compilation

Subsystem crates should interact only through public interfaces.

No circular dependencies are allowed.
