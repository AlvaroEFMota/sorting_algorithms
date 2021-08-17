use sorting_algorithms::*;

fn main() {
    let v = vec![2, 3, 1];
    Bubblesort::sort(&mut v);
    println!("v: {:?}", v);
}