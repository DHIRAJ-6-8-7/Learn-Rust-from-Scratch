<p align="center">
  <img src="assets/hero.svg" alt="Learning Rust from Scratch: 15-Day Systems and Backend Engineering Intensive" width="100%">
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Edition-Rust_2024-ce422b?style=for-the-badge&logo=rust&logoColor=white" alt="Edition Rust 2024" />
  <img src="https://img.shields.io/badge/Phase-Core_Language_%26_Modules-1f2937?style=for-the-badge" alt="Phase Core Language and Modules" />
  <img src="https://img.shields.io/badge/Duration-15_Days-2e7d32?style=for-the-badge" alt="Duration 15 Days" />
  <img src="https://img.shields.io/badge/Target-Backend_%26_Distributed_Systems-0d1117?style=for-the-badge" alt="Target Backend and Distributed Systems" />
</p>

<p align="center">
  <a href="#about">About</a> &nbsp;|&nbsp;
  <a href="#objectives">Objectives</a> &nbsp;|&nbsp;
  <a href="#curriculum">Curriculum</a> &nbsp;|&nbsp;
  <a href="#progress-notes--technical-deep-dives">Progress Notes</a> &nbsp;|&nbsp;
  <a href="#capstone-project--redis-compatible-key-value-server">Capstone</a> &nbsp;|&nbsp;
  <a href="#setup">Setup</a>
</p>

---

## About

| | |
| --- | --- |
| **Author** | Dhiraj Shrestha |
| **Institution** | Institute of Engineering, Pulchowk Campus (IOE), Nepal |
| **Prior Experience** | C/C++, Python, FastAPI, PostgreSQL, Redis, Linux, Git, Networking |
| **Engineering Focus** | Backend systems, distributed systems, network programming |

This repository documents a structured 15-day intensive study of the Rust programming language, progressing from fundamentals to a production-grade capstone project. The approach follows professional mentorship methodology: every concept is taught with its motivation, mental model, industry usage, and common failure modes.

---

## Objectives

- Develop the ability to read, write, review, and ship idiomatic Rust in a professional setting
- Build a deep understanding of why Rust's ownership, type, and concurrency models work the way they do
- Complete a Redis-compatible in-memory key-value server as the capstone project
- Reach independent learning capability by Day 15

---

## Curriculum

<p align="center">
  <img src="assets/progress.svg" alt="Progress: 3 of 15 days completed, Day 4 in progress" width="100%">
</p>

| Day | Topic | Status |
| --- | --- | --- |
| 01 | Toolchain setup, types, variables, mutability, control flow, functions, modules | ![Completed](https://img.shields.io/badge/-Completed-2e7d32?style=flat-square) |
| 02 | Ownership, moves, borrowing, references, slices, Copy vs Drop, RAII, NLL | ![Completed](https://img.shields.io/badge/-Completed-2e7d32?style=flat-square) |
| 03 | Structs (Named, Tuple, Unit-like), Debug/Display traits, Methods, Associated Functions, Enums, Pattern Matching | ![Completed](https://img.shields.io/badge/-Completed-2e7d32?style=flat-square) |
| 04 | Collections, String vs &str, iterators, closures, adapters | ![In Progress](https://img.shields.io/badge/-In_Progress-d99a1e?style=flat-square) |
| 05 | Traits, generics, trait objects, static vs dynamic dispatch, lifetimes | ![Pending](https://img.shields.io/badge/-Pending-4b5563?style=flat-square) |
| 06 | Modules, crates, workspaces, testing, documentation, semver | ![Pending](https://img.shields.io/badge/-Pending-4b5563?style=flat-square) |
| 07 | Smart pointers, interior mutability, professional error handling, CLI project | ![Pending](https://img.shields.io/badge/-Pending-4b5563?style=flat-square) |
| 08 | Concurrency: threads, channels, Arc/Mutex, atomics, rayon | ![Pending](https://img.shields.io/badge/-Pending-4b5563?style=flat-square) |
| 09 | Async Rust: futures, async/await, tokio internals — Capstone begins | ![Pending](https://img.shields.io/badge/-Pending-4b5563?style=flat-square) |
| 10 | Networking with tokio: TCP, framing, protocol parsing, backpressure | ![Pending](https://img.shields.io/badge/-Pending-4b5563?style=flat-square) |
| 11 | serde, axum, sqlx with PostgreSQL, configuration management | ![Pending](https://img.shields.io/badge/-Pending-4b5563?style=flat-square) |
| 12 | Observability: tracing, structured logging, graceful shutdown, benchmarking | ![Pending](https://img.shields.io/badge/-Pending-4b5563?style=flat-square) |
| 13 | Unsafe Rust, FFI with C, macros, performance tuning, memory layout | ![Pending](https://img.shields.io/badge/-Pending-4b5563?style=flat-square) |
| 14 | CI/CD, property testing, fuzzing, release profiles, API design patterns | ![Pending](https://img.shields.io/badge/-Pending-4b5563?style=flat-square) |
| 15 | Capstone finalization, benchmarks, code review, 3-month learning roadmap | ![Pending](https://img.shields.io/badge/-Pending-4b5563?style=flat-square) |

---

## Progress Notes & Technical Deep Dives

### Day 02–03 Milestones Covered

Expand any milestone to read the notes.

<p align="center">
  <img src="assets/ownership.svg" alt="Animation: a value moves from variable a to variable b, and using a afterwards is a compile error" width="80%">
</p>

<details open>
<summary><b>Ownership & Borrowing</b></summary>
<br>

Mastered partial moves in structs (tracking ownership down to individual fields) and borrowing rules (`&self` vs `&mut self`).

</details>

<details>
<summary><b>Struct Variants</b></summary>
<br>

Explored classic named-field structs, lightweight **tuple structs** for domain type safety, and zero-sized **unit-like structs** for trait anchoring and compile-time marker types.

</details>

<details>
<summary><b>Formatting Traits & Macros</b></summary>
<br>

Implemented `#[derive(Debug)]` for `{:#?}` multi-line inspection, understood `std::fmt::Display` for user-facing output, and utilized the `dbg!` macro (prints expression + line number to `stderr` while returning ownership).

</details>

<details>
<summary><b>Methods & Ergonomics</b></summary>
<br>

Defined `impl` blocks using dot syntax and leveraged Rust's **automatic referencing and dereferencing** mechanism.

</details>

<details>
<summary><b>Associated Functions</b></summary>
<br>

Implemented constructor patterns (`Type::new()`) and factory functions (`Type::square()`) using double-colon (`::`) namespace invocation.

</details>

<details>
<summary><b>Enums & <code>Option&lt;T&gt;</code></b></summary>
<br>

Modeled algebraic data types with variant payloads and eliminated null-pointer failures using Rust's `Option<T>` (`Some(T)` / `None`).

</details>

<details>
<summary><b>Pattern Matching & Control Flow</b></summary>
<br>

Mastered `match` exhaustiveness checks, payload extraction/binding, catch-all wildcards (`_`) vs. named catch-all variables, the unit type (`()`), and concise syntactic sugar with `if let` and `let...else` (diverging early exits to stay on the "Happy Path").

</details>

---

## Capstone Project — Redis-Compatible Key-Value Server

<p align="center">
  <img src="assets/terminal.svg" alt="Animated redis-cli session showing the planned behaviour of the server" width="85%">
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
