Sync issue
Mutex
Atomic Types
Memory Ordering
Mutex vs Atomic Types

**Mutex**: 
Use when you need to protect complex data(e.g struct, vectors) or perform multi step operations that can't be done atomically

**Atomic types**:
Use for simple, single value update (e.g counter, flags) where lock-free, CPU-level atomic operations are faster and sufficient , like incrementing a numbe ro r updating a flag or toggleing a boolean