fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let max_height =*candles.iter().max().unwrap();
    candles.iter().filter(|&&c| c == max_height).count() as i32
}