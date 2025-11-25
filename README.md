# Rust Connection Pooler Project Guide

## Project Overview

Build a connection pooler for PostgreSQL and potentially ScyllaDB in Rust. This project will deepen your understanding of database connection management, the Postgres wire protocol, and async Rust programming while giving you hands-on experience with the internals of tools like PgBouncer that you use daily.

## Learning Goals

- Master async I/O patterns in Rust using Tokio
- Understand the PostgreSQL wire protocol at a deep level
- Learn connection lifecycle management and pooling strategies
- Explore load balancing and query routing logic
- Build observability into distributed systems
- Understand the tradeoffs PgBouncer makes and why

## Implementation Roadmap

### Phase 1: Basic Pass-Through Proxy

- Accept client connections on a TCP port
- Forward raw bytes to actual Postgres server
- Return responses back to client
- Goal: Understand basic TCP proxying and Rust async patterns

### Phase 2: Connection Pooling

- Maintain N persistent connections to Postgres
- Implement connection checkout/checkin logic
- Handle connection recycling and cleanup
- Add configurable pool size and timeout settings

### Phase 3: Pooling Modes

- Session mode: Client gets dedicated connection for entire session
- Transaction mode: Connection returned after each transaction
- Statement mode: Connection returned after each statement (hardest)
- Understand when each mode is appropriate and their tradeoffs

### Phase 4: Query Routing

- Parse queries to detect reads vs writes
- Route reads to replica servers, writes to primary
- Handle transaction boundaries correctly
- Deal with consistency requirements (reads after writes)

### Phase 5: Observability and Metrics

- Track active connections, queue depth, wait times
- Log slow queries and connection errors
- Expose metrics endpoint (Prometheus format)
- Add health checking for backend databases

## Critical Concepts to Master

### PostgreSQL Wire Protocol

The protocol Postgres uses for client-server communication. You need to understand:

- Startup message and authentication flow
- Simple vs extended query protocol
- Message types (Query, Parse, Bind, Execute, etc)
- Transaction state tracking

### Connection States

- IDLE: Ready for new query
- ACTIVE: Query in progress
- IDLE IN TRANSACTION: Inside BEGIN block
- FAILED TRANSACTION: After error, waiting for ROLLBACK

### Key Challenges You Will Face

1. State management: Tracking which connections are in use, in transactions, etc
2. Error handling: What happens when backend dies or client disconnects mid-query
3. Protocol parsing: Need to understand message boundaries without a full parser
4. Async coordination: Managing thousands of connections efficiently
5. Fairness: Preventing connection starvation under load

## Technical Stack

### Core Dependencies

- tokio: Async runtime (start here, this is fundamental)
- tokio-postgres: For understanding the protocol (study the source code)
- bytes: Efficient byte buffer management
- tracing: Structured logging and diagnostics
- serde: Configuration parsing

### Optional but Useful

- prometheus: Metrics export
- clap: CLI argument parsing
- config: TOML/YAML configuration files

## Architecture Overview

Your pooler sits between clients and databases, multiplexing many client connections onto fewer backend connections. Clients connect to your pooler, which maintains a pool of connections to the actual database servers.

## What Makes This Different from PgBouncer

Add ONE distinguishing feature. Ideas:

- Query pattern analysis: Track which queries are slow, which tables are hot
- Adaptive routing: Learn which replicas are faster for certain query patterns
- Better observability: Per-client metrics, query latency histograms
- Smart prefetching: Predict connection needs based on traffic patterns
- ScyllaDB support: Extend to work with Scylla protocol too

Pick something that would actually help your team debug production issues.

## Development Strategy

### Week 1-2: Foundation

- Get basic TCP proxy working
- Study tokio and async Rust patterns
- Read PgBouncer source code and documentation
- Understand Postgres wire protocol basics

### Week 3-4: Core Pooling

- Implement connection pool data structure
- Add checkout/checkin logic
- Handle connection lifecycle
- Test with real Postgres instance

### Week 5-6: Advanced Features

- Add transaction vs session mode
- Implement basic query parsing
- Add metrics and logging
- Handle error cases properly

### Week 7 and Beyond: Polish and Unique Feature

- Implement your distinguishing feature
- Performance testing and optimization
- Documentation
- Deploy and use it for something real

## Resources You Need

### Must Read

- PgBouncer documentation and source code
- PostgreSQL wire protocol documentation (official docs)
- Tokio tutorial and documentation
- Asynchronous Programming in Rust book

### Study These Crates

- tokio-postgres source code (they have already solved these problems)
- pgbouncer C source (understand design decisions)
- deadpool or bb8 (Rust connection pool examples)

### Debugging Tools

- pgbench for load testing
- tcpdump or wireshark to inspect protocol traffic
- Postgres logs with detailed query logging
- strace to watch system calls

## Common Pitfalls to Avoid

1. Do not parse SQL fully: You only need to detect transaction boundaries and read/write patterns, not execute queries
2. Connection leaks will happen: Plan for them, add timeouts everywhere
3. Authentication is tricky: Start with trust auth, add real auth later
4. Prepared statements: These are per-connection and complicate pooling significantly
5. Network failures: Backend can die, client can disconnect, network can partition

## Success Criteria

You will know this project succeeded when:

1. You can explain exactly what PgBouncer does and why
2. You understand Postgres wire protocol well enough to debug production issues
3. You are comfortable with async Rust and tokio patterns
4. You have a working pooler that you actually use for something
5. You can discuss connection pooling tradeoffs with your team intelligently

## Next Steps After Completion

- Write a blog post explaining what you learned
- Present it to your team
- Consider contributing to actual pgbouncer or related tools
- Maybe add ScyllaDB support (different protocol, good learning)
- Performance comparison: your pooler vs PgBouncer

## The Real Win

This is not about replacing PgBouncer. It is about understanding the system you work with every day at a fundamental level. When production breaks at 3am because of connection exhaustion, you will know exactly what is happening and why.

Start simple, ship phases incrementally, and do not try to build everything at once. Get Phase 1 working this week.