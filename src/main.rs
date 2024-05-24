use std::io;

fn main() {
    let mut input = String::new();
    let mut fib: Vec<i32> = Vec::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: usize = input.trim().parse().expect("Enter a Number!");

    for i in 0..n {
        if i < 2 {
            fib.push(1);
        } else {
            let next_value = fib[i - 1] + fib[i - 2];
            fib.push(next_value);
        }
    }

    if let Some(num) = fib.pop() {
        println!("{num}");
    }
}