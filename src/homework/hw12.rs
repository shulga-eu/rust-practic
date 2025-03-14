// Функція написана за допомогою AI тому що не можу розібратися, але все одно не працює
fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total_weight: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total_weight % n != 0 {
        return 0;
    }

    let target_weight = total_weight / n;
    let mut moves = 0;
    let mut balance = 0;

    for &weight in shipments.iter() {
        balance += weight as i32 - target_weight as i32;
        moves += balance.abs() as usize;
        println!("weight: {}, target: {}, balance: {}, moves: {}", weight, target_weight, balance, moves);
    }

    println!("Final moves: {}", moves);
    moves
}



fn can_balance(shipments: &Vec<u32>) -> bool {
    let total_weight: u32 = shipments.iter().sum();
    let n = shipments.len();
    total_weight % n as u32 == 0
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let total_weight: u32 = n as u32 * 5;
    vec![total_weight / n as u32; n]
}

#[test]
fn test_count_permutation() {
    let shipments = vec![1, 1, 1, 1, 6];
    assert_eq!(count_permutation(&shipments), 10);

    let shipments = vec![8, 2, 2, 4, 4];
    assert_eq!(count_permutation(&shipments), 4);

    let shipments = vec![9, 3, 7, 2, 9];
    assert_eq!(count_permutation(&shipments), 7);
}


#[test]
fn test_can_balance() {
    assert_eq!(can_balance(&vec![1, 1, 1, 1, 6]), true);
    assert_eq!(can_balance(&vec![3, 5, 8]), false);
}

#[test]
fn test_gen_shipments() {
    let shipments = gen_shipments(4);
    assert!(can_balance(&shipments));
}

