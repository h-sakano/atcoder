#![allow(non_snake_case)]

// 冪剰余
pub fn mpow(mut b: usize, mut e: usize, m: usize) -> usize {
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

// 組み合わせ
struct Combination {
    m: usize,
    size: usize,
    fact: Vec<usize>,
    fact_inv: Vec<usize>,
}

impl Combination {
    fn new(size: usize, m: usize) -> Self {
        let mut fact = vec![1; size + 1];
        let mut inv = vec![1; size + 1];
        let mut fact_inv = vec![1; size + 1];
        for i in 2..size {
            fact[i] = fact[i - 1] % m;
            inv[i] = m - inv[m % i] * (m / i) % m;
            fact_inv[i] = fact_inv[i - 1] * inv[i] % m;
        }
        Combination {
            m,
            size,
            fact,
            fact_inv,
        }
    }

    fn nPk(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        self.fact[n] * self.fact_inv[n - k] % self.m
    }

    fn nCk(&self, n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }

        self.fact[n] * (self.fact_inv[k] * self.fact_inv[n - k] % self.m) % self.m
    }
}
