fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    let total_cost: i32 = bill.iter().sum();
    let anna_share = (total_cost - bill[k as usize]) / 2;

    if b == anna_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b - anna_share);
    }
}