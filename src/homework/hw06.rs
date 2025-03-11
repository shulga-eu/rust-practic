#[test]
fn main() {
    let num_triangles = 5;

    (1..=num_triangles).for_each(|i| {
        (1..=i).for_each(|row| {
            // Відступи
            let spaces = " ".repeat(num_triangles * 2 - row);
            let stars = "*".repeat(2 * row - 1);
            println!("{}{}", spaces, stars);
        });
    });
}


