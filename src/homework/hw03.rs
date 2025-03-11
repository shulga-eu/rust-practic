#[test]
fn envelope_test2() {
    const W: u32 = 25;
    const H: u32 = 10;
    let k = W as f32 / H as f32;

    let yk = |y: u32| (y as f32 *k) as u32;

    for y in 1..=H{
        for x in 1..=W{
            let sym: char = match (x, y) {
                (_, 1) => '*',
                (_, H) => '*',
                (1, _) => '*',
                (W, _) => '*',
                _ if x ==yk(y) => '*',
                _ if x == W - yk(y) => '*',
                _ => ' ',
            };
            print!("{}", sym);
        }
        println!();
    }
}