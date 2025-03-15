fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    let mut min_record = scores[0];
    let mut max_record = scores[0];
    let mut min_count = 0;
    let mut max_count = 0;

    for &score in scores.iter().skip(1) {
        if score > max_record {
            max_record = score;
            max_count += 1;
        } else if score < min_record {
            min_record = score;
            min_count += 1;
        }
    }

    vec![max_count, min_count]
}