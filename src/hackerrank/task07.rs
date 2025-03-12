fn miniMaxSum(arr: &[i32]) {
    let total_sum: i64 = arr.iter().map(|&x| x as i64).sum();
    let min_sum = total_sum - *arr.iter().max().unwrap() as i64;
    let max_sum = total_sum - *arr.iter().min().unwrap() as i64;

    println!("{} {}", min_sum, max_sum);
}