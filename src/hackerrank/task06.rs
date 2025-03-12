fn staircase(n: i32) {
    for i in 1..= n {
        println!("{:width$}{}", "", "#". repeat( i as usize), width = (n-i) as usize)
    }
}
