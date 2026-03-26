# lightpanda-automation

A high-performance browser automation service for Ubuntu.

This project is being built as a lightweight control plane around a future browser execution backend, with `lightpanda-io/browser` currently treated as the primary experimental candidate.

## Design goals

- low memory overhead
- small deployment footprint
- clear task lifecycle management
- pluggable browser execution backend
- high-performance single-node Ubuntu deployment

## Current stack

- Rust
- Axum
- Tokio
- SQLite
- systemd

## Current implementation status

The current codebase already includes:

- HTTP API skeleton
- SQLite task storage
- task and log schema migration
- in-memory task queue
- background dispatcher
- fake runner for end-to-end validation
- systemd service definition
- project architecture document

## Current API surface

### `GET /health`
Returns a simple health response.

### `POST /tasks`
Creates a task and queues it for execution.

### `GET /tasks`
Lists tasks.

### `GET /tasks/:id`
Returns a single task.

### `GET /tasks/:id/logs`
Returns task logs.

## Why the runner is still fake

The current implementation uses a `FakeRunner` on purpose.

This allows the project to validate the following system layers first:

- API behavior
- persistence
- queueing
- dispatch lifecycle
- result persistence
- error flow

A real browser backend should only be wired in after the control plane is stable.

## Current architecture direction

The system is split into:

### Control plane
Responsible for:

- HTTP API
- persistence
- queueing
- dispatch
- task lifecycle management

### Execution plane
Responsible for:

- browser automation execution
- screenshots
- extraction
- future browser worker pooling

This design keeps the execution backend replaceable.

## Planned `lightpanda/browser` integration approach

Instead of tightly coupling the whole system to one browser engine, the next step is to add an adapter layer under:

```text
src/runner/lightpanda/
```

Expected future modules:

```text
src/runner/lightpanda/
  mod.rs
  adapter.rs
  cli.rs
  models.rs
  parser.rs
```

This keeps `lightpanda-io/browser` integration isolated and makes future backend replacement possible.

## Current limitations

- `cargo check` is not fully green yet
- real browser execution is not integrated yet
- no artifact manager module yet
- no worker pool yet
- `lightpanda/browser` should still be treated as an experimental backend candidate until its integration surface is verified on Ubuntu

## Near-term roadmap

1. get `cargo check` fully passing
2. stabilize task lifecycle and API skeleton
3. add a pluggable browser adapter layer
4. implement a minimal real browser backend (`goto`, `wait`, `screenshot`)
5. optimize worker reuse after correctness is proven

## Deployment target

The intended Ubuntu deployment shape is:

- one Rust binary
- one SQLite database file
- one local artifact directory
- one `systemd` service

## Documentation

See:

- `docs/architecture.md`
