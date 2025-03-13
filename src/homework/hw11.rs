use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    (0..n).map(|_| rng.random_range(10..99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_sum, min_index, min_index + 1)
}

fn print_vector_with_min(data: &[i32], min_index: usize) {
    // Індекси
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:3} ", i);
    }
    println!();

    // Значення масиву
    print!("data:    ");
    for num in data {
        print!("{:3} ", num);
    }
    println!();

    print!("indexes: ");
    for i in 0..data.len() {
        if i == min_index {
            print!("\\__ ");
        } else if i == min_index + 1 {
            print!("__/ ");
        } else {
            print!("    ");
        }
    }
    println!();
}

fn main() {
    let data = gen_random_vector(20);
    let (min_sum, idx1, idx2) = min_adjacent_sum(&data);

    print_vector_with_min(&data, idx1);

    println!(
        "min adjacent sum={}+{}={} at indexes:{}:{}",
        data[idx1], data[idx2], min_sum, idx1, idx2
    );
}
