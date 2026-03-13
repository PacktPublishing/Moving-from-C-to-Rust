use rayon::prelude::*;

fn main() {
    println!("Sequential:");
    let v = vec![0, 1, 2, 3, 4, 5];
    let doubled = v.iter().map(|x| x * 2).collect::<Vec<_>>();
    println!("{:?}", doubled);

    println!("\nParallel:");
    let v = vec![0, 1, 2, 3, 4, 5];
    let doubled = v.par_iter().map(|x| x * 2).collect::<Vec<_>>();
    println!("{:?}", doubled);

    println!("\nNote: Output order is preserved with par_iter()");
}
