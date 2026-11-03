# Learning Rust from Scratch — 15-Day Intensive

<p align="center">
  <img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" alt="Rust Logo" width="120"/>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust_2024_Edition-orange?style=flat-square&logo=rust" />
  <img src="https://img.shields.io/badge/Status-In_Progress-blue?style=flat-square" />
  <img src="https://img.shields.io/badge/Duration-15_Days-green?style=flat-square" />
  <img src="https://img.shields.io/badge/Focus-Backend_Engineering-black?style=flat-square" />
</p>

---

## About

**Author:** Dhiraj Shrestha
**Institution:** Institute of Engineering, Pulchowk Campus (IOE), Nepal
**Prior Experience:** C/C++, Python, FastAPI, PostgreSQL, Redis, Linux, Git, Networking
**Engineering Focus:** Backend systems, distributed systems, network programming

This repository documents a structured 15-day intensive study of the Rust programming language, progressing from fundamentals to a production-grade capstone project. The approach follows professional mentorship methodology: every concept is taught with its motivation, mental model, industry usage, and common failure modes.

---

## Objectives

- Develop the ability to read, write, review, and ship idiomatic Rust in a professional setting
- Build a deep understanding of why Rust's ownership, type, and concurrency models work the way they do
- Complete a Redis-compatible in-memory key-value server as the capstone project
- Reach independent learning capability by Day 15

---

## Curriculum

| Day | Topic                                                                           | Status      |
| --- | ------------------------------------------------------------------------------- | ----------- |
| 01  | Toolchain setup, types, variables, mutability, control flow, functions, modules | In Progress |
| 02  | Ownership, moves, borrowing, references, slices, Copy vs Clone, RAII            | Pending     |
| 03  | Structs, enums, pattern matching, Option, Result, error handling                | Pending     |
| 04  | Collections, String vs &str, iterators, closures, adapters                      | Pending     |
| 05  | Traits, generics, trait objects, static vs dynamic dispatch, lifetimes          | Pending     |
| 06  | Modules, crates, workspaces, testing, documentation, semver                     | Pending     |
| 07  | Smart pointers, interior mutability, professional error handling, CLI project   | Pending     |
| 08  | Concurrency: threads, channels, Arc/Mutex, atomics, rayon                       | Pending     |
| 09  | Async Rust: futures, async/await, tokio internals — Capstone begins             | Pending     |
| 10  | Networking with tokio: TCP, framing, protocol parsing, backpressure             | Pending     |
| 11  | serde, axum, sqlx with PostgreSQL, configuration management                     | Pending     |
| 12  | Observability: tracing, structured logging, graceful shutdown, benchmarking     | Pending     |
| 13  | Unsafe Rust, FFI with C, macros, performance tuning, memory layout              | Pending     |
| 14  | CI/CD, property testing, fuzzing, release profiles, API design patterns         | Pending     |
| 15  | Capstone finalization, benchmarks, code review, 3-month learning roadmap        | Pending     |

---

## Capstone Project — Redis-Compatible Key-Value Server

<p align="center">
  <img src="https://www.rust-lang.org/static/images/wasm-ferris.png" alt="Rust Ferris" width="160"/>
</p>

An async, Redis-compatible in-memory key-value server built incrementally from Day 9 onwards.

**Planned features:**

- Async TCP server on tokio implementing the RESP protocol, compatible with `redis-cli`
- Supported commands: `GET`, `SET` (with `EX`/`PX`), `DEL`, `EXISTS`, `INCR`, `EXPIRE`, `TTL`, `PING`, `PUBLISH/SUBSCRIBE`
- Concurrent, sharded in-memory store with TTL expiry
- Persistence via append-only file with crash recovery on startup
- Structured logging with `tracing`, signal handling, and graceful shutdown
- Unit, integration, and property-based tests
- Criterion benchmarks with comparison against real Redis
- GitHub Actions CI pipeline and full documentation

---

## Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Check for errors (fast — use after every change)
cargo check

# Run
cargo run

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt

# Test
cargo test
```

---

## Repository Structure

```
.
├── day01/          # Types, variables, control flow, functions, modules
├── day02/          # Ownership and borrowing
├── ...
├── day09/          # Capstone milestone 1 — async TCP server skeleton
├── ...
└── capstone/       # Redis-compatible KV server — final project
```

---

## References

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [docs.rs](https://docs.rs)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

---

## Progress

- [x] Toolchain configured
- [ ] Day 1 complete
- [ ] Day 2 complete
- [ ] Day 3 complete
- [ ] Day 4 complete
- [ ] Day 5 complete
- [ ] Day 6 complete
- [ ] Day 7 complete
- [ ] Day 8 complete
- [ ] Capstone started
- [ ] Capstone complete
