# NUST Architecture Expansion Blueprint

This document is a full architecture expansion for NUST, prepared as a combined Chromium/Gecko/Servo-style review board output.

It intentionally goes beyond current repository docs and defines the target system architecture for a serious, modern browser engine.

---

## 1. Expanded Repository Structure

```text
nust/
  Cargo.toml
  docs/
    architecture_expansion_blueprint.md
    engine_gap_discovery_report.md
  crates/

    # Platform Layer
    platform_windowing/
    platform_input/
    platform_clipboard/
    platform_file_picker/
    platform_media_devices/
    platform_accessibility/
    platform_integration/

    # Core Engine Layer (Parsing + Rendering)
    networking_stack/
    resource_loader/
    html_tokenizer/
    html_parser/
    dom_engine/
    css_tokenizer/
    css_parser/
    css_selector_engine/
    css_style_system/
    render_tree/
    layout_engine/
    paint_engine/
    display_list/
    gpu_compositor/

    # Execution Layer
    javascript_runtime/
    wasm_runtime/
    event_loop/
    task_scheduler/
    web_api_runtime/
    worker_runtime/
    service_worker_runtime/

    # Storage Layer
    cookie_manager/
    storage_local/
    storage_session/
    storage_indexeddb/
    storage_cache/
    quota_manager/
    history_database/
    bookmark_database/
    session_restore/

    # Security Layer
    sandbox_engine/
    site_isolation/
    permission_manager/
    tls_security/
    certificate_validation/
    csp_engine/
    safe_browsing/
    secure_storage/
    policy_enforcement/

    # Performance Layer
    memory_manager/
    gc_coordinator/
    tab_lifecycle/
    scheduler_frame/
    speculative_loading/
    performance_telemetry/

    # Developer Layer
    devtools_protocol/
    devtools_frontend/
    console_system/
    profiler_performance/
    profiler_memory/
    tracing_engine/

    # Extension Platform
    extension_runtime/
    extension_permissions/
    extension_background_workers/
    extension_sandbox/
    extension_store_client/

    # Browser UX Layer
    browser_shell/
    tab_manager/
    navigation_engine/
    address_bar/
    download_manager/
    settings_system/
    workspace_manager/

    # NUST Innovation Layer
    ai_assistant/
    semantic_history/
    privacy_guardian/
    tab_intelligence/
    workspace_knowledge_graph/
```

---

## 2. Full Crate Map (purpose, APIs, dependencies)

### Platform Layer

#### `platform_windowing`
- **Purpose**: OS windows/surfaces, swapchain handles, monitor DPI.
- **Core API**: `create_window()`, `surface_handle()`, `set_title()`, `set_fullscreen()`.
- **Depends on**: `platform_integration`.

#### `platform_input`
- **Purpose**: keyboard/mouse/touch/gamepad normalization.
- **Core API**: `next_input_event() -> InputEvent`.
- **Depends on**: `platform_windowing`.

#### `platform_clipboard`
- **Purpose**: read/write clipboard with permission policy hooks.
- **Core API**: `read_text()`, `write_text()`.
- **Depends on**: `permission_manager`.

#### `platform_file_picker`
- **Purpose**: native file picker dialogs and sandbox constraints.
- **Core API**: `open_dialog()`, `save_dialog()`.
- **Depends on**: `sandbox_engine`, `permission_manager`.

#### `platform_media_devices`
- **Purpose**: enumerate/request mic/camera devices.
- **Core API**: `enumerate_devices()`, `open_stream()`.
- **Depends on**: `permission_manager`, `platform_integration`.

#### `platform_accessibility`
- **Purpose**: map DOM/AX tree to native accessibility APIs.
- **Core API**: `publish_ax_tree()`, `notify_ax_event()`.
- **Depends on**: `dom_engine`, `platform_windowing`.

---

### Core Engine Layer

#### `networking_stack`
- **Purpose**: DNS + TCP + HTTP1/2/3 transport.
- **Core API**: `fetch(request) -> response_stream`.
- **Depends on**: `tls_security`.

#### `resource_loader`
- **Purpose**: orchestrate fetch priorities, redirects, content sniffing.
- **Core API**: `load_main_resource()`, `load_subresource()`.
- **Depends on**: `networking_stack`, `cookie_manager`, `storage_cache`, `policy_enforcement`.

#### `html_tokenizer`, `html_parser`
- **Purpose**: tokenize/parse HTML5-compliant documents.
- **Core API**: `tokenize(bytes)`, `parse(tokens)`.
- **Depends on**: `dom_engine`.

