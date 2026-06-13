# Actor System

**Actor System** is a Rust library implementing the foundational actor runtime — system configuration, worker thread pools, mailbox sizing, and lifecycle management — providing the root context in which all actors, supervisors, routers, and dispatchers operate. The actor model, introduced by Hewitt (1973) and popularized by Erlang/OTP and Akka, treats computation as a society of isolated processes that communicate exclusively through asynchronous message passing. Each actor processes one message at a time, maintains private state, and can create child actors — forming hierarchical supervision trees.

## Why It Matters

Shared-state concurrency is the hardest problem in systems programming. Mutexes deadlock, condition variables race, and lock-free data structures are notoriously difficult to get right. The actor model eliminates these issues by design: actors share no memory, communicate only through typed messages, and process messages sequentially. This maps naturally to:

- **Distributed systems** — actors on different machines are identical to actors on the same machine (location transparency)
- **Fault tolerance** — the "let it crash" philosophy: supervisors restart failed children rather than trying to patch corrupted state
- **Backpressure** — bounded mailboxes create natural flow control; a slow actor's mailbox fills, and senders must adapt
- **Scalability** — actors are cheap (a few hundred bytes each); systems can have millions

The mathematical foundation is the **Actor calculus** (Agha, 1986), a formal model of computation where the only operations are:

1. **Send** a message to a known address (asynchronous, unordered by default)
2. **Create** a new actor (with a fresh address)
3. **Become** a new behavior for handling future messages (state transition)

This crate provides the system-level scaffolding: configuration, thread pool sizing, mailbox capacity, and lifecycle hooks. It is the root from which the actor supervision tree grows.

## How It Works

### System Configuration

`SystemConfig` defines the runtime parameters:

| Parameter | Default | Description |
|-----------|---------|-------------|
| `name` | `"default"` | System identifier for logging and namespacing |
| `worker_threads` | 4 | Thread pool size for dispatching messages |
| `mailbox_capacity` | 1024 | Maximum messages queued per actor before backpressure |

**Thread pool sizing** follows Little's Law from queuing theory:

```
L = λ × W
```

Where L is the average number of messages in flight, λ is message arrival rate, and W is average processing time. The optimal thread count is:

```
N* = min(L, available_cores)
```

Setting `worker_threads` above the number of physical cores only helps when actors perform I/O (where threads block on external calls).

### Mailbox Capacity and Backpressure

The mailbox is a bounded FIFO queue. When the queue reaches `mailbox_capacity`, new messages must either be dropped, rejected (sender notified), or the sender must block. The capacity choice involves a tradeoff:

- **Large mailbox** (e.g., 65,536): absorbs burst traffic, higher memory per actor (64K × message_size)
- **Small mailbox** (e.g., 16): low memory, fast backpressure signaling, risk of message loss during bursts

**Memory per actor** with default capacity: `mailbox_capacity × sizeof(Message) + actor_state ≈ 1024 × 64B + 256B ≈ 65.5 KB`.

**Big-O complexity:**

| Operation | Time | Space |
|-----------|------|-------|
| Send message | O(1) | O(1) |
| Dequeue message | O(1) | O(1) |
| Actor spawn | O(1) | O(mailbox_capacity) |
| Shutdown | O(N) for N children | O(N) |
| Supervision check | O(children_per_supervisor) | O(1) |

### Lifecycle

```
Created → Initialized → Running → (Stopping → Stopped)
                          ↕
                       (Suspended → Restarted)
```

The `ActorSystem` handle provides `shutdown()` for graceful termination — all actors receive a `PoisonPill` message, finish processing their current message, and the system waits for all mailboxes to drain before exiting.

## Quick Start

```rust
use actor_system::{ActorSystem, SystemConfig};

// Create with default configuration
let system = ActorSystem::new(SystemConfig::default());
assert_eq!(system.name(), "default");

// Custom configuration
let config = SystemConfig {
    name: "my-app".into(),
    worker_threads: 8,
    mailbox_capacity: 4096,
};
let system = ActorSystem::new(config);
assert_eq!(system.name(), "my-app");

// Graceful shutdown
system.shutdown();
```

## API

### `SystemConfig`

Configuration for the actor system.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `"default"` | System name |
| `worker_threads` | `usize` | 4 | Dispatcher thread pool size |
| `mailbox_capacity` | `usize` | 1024 | Max messages per actor mailbox |

### `ActorSystem`

Handle to a running actor system.

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `(SystemConfig) → Self` | Create and start the system |
| `config` | `(&self) → &SystemConfig` | Access configuration |
| `name` | `(&self) → &str` | Get system name |
| `shutdown` | `(&self) → ()` | Signal graceful shutdown |

## Architecture Notes

The actor system provides the **root context** for all computation. Within γ + η = C, it represents the computational substrate C through which agents (γ) exchange messages and the environment (η) provides scheduling. The supervision tree's hierarchical structure instantiates the conservation law: a parent's responsibility for its children's failures is a conserved quantity — every crash must be handled by exactly one supervisor, preventing error propagation from violating system coherence.

The mailbox boundary between actors is the boundary between γ and η domains: messages crossing it are the mechanism by which agent contributions interact with environmental responses, and the conservation law ensures no message is silently lost (at-most-once or at-least-once delivery guarantees are the conservation invariant).

See the [architecture overview](https://github.com/casey-digennaro/actor-system/blob/main/ARCHITECTURE.md).

## References

1. Hewitt, C. (1973). "A Universal Modular Actor Formalism for Artificial Intelligence." *IJCAI*.
2. Agha, G. (1986). *Actors: A Model of Concurrent Computation in Distributed Systems*. MIT Press.
3. Armstrong, J. (2003). *Making Reliable Distributed Systems in the Presence of Software Errors*. PhD Thesis, KTH. (Erlang/OTP design)
4. Little, J.D.C. (1961). "A Proof for the Queuing Formula L = λW." *Operations Research*, 9(3), 383–387.

## License

MIT
