fn timeConversion(s: &str) -> String {
    let hours: u32 = s[..2].parse().unwrap();
    let minutes_seconds = &s[2..8];
    let am_pm = &s[8..];

    let new_hours = if am_pm == "AM" {
        if hours == 12 { 0 } else { hours }
    } else { // PM
        if hours == 12 { 12 } else { hours + 12 }
    };

    format!("{:02}{}", new_hours, minutes_seconds)

}