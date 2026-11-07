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

| Day | Topic                                                                                                           | Status      |
| --- | --------------------------------------------------------------------------------------------------------------- | ----------- |
| 01  | Toolchain setup, types, variables, mutability, control flow, functions, modules                                 | Completed   |
| 02  | Ownership, moves, borrowing, references, slices, Copy vs Drop, RAII, NLL                                        | Completed   |
| 03  | Structs (Named, Tuple, Unit-like), Debug/Display traits, Methods, Associated Functions, Enums, Pattern Matching | In Progress |
| 04  | Collections, String vs &str, iterators, closures, adapters                                                      | Pending     |
| 05  | Traits, generics, trait objects, static vs dynamic dispatch, lifetimes                                          | Pending     |
| 06  | Modules, crates, workspaces, testing, documentation, semver                                                     | Pending     |
| 07  | Smart pointers, interior mutability, professional error handling, CLI project                                   | Pending     |
| 08  | Concurrency: threads, channels, Arc/Mutex, atomics, rayon                                                       | Pending     |
| 09  | Async Rust: futures, async/await, tokio internals — Capstone begins                                             | Pending     |
| 10  | Networking with tokio: TCP, framing, protocol parsing, backpressure                                             | Pending     |
| 11  | serde, axum, sqlx with PostgreSQL, configuration management                                                     | Pending     |
| 12  | Observability: tracing, structured logging, graceful shutdown, benchmarking                                     | Pending     |
| 13  | Unsafe Rust, FFI with C, macros, performance tuning, memory layout                                              | Pending     |
| 14  | CI/CD, property testing, fuzzing, release profiles, API design patterns                                         | Pending     |
| 15  | Capstone finalization, benchmarks, code review, 3-month learning roadmap                                        | Pending     |

---

## Progress Notes & Technical Deep Dives

### Day 02–03 Milestones Covered:

- **Ownership & Borrowing:** Mastered partial moves in structs (tracking ownership down to individual fields) and borrowing rules (`&self` vs `&mut self`).
- **Struct Variants:** Explored classic named-field structs, lightweight **tuple structs** for domain type safety, and zero-sized **unit-like structs** for trait anchoring and compile-time marker types.
- **Formatting Traits & Macros:** Implemented `#[derive(Debug)]` for `{:#?}` multi-line inspection, understood `std::fmt::Display` for user-facing output, and utilized the `dbg!` macro (prints expression + line number to `stderr` while returning ownership).
- **Methods & Ergonomics:** Defined `impl` blocks using dot syntax and leveraged Rust’s **automatic referencing and dereferencing** mechanism.
- **Associated Functions:** Implemented constructor patterns (`Type::new()`) and factory functions (`Type::square()`) using double-colon (`::`) namespace invocation.

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
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh

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
