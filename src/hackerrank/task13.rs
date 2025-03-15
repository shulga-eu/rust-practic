use std::cmp::{min, max};
use num::integer::{gcd, lcm};
fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let lcm_a = a.iter().copied().reduce(lcm).unwrap();
    let gcd_b = b.iter().copied().reduce(gcd).unwrap();

    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}
