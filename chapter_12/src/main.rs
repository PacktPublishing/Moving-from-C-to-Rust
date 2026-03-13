macro_rules! v {
  () => { ::std::vec::Vec::new() };
  ($val:expr; $count:expr) =>
    { ::std::vec::Vec::from_iter(::std::iter::repeat_n($val, $count)) };
  ($($x:expr),+) => { ::std::vec::Vec::from([$($x),+]) };
}

macro_rules! zip {
    (($($l:expr),*), ($($r:expr),*)) => {
        ($( ($l,$r) ),*)
    };
}

macro_rules! double_x {
    ($x:ident) => {
        $x = double($x)
    };
}

fn double(x: i32) -> i32 {
    return x * 2;
}

fn main() {
    let _v: Vec<u32> = v![];
    let v: Vec<u32> = v![0];
    println!("{:?}", v);
    let v: Vec<u32> = v![0, 1, 2, 3];
    println!("{:?}", v);
    let v = v![0u32; 5];
    println!("{:?}", v);

    let out = zip!((0, 1, 2), (3, 4, 5));
    println!("{:?}", out);

    let mut x = 1;
    double_x!(x);
    println!("{:?}", x);

    use prettyprint_derive::PrettyPrint;

    #[derive(PrettyPrint)]
    struct TestStructure(u32, u32, u32);

    let test = TestStructure(0, 1, 2);
    test.pretty_print();

    square(5);
}

#[log_call::log_call]
fn square(x: u32) -> u32 {
    x * x
}
