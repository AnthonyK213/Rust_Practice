static mut COUNT: usize = 0;

fn main() {
    let queen: usize = 8;

    let mut qlist: Vec<i32> = Vec::new();
    for i in 0..queen {
        qlist.push(i as i32);
    }

    n_queens(&mut qlist, 0, &queen);
}

fn n_queens(result: &mut Vec<i32>, split: usize, queen: &usize) {
    if split == *queen {
        unsafe {
            COUNT += 1;
            println!("{:?} {}", result, COUNT);
        }
        return;
    }

    for i in split..*queen {
        let mut res_cp = result.to_vec();
        let mut is_ok = true;

        for j in 0..split {
            if (res_cp[i] - res_cp[j]).abs() == (split as i32 - j as i32).abs() {
                is_ok = false;
                break;
            }
        }

        if is_ok {
            res_cp.swap(split, i);
            n_queens(&mut res_cp, split + 1, &queen);
        }
    }
}
