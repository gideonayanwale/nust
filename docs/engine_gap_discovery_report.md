# NUST Engine Gap Discovery Report

This report performs deep architecture gap discovery against the current NUST documentation and scaffolding, with the goal of evolving NUST into a Chromium-class, multi-platform browser platform.

---

## 1) Missing Subsystems Report

### 1.1 Document audit summary

From the current docs, NUST already declares broad subsystem categories (networking, rendering, layout, painting, GPU, JS runtime, automation, AI, distributed, extensions, devtools, observability) and phased goals. However, many production-critical browser internals are still implicit, underspecified, or absent.

### 1.2 Undocumented-but-required subsystems

#### Engine architecture gaps
- **Frame tree + browsing context group model** (main frame + iframes + OOPIF style support)
- **Navigation stack state machine** (provisional loads, same-document nav, history traversal semantics)
- **Document lifecycle system** (active, bfcache candidate, discarded, prerendered)
- **Back/Forward Cache (BFCache)** controller
- **Process assignment policy** (site instance, process-per-site/per-origin/per-tab modes)
- **Frame scheduler integration** (main thread/task priority policy)

#### Web platform/runtime gaps
- **WebIDL bindings generator** and typed binding layer
- **Full event loop model** (task queues: DOM manipulation, networking, timers, microtasks, rendering)
- **Web API surface crates** (fetch, streams, URL, encoding, storage events, workers, messaging)
- **Worker runtime family** (DedicatedWorker, SharedWorker, ServiceWorker)
- **Service Worker lifecycle** and fetch interception pipeline
- **Permissions policy parser/enforcer**

#### Security model gaps
- **Site isolation enforcement service**
- **Certificate validation + CT/OCSP/CRL policy engine**
- **Safe browsing / malicious URL reputation service integration**
- **Download quarantine + executable hardening policy**
- **Credential isolation & keychain integration**
- **Cross-origin embedder/opener/resource policy stack (COEP/COOP/CORP)**

#### Performance/graphics gaps
- **Display list + retained rendering pipeline**
- **Compositor frame scheduling + vsync alignment**
- **GPU process watchdog + context loss recovery**
- **Font subsystem** (font fallback, shaping stack, glyph cache)
- **Image decode pipeline** (progressive decode, animated formats, color management)
- **Incremental + dirty-region layout/paint invalidation**

#### Storage/data gaps
- **Unified quota manager** (per-origin quotas + eviction policy)
- **Storage partitioning system** (first-party sets / anti-tracking partitions)
- **Persistent profile format + migration manager**
- **Session restore snapshots + crash restart journal**
- **Credential/password manager**

#### Platform integration gaps
- **Native accessibility tree bridge** (AX API integration)
- **IME/text input composition support**
- **Clipboard read/write policy layer**
- **File picker + downloads shell integration**
- **Media playback stack** (MSE/EME path planning, audio focus)
- **Platform abstraction layer** (Win/macOS/Linux differences)

#### Testing/compliance gaps
- **WPT harness integration (web-platform-tests)**
- **Layout/reftest screenshot diff harness**
- **Fuzzing infra (HTML/CSS/JS parser fuzzers)**
- **Deterministic virtual time/test scheduler**
- **Browser-level integration test harness (multi-process)**

---

## 2) Engine Architecture Expansion

### 2.1 Required engine domains beyond current docs

1. **Navigation & frame lifecycle domain**
   - `navigation_engine/`
     - `navigation_state_machine`
     - `frame_tree`
     - `browsing_context_group`
     - `bfcache_manager`

2. **Web runtime domain**
   - `web_runtime/`
     - `webidl_bindings`
     - `web_api_registry`
     - `worker_runtime`
     - `service_worker_engine`

3. **Process & isolation domain**
   - `process_isolation/`
     - `site_instance_manager`
     - `process_assignment_policy`
     - `origin_agent_cluster`

4. **Rendering robustness domain**
   - `display_pipeline/`
     - `display_list`
     - `invalidation_tracker`
     - `frame_scheduler`
     - `bfcache_paint_restore`

5. **Compatibility/compliance domain**
   - `standards_compliance/`
     - `wpt_runner`
     - `compat_flags`
     - `quirks_mode`

### 2.2 Comparison-to-modern-engines findings

| Capability Class | Chromium/Gecko/WebKit Reality | NUST gap |
|---|---|---|
| Multi-process frame graph | Site isolation + frame-level process selection | Lacks site-instance/frame-process policies |
| Service worker networking | Request interception, offline cache, update lifecycle | No SW engine/interceptor lifecycle |
| Web API completeness | Huge WebIDL-generated API surfaces | No bindings generation pipeline |
| Rendering retention | Display lists + retained layers + invalidation | Only high-level pipeline modules |
| Standards validation | WPT + reftests + interop tracking | No formal compliance harness |
| Security hardening | COOP/COEP/CORP, cert validation, sandbox layers | Only core security headings documented |

