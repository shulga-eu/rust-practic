fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 > v2 && (x2 - x1) % (v1 - v2) == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}