#[test]
fn simpleArraySum(xs: &[i32]) -> i32 {
    let mut total = 0;

    for x in xs {
        total += x;
    }

    total

}
