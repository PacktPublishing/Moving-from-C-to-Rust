fn main() {
    println!("Using move closure:");
    let message = String::from("2. Hello world from thread!");
    let handle = std::thread::spawn(move || {
        println!("{}", message);
    });
    handle.join().unwrap();
    println!("3. Hello world from main!");
}
