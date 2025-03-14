// Сподіваюсь можна було використовувати зовнішні бібліотеки
use itertools::Itertools;

#[test]
fn solve_muxa_slon() {
    let digits: Vec<u32> = (1..=8).collect();
    let mut count_solutions = 0;

    for perm in digits.into_iter().permutations(8) {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];

        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        if muxa * a == slon {
            count_solutions += 1;
            println!("{}{}{}{} × {} = {}{}{}{}", m, u, x, a, a, s, l, o, n);
        }
    }

    println!("Кількість розв'язків: {}", count_solutions);
}
