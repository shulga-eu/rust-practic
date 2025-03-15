fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let mut pairs = 0;
    let mut checked = vec![false; n as usize];

    for i in 0..n as usize {
        if checked[i] {
            continue;
        }

        let mut count = 1;

        for j in (i + 1)..n as usize {
            if ar[i] == ar[j] {
                count += 1;
                checked[j] = true;
            }
        }

        pairs += count / 2;
    }

    pairs
}