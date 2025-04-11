Sync issue
Mutex
Atomic Types
Memory Ordering
Mutex vs Atomic Types

**Why is Arc used in this example?****
To share ownership across threads


**Mutex**: 

Shared state or data means that multiple threads access the same memory location at the same time. Rust uses mutexes (mutual exclusion locks) to implement shared-memory concurrency primitives.
Mutex is short for mutual exclusion, meaning only one thread can access certain data at any given time. To access data in a mutex, a thread must first acquire the lock. The lock is a data structure that tracks who currently has exclusive access.


Use when you need to protect complex data(e.g struct, vectors) or perform multi step operations that can't be done atomically

>Note: Mutex can still cause deadlocks. This can happen if two operations each need to lock two resources and two threads hold one lock each, waiting on the other—resulting in a circular wait.



**Atomic types**:
Use for simple, single value update (e.g counter, flags) where lock-free, CPU-level atomic operations are faster and sufficient , like incrementing a numbe ro r updating a flag or toggleing a boolean


> Rc<T> + RefCell<T> is typically used for single-threaded interior mutability

>    Arc<T> + Mutex<T> is used for multithreaded interior mutability


Concurrency vs Parallelism

- Concurrency allows for better resource utilization.
- It can improve responsiveness in applications.
- It is often implemented using threads, async/await patterns, or event loops.


    x Parallelism can significantly reduce execution time for large computations.
    x It requires careful management of shared resources to avoid race conditions.
    x It is often implemented using multi-threading or distributed computing.

#### Key Differences Between Concurrency and Parallelism
Execution:

- Concurrency is about dealing with lots of things at once (interleaving).
- Parallelism is about doing lots of things at once (simultaneously).

Use Cases:  
- Concurrency is ideal for I/O-bound tasks.
- Parallelism is suited for CPU-bound tasks.
Implementation:  
- Concurrency can be achieved with threads, async programming, or event-driven models.
- Parallelism typically requires multi-threading or distributed systems.


Basic terminology

### Concurrent Programming

- Threads: Rust provides a simple way to create threads using the std::thread module. You can spawn a new thread with the thread::spawn function, allowing your applications to perform multiple tasks simultaneously.
- Mutexes: To safely share data between threads, Rust uses Mutex from the std::sync module. A Mutex allows only one thread to access the data at a time, preventing data races. Our expertise ensures that your data remains secure and consistent.
- Channels: Rust provides channels for communication between threads. The std::sync::mpsc module allows you to create channels to send messages between threads safely. This facilitates efficient inter-thread communication, enhancing your application's performance.



Threads

Sharing Data Between Threads through the use of Arc (Atomic Reference Counted) and Mutex (Mutual Exclusion).

- Arc: This is a thread-safe reference-counting pointer that allows multiple threads to own the same data. It ensures that the data is deallocated only when all references are dropped.
- Mutex: This is a synchronization primitive that allows only one thread to access the data at a time. It provides a lock mechanism to ensure that data is accessed safely.


Thread safety is a crucial concept in concurrent programming, ensuring that shared data is accessed and modified safely by multiple threads.


To ensure thread safety in your Rust applications, consider the following:

- Use Arc<T> for shared ownership of data across threads.
- Leverage synchronization primitives like Mutex<T> or RwLock<T> to manage access to shared data.
- Always check if a type implements Send when passing it to a thread.


- RwLock<T>: A read-write lock that allows multiple readers or one writer at a time. This is useful when read operations are more frequent than write operations.
- Condvar: A condition variable that allows threads to wait for certain conditions to be met before proceeding. It is often used in conjunction with Mutex.

Barriers and semaphores are synchronization mechanisms that help manage the execution of threads in concurrent programming synchronization.

- Barriers:  
    - A barrier allows multiple threads to wait until a certain condition is met before proceeding.
    - It is useful for synchronizing phases of computation among threads.
- Semaphores:  
    - A semaphore is a signaling mechanism that controls access to a shared resource.
    - It maintains a count that represents the number of available resources.
    - Threads can acquire or release the semaphore, allowing for controlled access.



### Message Passing 

Message passing is a method for inter-process or inter-thread communication where data is exchanged through messages sent between entities via a communication channel.

This approach is often used in concurrent programming to avoid shared state and reduce the complexity of synchronization.

Real world scenarios
Isolation and Debugging:
Microservices Architecture: In a microservices-based application, different services communicate via APIs (often using message queues or RESTful calls). Each microservice runs in its own container or virtual machine, ensuring that they do not directly access each other’s data. This isolation makes it easier to debug and manage services independently, as issues in one service do not directly impact others. For instance, in an e-commerce platform, services for inventory, user accounts, and payment processing can be developed, tested, and debugged separately.


Portability and Scalability:
Cloud Computing: Cloud-based applications often use message passing through APIs or message queues to communicate between distributed components. For example, Amazon Web Services (AWS) uses services like SQS (Simple Queue Service) for message passing between different services. This architecture allows applications to scale horizontally by adding more instances without requiring changes to the underlying hardware or operating system, facilitating easy adaptation to varying loads and environments.



Security and Robustness:
Distributed File Systems: In distributed file systems like Google File System (GFS) or Hadoop Distributed File System (HDFS), message passing is used to manage file operations and handle node failures. These systems use messages to coordinate file storage, replication, and recovery. If a node fails, the system can recover by retransmitting messages to other nodes, ensuring data integrity and system resilience against failures. The isolation between nodes helps prevent unauthorized access and accidental interference.



In Rust, one primary tool for message-passing concurrency is the channel, a concept provided by the standard library. You can think of it like a water channel—a river or stream. If you place something like a rubber duck or a boat in it, it flows downstream to the receiver.

A channel has two parts: a transmitter and a receiver. When either the transmitter or receiver is dropped, the channel is considered closed.

Channels are implemented through the standard library's std::sync::mpsc, which stands for multiple producer, single consumer.





- Channels (mpsc)
- Crossbeam Channels
- Actor Model with Actix


### Async Programming in Rust

- Futures: The core abstraction for asynchronous programming in Rust. A future represents a value that may not be available yet.
- Async/Await Syntax: Rust provides async and await keywords to simplify writing asynchronous code.


Futures in Rust represent a value that may not be immediately available but will be computed at some point in the future. The async/await syntax simplifies working with these futures, making asynchronous programming in Rust more intuitive.

Futures:
- A future is an abstraction that allows you to work with values that are not yet available.
- It can be thought of as a placeholder for a value that will be computed later.

Async/Await:
- The async keyword is used to define an asynchronous function, which returns a future.
- The await keyword is used to pause the execution of an async function until the future is resolved.


#### Tokio Runtime

Tokio is an asynchronous runtime for Rust that provides the necessary tools to write non-blocking applications. It is built on top of the futures library and is designed to work seamlessly with async/await syntax.

Key Features:
- Event Loop: Tokio uses an event loop to manage asynchronous tasks efficiently.
- Task Scheduling: It schedules tasks to run concurrently, allowing for high throughput.
- Timers and I/O: Provides utilities for working with timers and asynchronous I/O operations.