---

## 3) Infrastructure Requirements

### 3.1 Development infrastructure
- CI matrix for Linux/macOS/Windows with tiered suites:
  - `cargo check`, unit tests, integration tests, reftests, WPT subset, fuzz-smoke
- Release branch protections with compatibility gates
- Benchmark lab harness (startup, input latency, page load, memory)
- Symbol server + crash dump processing pipeline

### 3.2 Distribution infrastructure
- Installer/packaging pipelines:
  - MSI (Windows), PKG/DMG (macOS), DEB/RPM/AppImage (Linux)
- Differential update channel service (stable/beta/nightly)
- Signed updates and rollback support
- Policy templates for enterprise deployment

### 3.3 Developer ecosystem infrastructure
- DevTools protocol server (CDP-compatible superset)
- Remote debugging endpoint auth model
- Extension packaging and signing service
- Extension review/scanning pipeline

---

## 4) Security Architecture Plan

### 4.1 Core plan
1. **Process isolation first**
   - Renderer sandbox + site isolation baseline before broad Web API rollout.
2. **Policy engine unification**
   - One policy coordinator for CSP, CORP, COEP, COOP, Permissions-Policy.
3. **Network trust chain**
   - Strict certificate validation + revocation + pinning exceptions store.
4. **Data isolation**
   - Storage partitioning and profile encryption boundaries.
5. **Threat telemetry**
   - Safe browsing lookups + suspicious behavior detectors.

### 4.2 Security modules to add
- `security_layer/site_isolation`
- `security_layer/certificate_validator`
- `security_layer/revocation_checker`
- `security_layer/coop_coep_corp`
- `security_layer/permissions_policy`
- `security_layer/download_protection`
- `security_layer/profile_encryption`

---

## 5) Performance Architecture Plan

### 5.1 Core plan
1. **GPU-first frame production** with robust software fallback.
2. **Incremental rendering** (dirty-region layout/paint invalidation).
3. **Scheduler sophistication** (input/task/frame priority tiers).
4. **Memory pressure automation** (tab freeze/discard + restoration).
5. **Cache intelligence** (code cache, image cache, bfcache, predictive preload).

### 5.2 Performance modules to add
- `core_engine/frame_scheduler`
- `layout_engine/invalidation_engine`
- `painting_engine/retained_display_list`
- `gpu_compositor/vsync_scheduler`
- `core_engine/memory_pressure_manager`
- `networking_stack/predictive_prefetch`
- `storage_system/page_snapshot_cache`

---

## 6) Future Feature Proposals (NUST-forward)

### 6.1 AI-native browser features
- **On-device assistant runtime** with per-origin safety boundaries.
- **Semantic history search** over title/text embeddings.
- **Agentic workflow synthesis** from repeated browsing patterns.
- **Explainable security assistant**: human-readable threat reasoning.

### 6.2 Privacy-first innovations
- Dynamic anti-fingerprinting profiles by site risk score.
- Per-workspace identity containers (cookies/storage/process partition bundles).
- Tracker graph visualizer in DevTools.

### 6.3 UX innovations
- Vertical tabs + workspace graph view.
- Time-travel browsing timeline (history as DAG + snapshots).
- Spatial “research canvas” linking tabs, notes, extracted entities, agents.

### 6.4 Systems innovations
- Distributed crawling/automation jobs with deterministic replay.
- Hybrid local+cluster rendering diagnostics for large page workloads.

---

## 7) Implementation Task Graph

### Phase 1 — Core browser engine
| Subsystem | Purpose | Dependencies | Suggested crate/module | Priority | Complexity |
|---|---|---|---|---|---|
| Navigation State Machine | Correct load/commit/error transitions | browser_shell, networking_stack, dom_engine | `navigation_engine/navigation_state_machine` | P0 | High |
| Frame Tree Model | Main frame + iframe orchestration | dom_engine, rendering_engine | `navigation_engine/frame_tree` | P0 | High |
| Document Lifecycle | Active/prerender/bfcache states | navigation_engine, core_engine | `navigation_engine/document_lifecycle` | P0 | High |
| Resource Loader | Unified network fetch orchestration | networking_stack, security_layer | `networking_stack/resource_loader` | P0 | Medium |

### Phase 2 — Web standards compliance
| Subsystem | Purpose | Dependencies | Suggested crate/module | Priority | Complexity |
|---|---|---|---|---|---|
| WebIDL Bindings Generator | Typed Web API glue | javascript_runtime, dom_engine | `web_runtime/webidl_bindings` | P0 | Very High |
| Worker Runtime | Multi-threaded JS workers | javascript_runtime, core_engine | `web_runtime/worker_runtime` | P0 | High |
| Service Worker Engine | Offline/interception lifecycle | networking_stack, storage_system | `web_runtime/service_worker_engine` | P0 | High |
| WPT Harness | Standards verification | all subsystems | `standards_compliance/wpt_runner` | P0 | High |

