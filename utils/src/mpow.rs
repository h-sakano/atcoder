// 冪剰余, べき剰余

pub fn mpow(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut result = 1;

    while e > 0 {
        if e & 1 == 1 {
            result = result * b % m;
        }
        e >>= 1;
        b = (b * b) % m;
    }

    result
}
