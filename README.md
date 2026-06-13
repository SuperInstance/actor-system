# Actor System

**Actor System** is a Rust library implementing the foundational actor runtime — system configuration, worker thread pools, mailbox sizing, and lifecycle management — providing the root context in which all actors, supervisors, routers, and dispatchers operate.

## Why It Matters

An actor system is the runtime container that manages thread pools, scheduling, mailbox delivery, and the actor lifecycle hierarchy. Without it, individual actors are isolated objects with no infrastructure for message delivery, failure propagation, or resource management. The actor system provides three critical services: (1) a configurable thread pool that executes actor message handlers, (2) bounded mailboxes that apply back-pressure, and (3) a hierarchical supervision tree root that contains failures. This mirrors the Erlang/OTP and Akka model, where the `ActorSystem` is the single entry point for creating, configuring, and shutting down all actors in an application.

## How It Works

The actor system is configured via `SystemConfig` and holds a shared handle (`Arc<SystemConfig>`) accessible to all actors:

**Thread pool sizing:** The default of 4 worker threads matches the typical multi-core desktop. The optimal pool size follows Little's Law:

```
N_threads = arrival_rate × mean_service_time
```

For CPU-bound actors, N_threads ≈ physical cores. For I/O-bound actors (network, disk), N_threads can exceed cores significantly because threads spend time blocked.

**Mailbox capacity:** Default 1024 messages. Sized to absorb burst traffic without applying back-pressure on every spike. Too small: producers throttle unnecessarily. Too large: memory pressure and latency accumulation. The capacity is a tuning parameter:

```
mailbox_size ≈ burst_size × safety_factor
```

Where burst_size is the maximum expected message rate × tolerated latency.

**Lifecycle:**
- `new(config)` — initialize the system (O(1))
- `name()` — access system identity (O(1))
- `config()` — read-only config access (O(1))
- `shutdown()` — graceful termination signal

The `ActorSystem` uses `Arc` internally so it can be cheaply cloned and shared across threads. The actual actor spawning and scheduling are delegated to the runtime (typically `tokio` or a custom executor).

| Config Parameter | Default | Purpose |
|-----------------|---------|---------|
| `name` | "default" | System identity for logging/coordination |
| `worker_threads` | 4 | Parallelism for message processing |
| `mailbox_capacity` | 1024 | Back-pressure threshold per actor |

## Quick Start

```rust
fn main() {
    let sys = ActorSystem::new(SystemConfig::default());
    assert_eq!(sys.name(), "default");
    println!("Threads: {}", sys.config().worker_threads);
    println!("Mailbox: {}", sys.config().mailbox_capacity);
    sys.shutdown();
}
```

## API

| Type/Method | Description |
|-------------|-------------|
| `SystemConfig` | Configuration: name, worker_threads, mailbox_capacity |
| `ActorSystem::new` | `(SystemConfig) → ActorSystem` |
| `ActorSystem::config` | `() → &SystemConfig` |
| `ActorSystem::name` | `() → &str` |
| `ActorSystem::shutdown` | `() → ()` |

## Architecture Notes

The Actor System is the **root runtime container** in the SuperInstance fleet. It hosts the supervision tree that enforces γ + η = C conservation: γ-layer actors (sensor readers, physical controllers) and η-layer actors (model inference, planning) share the same system but run in separate thread pools with independently sized mailboxes, ensuring that a flood of sensor data cannot starve the planning layer.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

1. Agha, G. (1986). *Actors: A Model of Concurrent Computation in Distributed Systems*. MIT Press.
2. Armstrong, J. (2003). *Making Reliable Distributed Systems in the Presence of Software Errors*. PhD Thesis, KTH.
3. Little, J.D.C. (1961). "A Proof for the Queuing Formula L = λW." *Operations Research*, 9(3), 383–387.

## License

MIT
