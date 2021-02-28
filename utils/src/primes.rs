use std::collections::HashMap;

fn make_primes(mut n: usize) -> HashMap<usize, usize> {
    let mut primes = HashMap::new();
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            n /= i;
            if primes.contains_key(&i) {
                let x = primes.get_mut(&i).unwrap();
                *x += 1;
            } else {
                primes.insert(i, 1);
            }
        }
        i += 1;
    }

    if n > 1 {
        if primes.contains_key(&n) {
            let x = primes.get_mut(&n).unwrap();
            *x += 1;
        } else {
            primes.insert(n, 1);
        }
    }

    return primes;
}
