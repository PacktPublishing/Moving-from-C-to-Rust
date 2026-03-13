use elapsed::measure_time;
use performance_test::{make_random, matrix_multiply};
use std::env;

fn get_args() -> Vec<usize> {
    let mut ret = Vec::new();
    for arg in env::args().skip(1).take(3) {
        if let Ok(val) = arg.parse::<usize>() {
            ret.push(val);
        }
    }
    ret
}

fn main() {
    let args = get_args();
    if args.len() != 3 {
        eprintln!("Usage: m k n");
        return;
    }
    let (m, k, n) = (args[0], args[1], args[2]);

    // Simple timing with elapsed crate
    flame::start("initialization");
    let a = make_random(m, k);
    let b = make_random(k, n);
    flame::end("initialization");

    flame::start("multiply");
    let (elapsed, _c) = measure_time(|| matrix_multiply(a, b));
    flame::end("multiply");

    println!("Multiply took {}", elapsed);

    let f = std::fs::File::create("perf.html").unwrap();
    flame::dump_html(f).unwrap();
}
