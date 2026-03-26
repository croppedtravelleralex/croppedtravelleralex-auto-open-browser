# Architecture

## Project goal

`lightpanda-automation` is a high-performance browser automation service intended to run on Ubuntu.

The current design favors:

- low memory overhead
- small deployment footprint
- clear task lifecycle management
- future integration with `lightpanda-io/browser`
- pluggable execution backends

## Current stack

- Rust
- Axum
- Tokio
- SQLite
- systemd

## Core design

The system is intentionally split into a stable control plane and a replaceable execution plane.

### Control plane

The control plane is expected to remain stable even if the browser execution engine changes.

Responsibilities:

- expose HTTP API
- persist tasks and logs
- queue and dispatch tasks
- manage status transitions
- store artifacts metadata
- enforce concurrency limits

### Execution plane

The execution plane is responsible for actually running browser automation steps.

The current implementation uses a `FakeRunner` only to validate end-to-end task flow.

The long-term plan is to replace it with a real browser runner, ideally via an adapter around `lightpanda-io/browser`.

## Current module structure

### API layer

- `src/api/routes.rs`
- `src/api/handlers.rs`

Responsibilities:

- task creation
- task retrieval
- task listing
- task log retrieval
- health endpoint

### Configuration and shared state

- `src/config.rs`
- `src/state.rs`
- `src/error.rs`

Responsibilities:

- runtime configuration
- shared app state
- unified error handling

### Domain model

- `src/model/task.rs`

Responsibilities:

- task status model
- task DTOs
- task log DTOs
- step DSL definitions are referenced from runner layer

### Storage layer

- `src/storage/sqlite.rs`
- `migrations/001_init.sql`

Responsibilities:

- SQLite initialization
- schema bootstrap
- task persistence
- task log persistence
- state transitions

### Scheduler layer

- `src/scheduler/queue.rs`
- `src/scheduler/dispatcher.rs`

Responsibilities:

- in-memory task queue
- background dispatch loop
- concurrency limiting
- handoff to runner implementation

### Runner layer

- `src/runner/fake_runner.rs`
- `src/runner/browser_runner.rs`
- `src/runner/steps.rs`

Responsibilities:

- define execution step DSL
- define task runner behavior
- currently provide a fake execution path
- later host the real browser execution adapter

## Why the execution layer should stay pluggable

`lightpanda-io/browser` should currently be treated as an experimental candidate backend rather than a permanently assumed mature core dependency.

Because of that, the system should not be tightly coupled to one browser engine implementation.

A pluggable runner architecture provides:

- lower rewrite risk
- safer experimentation
- easier fallback to another backend
- cleaner separation between orchestration and execution

## Recommended next architecture evolution

Add a dedicated adapter subtree:

```text
src/runner/lightpanda/
  mod.rs
  adapter.rs
  cli.rs
  models.rs
  parser.rs
```

### Proposed responsibilities

- `adapter.rs`: define the browser adapter trait and shared result model
- `cli.rs`: implement process-based integration if `lightpanda/browser` is CLI-driven
- `models.rs`: request/response and normalized result types
- `parser.rs`: parse backend output into internal structures

## Execution model roadmap

### Stage 1

- keep fake runner
- finish compile stability
- preserve API/storage/scheduler architecture

### Stage 2

- introduce `BrowserAdapter` abstraction
- implement a thin `LightpandaAdapter`
- support minimal real actions: `goto`, `wait`, `screenshot`

### Stage 3

- add richer actions: `click`, `type`, `extract_text`, `evaluate`
- add structured artifact output
- add timeout and restart policies

### Stage 4

- move from per-task execution to reusable worker instances if `lightpanda/browser` supports stable long-lived sessions
- add worker pool lifecycle control

## Why SQLite is still correct here

SQLite is intentionally chosen for the first version because:

- it avoids another always-on service
- it keeps memory and deployment overhead low
- it is sufficient for task metadata and logs in a single-node Ubuntu deployment
- it supports a clean future migration path if needed

## Deployment model

Target deployment:

- one Rust binary
- one SQLite database file
- local artifact directory
- `systemd` service on Ubuntu

This keeps the first production shape small and operationally simple.

## Current known limitations

- compile is not fully green yet
- fake runner is still in use
- no artifact manager module yet
- no real `lightpanda/browser` adapter implemented yet
- no browser worker pool yet

## Current development priority

1. get `cargo check` clean
2. stabilize task lifecycle and API skeleton
3. add pluggable browser adapter layer
4. integrate a minimal real browser backend
5. optimize worker reuse only after correctness is proven
