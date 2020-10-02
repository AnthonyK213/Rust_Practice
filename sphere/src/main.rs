fn main() {
    sphere();
}

fn sphere() {
    let mut y: f64 = 1.0;
    while y >= -1.05 {
        print!("\n");
        let mut x: f64 = -1.0;
        while x <= 1.025 {
            let outchar = if x * x + y * y > 1.0 {" "} else {["@", "%", "#", "*", "+", "=", ";", ":", "-", "·", " "][(((x + y + (1.0 - (x * x + y * y)).powf(0.5)) * 0.5773502692 + 1.0) * 5.0 + 0.5) as usize]};
            print!("{}", outchar);
            x += 0.025;
        }
        y -= 0.05;
    }
}

