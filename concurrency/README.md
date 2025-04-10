Sync issue
Mutex
Atomic Types
Memory Ordering
Mutex vs Atomic Types

**Why is Arc used in this example?****
To share ownership across threads


**Mutex**: 
Use when you need to protect complex data(e.g struct, vectors) or perform multi step operations that can't be done atomically

**Atomic types**:
Use for simple, single value update (e.g counter, flags) where lock-free, CPU-level atomic operations are faster and sufficient , like incrementing a numbe ro r updating a flag or toggleing a boolean



Concurrency vs Parallelism

- Concurrency allows for better resource utilization.
- It can improve responsiveness in applications.
- It is often implemented using threads, async/await patterns, or event loops.

ø Parallelism can significantly reduce execution time for large computations.
ø It requires careful management of shared resources to avoid race conditions.
ø It is often implemented using multi-threading or distributed computing.

