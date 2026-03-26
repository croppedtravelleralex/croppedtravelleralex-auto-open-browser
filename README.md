# lightpanda-automation

A high-performance browser automation service for Ubuntu, designed around a Rust control plane and a future `lightpanda-io/browser` runner.

## Current status

This first version provides:

- Rust HTTP API server
- SQLite task metadata storage
- In-memory task queue
- Background dispatcher
- Fake runner for end-to-end task lifecycle validation

## Planned next step

Replace the fake runner with a real `lightpanda/browser` integration while keeping the API, scheduler, and storage layers unchanged.
