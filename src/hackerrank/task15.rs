fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut count = 0;
    for i in 0..=s.len() - m as usize {
        if s[i..i + m as usize].iter().sum::<i32>() == d {
            count += 1;
        }
    }
    count
}