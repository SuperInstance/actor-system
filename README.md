# Actor System

An **actor system** provides the runtime infrastructure for creating, scheduling, and managing actors — the fundamental unit of computation in the actor model.

## Why It Matters

Actor systems enable massive concurrency without locks or shared state. Each actor processes messages sequentially with isolated state. This model powers Erlang/OTP, Akka, and modern async runtimes.

## How It Works

Implements actor creation, addressing, message dispatch, supervision trees, and lifecycle management. Actors are lightweight (≈300 bytes) — millions can exist simultaneously.

## Usage

```toml
[dependencies]
actor-system = "0.1.0"
```

```rust
use actor_system;

// See examples/ directory for detailed usage
```

## API

- `SystemConfig` (lib.rs)
- `ActorSystem` (lib.rs)

## Architecture

This crate is part of the **[SuperInstance](https://github.com/SuperInstance)** ecosystem — a conservation-law-based framework for fleet coordination, ternary computation, and distributed agent systems.

### Related Crates

- [`superinstance-core`](https://github.com/SuperInstance/superinstance-core) — Core conservation law (γ + η = C)
- [`superinstance-harness`](https://github.com/SuperInstance/superinstance-harness) — Build harness and self-improving loop
- [`fleet-coordinator`](https://github.com/SuperInstance/fleet-coordinator) — Fleet-level coordination

## References

- [SuperInstance Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)
- [Conservation Law Paper](https://github.com/SuperInstance/SuperInstance/blob/main/docs/conservation-law.md)

## License

MIT
