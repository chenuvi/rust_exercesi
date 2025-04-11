pub fn raindrops(n: u32) -> String {
    let is_pling = |n| n % 3 == 0;
    let is_plang = |n| n % 5 == 0;
    let is_plong = |n| n % 7 == 0;
    let mut res = String::new();
    if is_pling(n) {
        res.push_str("Pling");
    }
    if is_plang(n) {
        res.push_str("Plang");
    }
    if is_plong(n) {
        res.push_str("Plong");
    }
    if res.is_empty() {
        let s = format!("{}", n);
        res.push_str(&s);
    }
    res
}
