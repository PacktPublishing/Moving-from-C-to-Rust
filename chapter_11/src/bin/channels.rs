fn main() {
    println!("Channel with iterator:");
    let (tx, rx) = std::sync::mpsc::channel();

    for i in 0..10 {
        let tx = tx.clone();
        std::thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }

    drop(tx);

    for i in rx.iter() {
        println!("Received: {}", i);
    }

    println!("\nChannel with recv():");
    let (tx, rx) = std::sync::mpsc::channel();

    for i in 0..10 {
        let tx = tx.clone();
        std::thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }

    for _ in 0..10 {
        println!("Received: {}", rx.recv().unwrap());
    }
}
