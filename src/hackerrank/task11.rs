fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let mut apple_count = 0;
    for delta in apples {
        let absolute = a + delta;
        if absolute >= s && absolute <= t {
            apple_count += 1;
        }
    }

    let mut orange_count = 0;
    for delta in oranges {
        let absolute = b + delta;
        if absolute >= s && absolute <= t {
            orange_count += 1;
        }
    }

    println!("{}\n{}", apple_count, orange_count)
}
