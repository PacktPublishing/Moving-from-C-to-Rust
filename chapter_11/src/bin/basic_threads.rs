fn main() {
    println!("1. Hello world from main!");
    std::thread::spawn(|| {
        println!("2. Hello world from thread!");
    });
    println!("3. Hello world from main!");

    std::thread::sleep(std::time::Duration::from_millis(100));

    println!("\nWith JoinHandle:");
    println!("1. Hello world from main!");
    let handle = std::thread::spawn(|| {
        println!("2. Hello world from thread!");
    });
    handle.join().unwrap();
    println!("3. Hello world from main!");
}
