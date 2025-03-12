fn plusMinus(arr: &[i32]) {
    let mut positive = 0;
    let mut negative = 0;
    let mut zero = 0;

    for &num in arr {
        if num > 0 {
            positive += 1;
        } else if num <0 {
            negative += 1;
        } else {
            zero += 1;
        }
    }

    let total = arr.len() as f64;

    let positive_ratio = positive as f64 / total;
    let negative_ratio = negative as f64 / total;
    let zero_ratio = zero as f64 / total;

    println!("{:.6}", positive_ratio);
    println!("{:.6}", negative_ratio);
    println!("{:.6}", zero_ratio);
}