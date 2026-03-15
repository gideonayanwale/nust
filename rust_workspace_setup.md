# NUST Browser – Rust Workspace Setup

This document defines how the Rust workspace for NUST must be configured.

---

# Workspace Structure

Root directory:

nust/

Contains:

Cargo.toml
crates/

---

# Root Cargo.toml

Workspace configuration:

[workspace]

members = [
"crates/browser_shell",
"crates/core_engine",
"crates/networking_stack",
"crates/rendering_engine",
"crates/layout_engine",
"crates/painting_engine",
"crates/gpu_compositor",
"crates/dom_engine",
"crates/javascript_runtime",
"crates/event_system",
"crates/storage_system",
"crates/security_layer",
"crates/automation_platform",
"crates/data_extraction_engine",
"crates/ai_platform",
"crates/distributed_cluster",
"crates/extension_platform",
"crates/devtools_platform",
"crates/observability_system"
]

resolver = "2"

---

# Example Crate Configuration

Example crate:

rendering_engine

Cargo.toml:

[package]

name = "rendering_engine"
version = "0.1.0"
edition = "2021"

[dependencies]

dom_engine = { path = "../dom_engine" }

---

# Core Dependency Examples

layout_engine dependencies:

rendering_engine
dom_engine

painting_engine dependencies:

layout_engine

gpu_compositor dependencies:

painting_engine

automation_platform dependencies:

browser_shell
dom_engine

---

# Development Dependencies

Recommended workspace dependencies:

tokio
serde
serde_json
anyhow
thiserror
wgpu
winit

These provide:

async runtime
serialization
error handling
graphics interface

---

# Build Commands

Build workspace:

cargo build

Run browser:

cargo run -p browser_shell

Run tests:

cargo test

---

# Development Workflow

Standard workflow:

1. implement module
2. run cargo check
3. run cargo test
4. run browser locally

---

# Continuous Integration

CI must perform:

cargo check
cargo build
cargo test

for every commit.

---

# Workspace Rules

Crates must follow:

strict modular boundaries
no circular dependencies
clear public APIs

Subsystem crates must remain independently testable.

This ensures the NUST browser remains scalable as the codebase grows.
