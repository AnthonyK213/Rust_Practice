use std::io;

fn wc(n: usize) -> usize {
    match n {
        1 => 1,
        2 => 1,
        3 => 2,
        _ => {
            if n % 2 == 0 {
                return wc(n / 2) + wc(n / 2 + 1) - 1;
            } else {
                return 2 * wc((n + 1) / 2) - 1;
            }
        },
    }
}

fn main() {
    println!("Welcome to men's toilet! Input 'q' to quit.");
    loop {
        println!("n = ");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line.");

        if n.trim() == "q" {
            break;
        } else {
            let n: usize = n.trim().parse::<usize>().unwrap();
            let res = wc(n);
            println!("{} can be used;", res);
            println!("Utilization ratio is {:.3};", res as f64 / n as f64);
        }
    }
}