### Phase 3 — Security architecture
| Subsystem | Purpose | Dependencies | Suggested crate/module | Priority | Complexity |
|---|---|---|---|---|---|
| Site Isolation Manager | Process boundary by site/origin | core_engine, navigation_engine | `security_layer/site_isolation` | P0 | High |
| Certificate Validator | TLS trust decisions | networking_stack | `security_layer/certificate_validator` | P0 | Medium |
| Policy Coordinator | CSP/COOP/COEP/CORP/Permissions-Policy | dom_engine, networking_stack | `security_layer/policy_coordinator` | P0 | High |
| Download Protection | Malware/executable control | browser_shell, networking_stack | `security_layer/download_protection` | P1 | Medium |

### Phase 4 — Performance optimization
| Subsystem | Purpose | Dependencies | Suggested crate/module | Priority | Complexity |
|---|---|---|---|---|---|
| Frame Scheduler | Input/render/task prioritization | core_engine, gpu_compositor | `core_engine/frame_scheduler` | P0 | High |
| Invalidation Engine | Incremental layout/paint | layout_engine, painting_engine | `layout_engine/invalidation_engine` | P0 | High |
| Memory Pressure Manager | Freeze/discard tabs | browser_shell, storage_system | `core_engine/memory_pressure_manager` | P1 | Medium |
| Predictive Prefetch | Performance uplift for likely nav | networking_stack | `networking_stack/predictive_prefetch` | P1 | Medium |

### Phase 5 — Ecosystem and tooling
| Subsystem | Purpose | Dependencies | Suggested crate/module | Priority | Complexity |
|---|---|---|---|---|---|
| DevTools Protocol Server | Rich remote debugging | devtools_platform, observability_system | `devtools_platform/protocol_server` | P0 | High |
| Extension Signing Pipeline | Secure extension ecosystem | extension_system, security_layer | `extension_system/signing` | P1 | Medium |
| Crash Symbolication Service | Production triage | observability_system | `observability_system/symbolication` | P1 | Medium |
| Auto-update Service | Safe incremental updates | browser_shell, security_layer | `distribution/update_service` | P1 | High |

### Phase 6 — NUST unique innovations
| Subsystem | Purpose | Dependencies | Suggested crate/module | Priority | Complexity |
|---|---|---|---|---|---|
| Semantic History Search | AI-powered retrieval | ai_platform, history_manager | `ai_platform/semantic_history` | P1 | Medium |
| Workspace Containers | Privacy-isolated workspaces | storage_system, security_layer | `browser_shell/workspace_containers` | P1 | High |
| Agentic Workflow Studio | Visual automation+AI graph | automation_platform, ai_platform | `automation_platform/workflow_studio` | P2 | High |
| Timeline Browsing DAG | Time-travel browsing state model | history_manager, storage_system | `browser_shell/timeline_history` | P2 | Medium |

---

## 8) Updated Repository Structure (Proposed Expansion)

```text
nust/
  crates/
    browser_shell/
      src/
        workspace_containers.rs
        timeline_history.rs
    navigation_engine/
      src/
        navigation_state_machine.rs
        frame_tree.rs
        document_lifecycle.rs
        bfcache_manager.rs
    web_runtime/
      src/
        webidl_bindings/
        worker_runtime.rs
        service_worker_engine.rs
        web_api_registry.rs
    process_isolation/
      src/
        site_instance_manager.rs
        process_assignment_policy.rs
    display_pipeline/
      src/
        display_list.rs
        invalidation_tracker.rs
        frame_scheduler.rs
    standards_compliance/
      src/
        wpt_runner.rs
        reftest_harness.rs
        compat_flags.rs
    distribution/
      src/
        update_service.rs
        packaging.rs
    security_layer/
      src/
        site_isolation.rs
        certificate_validator.rs
        policy_coordinator.rs
        download_protection.rs
```

### Minimal dependency direction for new crates
- `browser_shell -> navigation_engine -> web_runtime/rendering/layout/painting`
- `web_runtime -> javascript_runtime/dom_engine/networking_stack/storage_system`
- `security_layer` can be called by every runtime edge but remains policy-only (no UI coupling)
- `standards_compliance` depends on all tested crates, but no runtime crate depends on it

---

## Closing recommendation

The current NUST architecture is a strong scaffold for a programmable browser, but achieving Chromium-class capability requires prioritizing **navigation+frame lifecycle correctness**, **WebIDL/Web API completeness**, **site isolation/security hardening**, and **standards compliance infrastructure** before advanced AI/cluster differentiators scale to production.