#### `dom_engine`
- **Purpose**: DOM tree, mutation hooks, node lifecycle.
- **Core API**: `create_document()`, `query_selector()`, `mutate()`.
- **Depends on**: `event_loop`.

#### `css_tokenizer`, `css_parser`, `css_selector_engine`, `css_style_system`
- **Purpose**: CSS parsing, selector matching, computed style generation.
- **Core API**: `parse_css()`, `match_rules()`, `compute_styles()`.
- **Depends on**: `dom_engine`.

#### `render_tree`, `layout_engine`, `paint_engine`, `display_list`, `gpu_compositor`
- **Purpose**: full rendering path to GPU output.
- **Core API**: `build_render_tree()`, `compute_layout()`, `emit_paint_ops()`, `compose_layers()`.
- **Depends on**: `dom_engine`, `css_style_system`, `scheduler_frame`, `platform_windowing`.

---

### Execution Layer

#### `javascript_runtime`
- **Purpose**: JS engine embed, script execution, realms.
- **Core API**: `evaluate(script)`, `call_function()`.
- **Depends on**: `web_api_runtime`, `event_loop`.

#### `wasm_runtime`
- **Purpose**: WebAssembly module loading/execution.
- **Core API**: `compile_module()`, `instantiate()`.
- **Depends on**: `javascript_runtime`.

#### `event_loop`
- **Purpose**: browser task model + microtasks + rendering ticks.
- **Core API**: `post_task(queue, task)`, `run_once()`.
- **Depends on**: `task_scheduler`.

#### `task_scheduler`
- **Purpose**: task priority lanes (input, script, layout, network callbacks).
- **Core API**: `schedule(priority, task)`.
- **Depends on**: `scheduler_frame`.

#### `web_api_runtime`
- **Purpose**: Fetch, Streams, URL, Timers, DOM events, etc.
- **Core API**: `register_api(namespace, bindings)`.
- **Depends on**: `dom_engine`, `resource_loader`, `storage_*`.

#### `worker_runtime`, `service_worker_runtime`
- **Purpose**: background execution and SW interception lifecycle.
- **Core API**: `spawn_worker()`, `register_service_worker()`, `intercept_fetch()`.
- **Depends on**: `javascript_runtime`, `storage_cache`, `resource_loader`.

---

### Storage Layer

#### `cookie_manager`
- **Purpose**: cookie parsing, persistence, policy enforcement.
- **Core API**: `set_cookie()`, `cookies_for_request()`.
- **Depends on**: `policy_enforcement`, `secure_storage`.

#### `storage_local`, `storage_session`, `storage_indexeddb`, `storage_cache`
- **Purpose**: Web storage APIs and offline cache.
- **Core API**: standard CRUD + origin scoping.
- **Depends on**: `quota_manager`, `secure_storage`.

#### `quota_manager`
- **Purpose**: per-origin quota accounting + eviction policy.
- **Core API**: `reserve(origin, bytes)`, `evict(origin)`.
- **Depends on**: storage crates.

#### `history_database`, `bookmark_database`, `session_restore`
- **Purpose**: browser profile data and crash/session resume.
- **Core API**: `record_visit()`, `save_session()`, `restore_session()`.
- **Depends on**: `secure_storage`.

---

### Security Layer

#### `sandbox_engine`
- **Purpose**: process sandbox profiles and broker IPC.
- **Core API**: `launch_sandboxed(process_type)`.
- **Depends on**: `platform_integration`.

#### `site_isolation`
- **Purpose**: origin/site process assignment and OOPIF policies.
- **Core API**: `assign_renderer(origin)`.
- **Depends on**: `navigation_engine`, `sandbox_engine`.

#### `tls_security`, `certificate_validation`
- **Purpose**: TLS transport + cert chain validation/revocation.
- **Core API**: `verify_certificate(chain, host)`.
- **Depends on**: `networking_stack` (integration).

#### `csp_engine`, `policy_enforcement`, `permission_manager`, `safe_browsing`, `secure_storage`
- **Purpose**: policy gates, permission prompts, malicious URL checks, encrypted profile data.
- **Core API**: `check_policy()`, `request_permission()`, `classify_url()`, `encrypt_at_rest()`.
- **Depends on**: multi-layer.

---

### Performance Layer

#### `memory_manager`
- **Purpose**: process/tab memory budgets.
- **Core API**: `set_tab_limit()`, `on_pressure_signal()`.
- **Depends on**: `tab_lifecycle`, `gc_coordinator`.

