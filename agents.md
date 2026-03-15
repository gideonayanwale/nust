# NUST Browser – Agent Specification

This file defines the autonomous development agents responsible for building and maintaining the NUST browser engine.

NUST is a programmable browser platform written in Rust. The system is composed of multiple coordinated agents, each responsible for a subsystem.

Agents must follow modular architecture and communicate through defined interfaces.

---

## 1. Architect Agent

Responsibilities:

• maintain system-wide architecture
• enforce modular boundaries
• validate dependency graphs
• ensure scalability of subsystems

Capabilities:

* evaluate pull requests
* detect architectural violations
* propose system refactors

Scope:

entire repository

---

## 2. Core Engine Agent

Responsible for building the runtime core of the browser.

Subsystem:

core_engine/

Modules:

scheduler
task_queue
thread_pool
memory_manager
resource_manager
process_manager
ipc_system
event_loop

Responsibilities:

• event scheduling
• process orchestration
• task execution

---

## 3. Networking Agent

Subsystem:

networking_stack/

Modules:

dns_resolver
tcp_stack
tls_handler
http1_client
http2_client
http3_client
request_builder
response_parser
redirect_handler
connection_pool
cookie_store
cache_system

Responsibilities:

• network request handling
• connection management
• caching policies

---

## 4. Rendering Agent

Subsystem:

rendering_engine/

Modules:

html_tokenizer
html_parser
dom_tree_builder
css_tokenizer
css_parser
cssom
selector_matcher
style_engine
render_tree_builder

Responsibilities:

• parse HTML
• parse CSS
• build render tree

---

## 5. Layout Agent

Subsystem:

layout_engine/

Modules:

block_layout
inline_layout
flexbox_layout
grid_layout
box_model
layout_tree
layout_cache

Responsibilities:

• layout computation
• geometry calculation
• box model evaluation

---

## 6. Graphics Agent

Subsystem:

painting_engine/ and gpu_compositor/

Responsibilities:

• paint commands generation
• GPU compositing
• rasterization

Modules:

paint_tree
paint_commands
text_renderer
image_renderer
border_renderer
background_renderer

GPU modules:

layer_tree
compositor
rasterizer
texture_manager
shader_manager
animation_engine

---

## 7. JavaScript Runtime Agent

Subsystem:

javascript_runtime/

Modules:

script_loader
runtime_context
dom_bindings
event_bridge
timer_system
promise_scheduler

Responsibilities:

• JavaScript execution
• DOM manipulation bridge
• async task management

---

## 8. Automation Agent

Subsystem:

automation_platform/

Responsibilities:

• browser automation engine
• workflow execution
• task orchestration

Modules:

automation_runtime
script_executor
navigation_automation
dom_automation
task_scheduler
workflow_engine

---

## 9. AI Agent

Subsystem:

ai_platform/

Responsibilities:

• page summarization
• intelligent browsing
• research automation

Modules:

page_analyzer
summarizer
research_agent
form_autofill_agent
automation_agent

---

## 10. Security Agent

Subsystem:

security_layer/

Modules:

sandbox
permission_manager
same_origin_policy
cors_validator
content_security_policy

Responsibilities:

• security policies
• browser sandboxing
• permission enforcement

---

## 11. DevTools Agent

Subsystem:

devtools_platform/

Modules:

dom_inspector
network_monitor
console
performance_profiler
memory_profiler

Responsibilities:

• developer debugging
• runtime inspection

---

## 12. Observability Agent

Subsystem:

observability_system/

Modules:

logging_system
metrics_collector
crash_reporter
tracing_engine

Responsibilities:

• runtime monitoring
• telemetry
• diagnostics

---

## Agent Coordination Rules

Agents must follow these rules:

1. no cross-module imports outside defined interfaces
2. communicate through service APIs
3. maintain Rust workspace boundaries
4. enforce clean architecture

Architect Agent has final authority over system design.
