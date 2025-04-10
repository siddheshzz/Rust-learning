mod prelude;
mod simple_threads;
mod simple_mutex;
mod simple_atomic_types;
fn main() {
    simple_threads::simpleThreads();
    simple_mutex::simple_mutex();
    simple_atomic_types::simple_atomic_types();

}
