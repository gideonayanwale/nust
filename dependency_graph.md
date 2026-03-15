# NUST Browser – Dependency Graph

This document defines allowed dependencies between subsystems.

The purpose is to prevent architectural collapse as the codebase grows.

---

## Dependency Layers

NUST follows a layered architecture.

Layer 1 – Browser Shell
Layer 2 – Core Engine
Layer 3 – Network and Runtime Systems
Layer 4 – Rendering Systems
Layer 5 – GPU and UI Systems
Layer 6 – Platform Extensions

Dependencies must flow downward.

---

## Layer Breakdown

Layer 1:

browser_shell

Layer 2:

core_engine

Layer 3:

networking_stack
storage_system
security_layer
event_system
javascript_runtime

Layer 4:

dom_engine
rendering_engine
layout_engine
painting_engine

Layer 5:

gpu_compositor

Layer 6:

automation_platform
data_extraction_engine
ai_platform
distributed_cluster
extension_platform
devtools_platform
observability_system

---

## Allowed Dependency Flow

browser_shell
→ core_engine

core_engine
→ networking_stack
→ storage_system
→ event_system

rendering_engine
→ dom_engine

layout_engine
→ rendering_engine

painting_engine
→ layout_engine

gpu_compositor
→ painting_engine

automation_platform
→ dom_engine
→ browser_shell

ai_platform
→ data_extraction_engine
→ dom_engine

devtools_platform
→ rendering_engine
→ networking_stack

observability_system
→ all modules (read-only)

---

## Forbidden Dependencies

Rendering engine must not depend on:

automation_platform
ai_platform
distributed_cluster

Networking stack must not depend on:

rendering_engine
layout_engine

Automation platform must not manipulate GPU components directly.

---

## Dependency Validation

The architect agent must enforce dependency validation during development.

If a crate introduces a forbidden dependency:

the build must fail.

Automated checks must run in CI.
