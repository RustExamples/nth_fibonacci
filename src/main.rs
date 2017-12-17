use std::io;

fn main() {
    println!("What's the nth Fibonacci?");

    loop {
        let mut n = String::new();

        println!("Enter n:");

        io::stdin().read_line(&mut n).expect("Error occured reading.");

        let n: i32 = match n.trim().parse() {
            Ok(num) => {
                if num < 0 {
                    continue;
                }
                else {
                    num
                }
            },
            Err(_) => continue
        };

        let result = nth_fibo(n);

        println!("{}th Fibonacci is {}", n, result);
    }
}

fn nth_fibo(num: i32) -> i32 {
    if num <= 0 {
        1
    }
    else{
        nth_fibo(num - 1) + nth_fibo(num - 2)
    } 
}