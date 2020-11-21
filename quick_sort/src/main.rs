fn main() {
    let sort_int: Vec<u32>  = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let sort_flt: Vec<f32>  = vec![3.14, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
    let sort_str: Vec<char> = "The five boxing wizards jump quickly".to_lowercase().chars().collect();
    println!("{:?}", quick_sort(&sort_int));
    println!("{:?}", quick_sort(&sort_flt));
    println!("{:?}", quick_sort(&sort_str));
}

fn quick_sort<T>(set: &Vec<T>) -> Vec<T>
    where T: PartialOrd + Copy {
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
