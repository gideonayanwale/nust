# NUST Browser – System Modules

This file documents all modules used in the NUST architecture.

Total modules: ~120

Modules are grouped by subsystem.

---

# 1. Browser Shell

window_manager
tab_manager
navigation_controller
address_bar
session_manager
workspace_manager
bookmark_manager
history_manager
download_manager

---

# 2. Core Engine

scheduler
task_queue
thread_pool
memory_manager
resource_manager
process_manager
ipc_system
event_loop

---

# 3. Networking Stack

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

---

# 4. Rendering Engine

html_tokenizer
html_parser
dom_tree_builder
css_tokenizer
css_parser
cssom
selector_matcher
style_engine
render_tree_builder

---

# 5. Layout Engine

block_layout
inline_layout
flexbox_layout
grid_layout
box_model
layout_tree
layout_cache

---

# 6. Painting Engine

paint_tree
paint_commands
text_renderer
image_renderer
border_renderer
background_renderer

---

# 7. GPU Compositor

layer_tree
compositor
rasterizer
texture_manager
shader_manager
animation_engine

---

# 8. JavaScript Runtime

script_loader
runtime_context
dom_bindings
event_bridge
timer_system
promise_scheduler

---

# 9. DOM Engine

node
element
document
attribute
text_node
dom_query
mutation_observer

---

# 10. Event System

event_dispatcher
event_queue
input_router
keyboard_handler
pointer_handler
gesture_recognizer

---

# 11. Storage System

cookie_db
local_storage
session_storage
indexed_db
cache_storage

---

# 12. Security Layer

sandbox
permission_manager
same_origin_policy
cors_validator
content_security_policy

---

# 13. Automation Platform

automation_runtime
script_executor
navigation_automation
dom_automation
task_scheduler
workflow_engine

---

# 14. Data Extraction Engine

article_extractor
table_parser
product_parser
metadata_parser
structured_export

---

# 15. AI Platform

page_analyzer
summarizer
research_agent
form_autofill_agent
automation_agent

---

# 16. Distributed Cluster

worker_runtime
node_coordinator
task_distributor
result_aggregator

---

# 17. Extension Platform

extension_runtime
permission_model
api_gateway
extension_store

---

# 18. DevTools Platform

dom_inspector
network_monitor
console
performance_profiler
memory_profiler

---

# 19. Observability

logging_system
metrics_collector
crash_reporter
tracing_engine

---

Modules must be implemented as independent Rust crates where possible.
