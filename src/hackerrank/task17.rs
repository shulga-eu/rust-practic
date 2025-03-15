fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6];

    for &bird in arr {
        counts[bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut min_id = 0;

    for i in 1..=5 {
        if counts[i] > max_count {
            max_count = counts[i];
            min_id = i as i32;
        }
    }

    min_id
}