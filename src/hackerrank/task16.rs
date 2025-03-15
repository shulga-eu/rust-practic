fn divisibleSumPairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..n as usize {
        for j in i + 1..n as usize {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }
    count
}
