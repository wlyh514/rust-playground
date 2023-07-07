pub mod chapters;

fn main() {
    println!("Hello, world!");

    dbg!("dbg! takes the ownership and prints to stderr\n");
    println!("println! takes a reference and prints to stdout");

    // chapters::types::types();
    // chapters::types::tuples();
    // chapters::types::arrays();

    // chapters::ownership::ownership();

    // chapters::refs::reference_and_borrowing();
    // chapters::refs::slices();
    // chapters::refs::pass_by_ref();

    // chapters::structs::structs();

    // chapters::enums::enums();
    
    // chapters::collections::vector();
    // chapters::collections::string();
    // chapters::collections::hashmap();

    // chapters::lifetime::lifetime();

    // chapters::smart_pointers::boxes();
    // chapters::smart_pointers::my_box();
    // chapters::smart_pointers::my_drop();
    // chapters::smart_pointers::rc();
    // chapters::smart_pointers::ref_cycles();

    // chapters::concurrency::spawn_threads();
    // chapters::concurrency::thread_move();
    // chapters::concurrency::channel();
    // chapters::concurrency::multi_producer();
    // chapters::concurrency::counter_using_mutex();
    chapters::concurrency::deadlock();
}