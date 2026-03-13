fn main() {
    println!("Basic scoped threads:");
    std::thread::scope(|s| {
        s.spawn(|| {
            println!("Hello from thread one!");
        });
        s.spawn(|| {
            println!("Hello from thread two!");
        });
    });

    println!("\nSharing data with scoped threads:");
    let message = String::from("Shared message");
    std::thread::scope(|s| {
        s.spawn(|| {
            println!("Hello from thread one: {}", message);
        });
        s.spawn(|| {
            println!("Hello from thread two: {}", message);
        });
    });
}
