fn main() {
    let message = String::from("Shared message");

    println!("Using rayon scoped threads:");
    rayon::scope(|s| {
        s.spawn(|_| {
            println!("Hello from thread one: {}", message);
        });
        s.spawn(|_| {
            println!("Hello from thread two: {}", message);
        });
    });

    println!("\nNote: rayon uses work-stealing scheduling automatically");
}
