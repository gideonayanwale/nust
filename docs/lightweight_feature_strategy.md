# NUST Lightweight Feature Strategy

Goal: move toward broad browser feature parity while keeping NUST resource-efficient.

## Principle
- "Feature parity by capability, not by bloat"
- keep expensive systems off by default unless needed
- expose profile modes for lightweight/balanced/compatibility

## Implemented now
- browser settings profile with `Lightweight`, `Balanced`, `MaximumCompatibility`
- skin setting with Edge + Chrome + Brave inspired fusion theme
- capability matrix report to track parity trajectory and runtime posture
- per-tab thread process registry hook in tab manager
- automation workflow registry for autopilot-ready tabs

## Next steps
1. tie capability matrix to real subsystem readiness checks
2. implement predictive preload only in balanced/compat mode
3. add tab lifecycle discarding heuristics based on memory pressure
4. gate high-cost extensions/AI features by profile and origin trust
