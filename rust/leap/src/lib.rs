pub fn is_leap_year(year: u64) -> bool {
    let l4 = year % 4 == 0;
    let nl100 = year % 100 != 0;
    let l400 = year % 400 == 0;

    match (l4, nl100, l400) {
        (true, true, _) => true,
        (true, false, true) => true,
        _ => false
    }

}