#### `gc_coordinator`
- **Purpose**: coordinate JS GC and DOM/resource release checkpoints.
- **Core API**: `request_gc(reason)`.
- **Depends on**: `javascript_runtime`, `dom_engine`.

#### `tab_lifecycle`
- **Purpose**: active/frozen/discarded/prerender states.
- **Core API**: `transition(tab, state)`.
- **Depends on**: `tab_manager`, `session_restore`.

#### `scheduler_frame`
- **Purpose**: frame production and vsync pacing.
- **Core API**: `request_animation_frame_tick()`.
- **Depends on**: `gpu_compositor`, `event_loop`.

#### `speculative_loading`
- **Purpose**: prefetch/prerender/predictive warmup.
- **Core API**: `schedule_prefetch(url)`.
- **Depends on**: `resource_loader`, `navigation_engine`.

---

### Developer Layer

#### `devtools_protocol`
- **Purpose**: remote debugging protocol endpoint (CDP-like).
- **Core API**: `handle_command(json)`.
- **Depends on**: engine/runtime crates.

#### `devtools_frontend`, `console_system`, `profiler_performance`, `profiler_memory`, `tracing_engine`
- **Purpose**: interactive debugging and telemetry.
- **Core API**: inspect DOM/network/perf/memory timelines.
- **Depends on**: `devtools_protocol`, `observability_system`.

---

### Extension Platform

#### `extension_runtime`
- **Purpose**: lifecycle of installed extensions.
- **Core API**: `install_extension()`, `activate_extension()`.
- **Depends on**: `extension_permissions`, `extension_sandbox`.

#### `extension_permissions`, `extension_background_workers`, `extension_sandbox`, `extension_store_client`
- **Purpose**: permission gate, bg execution, isolation, distribution.
- **Depends on**: `policy_enforcement`, `worker_runtime`, `safe_browsing`.

---

### Browser UX Layer

#### `browser_shell`, `tab_manager`, `navigation_engine`, `address_bar`, `download_manager`, `settings_system`, `workspace_manager`
- **Purpose**: user-visible browser app functionality.
- **Core API**: open tabs, navigate, download, configure browser.
- **Depends on**: almost all lower layers via interfaces.

---

### NUST Innovation Layer

#### `ai_assistant`
- **Purpose**: local/edge assistant for summarization and task help.
- **Core API**: `summarize_page()`, `propose_actions()`.
- **Depends on**: `dom_engine`, `semantic_history`, `privacy_guardian`.

#### `semantic_history`
- **Purpose**: embedding-based history retrieval.
- **Core API**: `search_semantic(query)`.
- **Depends on**: `history_database`, `ai_assistant`.

#### `privacy_guardian`
- **Purpose**: anti-tracking/fingerprinting protections.
- **Core API**: `apply_privacy_profile(origin)`.
- **Depends on**: `policy_enforcement`, `networking_stack`, `storage_*`.

#### `tab_intelligence`, `workspace_knowledge_graph`
- **Purpose**: suspend/prefetch orchestration and knowledge workspace browsing.
- **Depends on**: `tab_lifecycle`, `semantic_history`, `workspace_manager`.

---

## 3. Rendering Pipeline Architecture

### End-to-end pipeline and owning crates

1. **Network Request** → `networking_stack`
2. **Resource Loader orchestration** → `resource_loader`
3. **HTML Tokenization** → `html_tokenizer`
4. **DOM Construction** → `html_parser` + `dom_engine`
5. **CSS Parsing** → `css_tokenizer` + `css_parser`
6. **CSSOM Construction** → `css_style_system`
7. **Style Resolution** → `css_selector_engine` + `css_style_system`
8. **Render Tree Construction** → `render_tree`
9. **Layout Calculation** → `layout_engine`
10. **Paint Commands** → `paint_engine`
11. **Layer Compositing** → `display_list` + `gpu_compositor`
12. **GPU Rasterization** → `gpu_compositor`
13. **Screen Output** → `platform_windowing`

### Cross-cutting controllers
- frame pacing: `scheduler_frame`
- memory pressure: `memory_manager`
- security policy checks during loads: `policy_enforcement`

---

## 4. Process Model Diagram

