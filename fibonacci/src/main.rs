use std::io;

/*
macro_rules! numin {
    () => {
        {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input.trim().parse().unwrap()
        }
    };
}*/

fn fibonacci(n: u64) -> u64 {
    let mut fib_0: u64 = 0;
    let mut fib_1: u64 = 1;
    let mut i = 0;
    while i < n-1 {
        let tmp: u64 = fib_0;
        fib_0 = fib_1;
        fib_1+= tmp;
        i += 1;
    }
    return fib_1;
}

/*
fn fibonacci2(n: u64) -> u64 {
    //let n: u64 = n - 1;
    match n {
        1 => 1,
        2 => 1,
        _ => fibonacci2(n - 1) + fibonacci2(n - 2),
    }
}*/

fn main() {
    //let num: u64 = numin!();
    /*
    loop {
        let mut n = String::new();
        println!("\nEnter a positive integer:");
        io::stdin().read_line(&mut n).expect("Failed to read line");
        if n.trim() == "quit" {
            break;
        }
        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("{}", fibonacci(n));
    }*/

    let mut n = String::new();
    println!("\nEnter a positive integer:");
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u64 = n.trim().parse::<u64>().unwrap();
    println!("{}", fibonacci(n));
}

/*
fn add(a: u32, b: u32) -> u32 {
    return a + b
}*/

