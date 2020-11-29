static mut COUNT: usize = 0;
const QUEEN: usize = 16;

fn main() {
    let mut qlist: [i32;QUEEN] = [0;QUEEN];
    for i in 0..QUEEN { qlist[i] = i as i32; }

    n_queens(&mut qlist, 0);
}

fn n_queens(result: &mut [i32], split: usize) {
    if split == QUEEN {
        unsafe {
            COUNT += 1;
            println!("{:?} {}", result, COUNT);
        }
        return;
    }

    let mut i = split;
    while i < QUEEN {
        let mut res_cp: [i32;QUEEN] = [0;QUEEN];

        let mut k = 0;
        while k < QUEEN {
            res_cp[k] = result[k];
            k += 1;
        }

        let mut is_ok = true;

        let mut j = 0;
        while j < split {
            let d_x: i32 = split as i32 - j as i32;
            let d_y: i32 = res_cp[i] - res_cp[j];
            if d_x == d_y || d_x + d_y == 0 {
                is_ok = false;
                break;
            }
            j += 1;
        }

        if is_ok {
            res_cp.swap(split, i);
            n_queens(&mut res_cp, split + 1);
        }

        i += 1;
    }
}