```text
+------------------------- Browser Process -------------------------+
| browser_shell | tab_manager | navigation_engine | permissions UI  |
| download_manager | settings_system | extension_runtime host       |
+---------------------------+--------------------+------------------+
                            | IPC (typed channels)
                            v
+------------------------ Renderer Process(es) ---------------------+
| html/css parse | dom_engine | style | layout | paint | js/wasm     |
| workers (dedicated/shared) | accessibility tree producer          |
+---------------------------+--------------------+------------------+
                            | IPC
         +------------------+-------------------+
         v                                      v
+-------------------- GPU Process -------------------+   +--------- Network Process --------+
| display_list -> compositor -> rasterization        |   | networking_stack + tls + cache  |
| swapchain/surface binding                           |   | cookies + resource_loader hooks |
+----------------------------------------------------+   +----------------------------------+
                            ^
                            |
                     +------+------+
                     | Utility Proc |
                     | (media, file |
                     | picker, etc.)|
                     +-------------+
```

### IPC interfaces (must be explicit)
- `ipc/navigation`: begin/commit/fail navigation
- `ipc/render`: frame submission, input events, hit test
- `ipc/network`: request/response streaming, cache outcomes
- `ipc/security`: permission checks, policy verdicts, safe browsing verdicts
- `ipc/devtools`: tracing/profiler streams

---

## 5. Security Architecture

### 5.1 Mandatory controls
- process sandbox for renderer/GPU/network/utility
- site isolation (origin/site-based process boundaries)
- strict cross-origin policy matrix (SOP/CORS/CSP/COOP/COEP/CORP)
- robust certificate validation with revocation policy
- secure storage at rest for profile/credentials
- permission prompts tied to user gesture and origin trust
- safe browsing URL/download classification

### 5.2 Security data flow
1. navigation starts in browser process
2. `policy_enforcement` preflight checks origin and permissions
3. network process performs TLS + cert validation
4. safe browsing verdict influences allow/warn/block
5. renderer sandbox receives policy-limited capabilities
6. storage writes pass through origin partition + secure storage

### 5.3 Security hardening roadmap
- P0: sandbox + site isolation + cert validation + permission manager
- P1: safe browsing + download hardening + secure storage encryption
- P2: anti-fingerprinting/privacy guardian + policy auditing console

---

## 6. Performance Architecture

### 6.1 Optimization pillars
- **Parallelism**: network, parse, style, script, and raster on separate threads/processes.
- **Incrementality**: dirty-region layout/paint and retained display lists.
- **Predictive loading**: preconnect, prefetch, prerender under policy budget.
- **Memory discipline**: tab lifecycle state machine + pressure-driven discard.
- **GPU-first rendering**: compositing/raster in dedicated process with watchdog/recovery.

### 6.2 Core mechanisms
- frame scheduler aligned to vsync (`scheduler_frame`)
- lifecycle transitions: Active → Hidden → Frozen → Discarded (`tab_lifecycle`)
- JS/DOM coordinated collection (`gc_coordinator`)
- hot resource caches (code cache, image cache, style cache)
- startup fast-path with profile warming and lazy subsystem activation

---

## 7. Implementation Roadmap (phased)

### Phase 1 — Minimal browser engine
1. `resource_loader` foundation
2. `navigation_engine` state machine
3. parser/DOM/CSS/style/render/layout/paint integration correctness
4. `platform_windowing` output loop

### Phase 2 — Rendering correctness
1. CSS compatibility improvements
2. incremental layout invalidation
3. display list + compositor pipeline
4. image/font subsystems

### Phase 3 — JavaScript runtime
1. JS embedding hardening + event loop alignment
2. WebAssembly runtime integration
3. worker runtime (dedicated/shared)

### Phase 4 — Web standards support
1. service worker runtime
2. fetch/streams/web API runtime expansion
3. WPT + reftest harness integration

### Phase 5 — Performance systems
1. memory manager + tab lifecycle
2. speculative loading and prefetch policies
3. frame scheduler + GPU watchdog

### Phase 6 — Security architecture
1. site isolation
2. sandbox profiles
3. cert validation/revocation pipeline
4. safe browsing and download protection

### Phase 7 — Browser UX
1. robust tab/session/history/bookmark persistence
2. settings system and profile management
3. download UX and permission UX

### Phase 8 — NUST unique features
1. AI assistant + semantic history
2. privacy guardian and anti-fingerprinting
3. workspace intelligence and knowledge graph browsing

---

## Initialization Order (cold start)

1. `platform_integration`, `platform_windowing`, `platform_input`
2. `observability` + crash handler + tracing
3. `security_layer` baseline (policy/cert/permission services)
4. `networking_stack` + `resource_loader`
5. `browser_shell` + profile/session restore
6. spawn network/gpu/renderer process pools
7. initialize runtime stacks (DOM/CSS/JS/event loop)
8. open initial tab and begin navigation

