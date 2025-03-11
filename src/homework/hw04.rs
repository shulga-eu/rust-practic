# [test]
fn envelope_test4() {
    const W: u32 = 15;
    const H: u32 = 15;

    let mid = W / 2;

    for y in 0..H {
        for x in 0..W {
            let sym: char = if y <= H / 2 {
                if x >= mid - y && x <= mid + y { '*' } else { ' ' }
            } else {
                if x >= (y - H / 2) && x <= (W - (y - H / 2) - 1) { '*' } else { ' ' }
            };
            print!("{}", sym);
        }
        println!();
    }
}


