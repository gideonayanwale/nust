# NUST Browser – Steering Guidelines

This document defines how development decisions are made.

---

## Architectural Priorities

1. stability
2. security
3. performance
4. modularity
5. extensibility

---

## Code Quality Rules

All code must:

follow Rust best practices
include documentation
avoid unsafe blocks unless necessary
maintain clear module boundaries

---

## Dependency Rules

Subsystems must not create circular dependencies.

Allowed flow:

browser_shell
→ core_engine
→ networking/rendering
→ layout/painting
→ gpu_compositor

Automation and AI systems must interact through APIs.

---

## Performance Strategy

NUST must prioritize:

GPU acceleration
multi-threading
efficient memory usage

---

## Security Strategy

Implement:

sandboxing
process isolation
strict origin policies

Security must always take precedence over convenience.

---

## Feature Development Strategy

New features must:

not compromise rendering performance
not introduce unsafe global state
maintain compatibility with existing architecture
