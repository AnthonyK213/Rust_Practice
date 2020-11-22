fn main() {
    let sort_int: Vec<u32>  = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let sort_flt: Vec<f32>  = vec![3.14, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
    let sort_str: Vec<char> = "The five boxing wizards jump quickly".to_lowercase().chars().collect();
    println!("quick_sort<int>: {:?}", quick_sort(&sort_int));
    println!("quick_sort<flt>: {:?}", quick_sort(&sort_flt));
    println!("quick_sort<str>: {:?}", quick_sort(&sort_str));

    let mut self_sort_int: [u32;8] = [3, 1, 4, 1, 5, 9, 2, 6];
    self_quick_sort(&mut self_sort_int, 0, 8);
    println!("self_quick_sort<int>: {:?}", &self_sort_int);
}

fn quick_sort<T>(arr: &Vec<T>) -> Vec<T>
    where T: PartialOrd + Copy {
    match arr.len() {
        0 | 1 => arr.to_vec(),
        _     => {
            let mut l_side: Vec<T> = vec![];
            let mut r_side: Vec<T> = vec![];
            let pivot: &T = &arr[0];

            for i in 1..arr.len() {
                if &arr[i] <= pivot {
                    l_side.push(arr[i]);
                } else {
                    r_side.push(arr[i]);
                }
            }

            [quick_sort(&l_side), quick_sort(&r_side)].join(pivot)
        }
    }
}

fn self_quick_sort<T: PartialOrd + Copy>(arr: &mut [T], start: usize, end: usize) {
    if start == end { return; }

    let pivot = arr[start];
    let mut i = start;
    let mut j = i + 1;

    while j < end {
        if arr[j] <= pivot {
            arr.swap(i + 1, j);
            i += 1;
        }
        j += 1;
    }

    if i != start { arr.swap(start, i); }

    self_quick_sort(arr, start, i);
    self_quick_sort(arr, i + 1, end);
}
