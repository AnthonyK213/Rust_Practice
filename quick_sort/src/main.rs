fn main() {
    let sort_list = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("{:?}", quick_sort(&sort_list));
}

fn quick_sort<T: Ord + Copy>(set: &Vec<T>) -> Vec<T> {
    match set.len() {
        0 | 1 => set.to_vec(),
        _ => {
            let mut l_side: Vec<T> = vec![];
            let mut r_side: Vec<T> = vec![];
            let m: &T = &set[0];
            for i in 1..set.len() {
                if &set[i] <= m {
                    l_side.push(set[i]);
                } else {
                    r_side.push(set[i]);
                }
            }
            [quick_sort(&l_side), quick_sort(&r_side)].join(m)
        }
    }
}
