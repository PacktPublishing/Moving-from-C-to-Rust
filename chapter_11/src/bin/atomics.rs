use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    println!("Atomic increment with fetch_add:");
    let atomic = AtomicUsize::new(0);
    std::thread::scope(|s| {
        s.spawn(|| atomic.fetch_add(1, Ordering::SeqCst));
        s.spawn(|| atomic.fetch_add(1, Ordering::SeqCst));
        s.spawn(|| atomic.fetch_add(1, Ordering::SeqCst));
    });
    println!("Final value: {:?}", atomic);

    println!("\nLock-free increment with CAS:");
    let atomic = AtomicUsize::new(0);

    std::thread::scope(|s| {
        s.spawn(|| loop {
            let expected = atomic.load(Ordering::SeqCst);
            let new = expected + 1;
            if atomic
                .compare_exchange_weak(expected, new, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                break;
            }
        });

        s.spawn(|| loop {
            let expected = atomic.load(Ordering::SeqCst);
            let new = expected + 1;
            if atomic
                .compare_exchange_weak(expected, new, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                break;
            }
        });
    });

    println!("Final value after CAS: {:?}", atomic);
}
