fn quick_sort(vct: &mut Vec<i32>) -> Vec<i32> {
    if vct.len() >= 2 {
        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();
        let mut res: Vec<i32> = Vec::new();

        let mid: i32 = vct[vct.len()>>1];
        vct.remove(vct.len()>>1);

        for num in vct.iter() {
            if num >= &mid {
                right.push(*num);
            } else {
                left.push(*num);
            }
        }

        res.append(&mut quick_sort(&mut left));
        res.push(mid);
        res.append(&mut quick_sort(&mut right));
        return res;
    } else {
        return vct.to_vec();
    }
}

fn main() {
    let mut vct: Vec<i32> = vec![2, 3, 5, 7, 1, 4, 6, 10, 0, 13, 15, 5, 2, 7, 9, 10, 15, 9, 17, 12];
    println!("{:?}", quick_sort(&mut vct));
}
