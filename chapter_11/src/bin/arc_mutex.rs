use std::sync::{Arc, Mutex};

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let owned_data = Arc::new(Mutex::new(v));

    let data = Arc::clone(&owned_data);
    let thread_a = std::thread::spawn(move || {
        let mut locked = data.lock().unwrap();
        locked.push(7);
    });

    let data = Arc::clone(&owned_data);
    let thread_b = std::thread::spawn(move || {
        let locked = data.lock().unwrap();
        println!("Vector contents: {:?}", *locked);
    });

    thread_a.join().unwrap();
    thread_b.join().unwrap();

    println!("Final vector: {:?}", owned_data.lock().unwrap());
}
