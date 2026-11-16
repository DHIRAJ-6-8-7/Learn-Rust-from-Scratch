# Learning Rust from Scratch

15-Day Systems & Backend Engineering Intensive

Edition**Rust 2024**

Phase**Core Language & Modules**

Duration**15 Days**

Target**Backend & Distributed Systems**

Pick an operation and watch where the data goes. The compiler decides who owns the heap value.

```

```

```

```

## About

Author

Dhiraj Shrestha

Institution

Institute of Engineering, Pulchowk Campus (IOE), Nepal

Prior Experience

C/C++PythonFastAPIPostgreSQLRedisLinuxGitNetworking

Engineering Focus

Backend systems, distributed systems, network programming

This repository documents a structured 15-day intensive study of the Rust programming language, progressing from fundamentals to a production-grade capstone project. The approach follows professional mentorship methodology: every concept is taught with its motivation, mental model, industry usage, and common failure modes.

## Objectives

- Develop the ability to read, write, review, and ship idiomatic Rust in a professional setting
- Build a deep understanding of why Rust’s ownership, type, and concurrency models work the way they do
- Complete a Redis-compatible in-memory key-value server as the capstone project
- Reach independent learning capability by Day 15

## Curriculum

## Progress Notes & Technical Deep Dives

Day 02–03 Milestones Covered

## Capstone Project: Redis-Compatible Key-Value Server

An async, Redis-compatible in-memory key-value server built incrementally from Day 9 onwards.

### Planned features

- Async TCP server on tokio implementing the RESP protocol, compatible with `redis-cli`
- Supported commands: `GET`, `SET` (with `EX`/`PX`), `DEL`, `EXISTS`, `INCR`, `EXPIRE`, `TTL`, `PING`, `PUBLISH/SUBSCRIBE`
- Concurrent, sharded in-memory store with TTL expiry
- Persistence via append-only file with crash recovery on startup
- Structured logging with `tracing`, signal handling, and graceful shutdown
- Unit, integration, and property-based tests
- Criterion benchmarks with comparison against real Redis
- GitHub Actions CI pipeline and full documentation

Browser preview of the command surface. Try a command below.

127.0.0.1:6379>

Simulated in the page for a feel of the planned behaviour. The real server is built on tokio from Day 9. PUBLISH/SUBSCRIBE is not part of this preview.

## Setup
