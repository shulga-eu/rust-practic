fn pageCount(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = (n / 2) - ( p / 2 );
    from_front.min(from_back)
}
