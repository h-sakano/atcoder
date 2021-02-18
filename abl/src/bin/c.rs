use proconio::{fastout, input, marker::Usize1};

pub struct UnionFind {
    pub n: usize,
    pub parents: Vec<usize>,
    pub rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let mut parents = vec![];
        for x in 0..n {
            parents.push(x);
        }
        UnionFind {
            n: n,
            parents: parents,
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            self.parents[x] = self.find(self.parents[x]);
            self.parents[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.parents[x] = y;
        } else {
            self.parents[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abs: [(Usize1, Usize1); m],
    }

    let mut ans = n - 1;
    let mut uf = UnionFind::new(n);
    for (a, b) in abs {
        if !uf.same(a, b) {
            uf.unite(a, b);
            ans -= 1;
        }
    }

    println!("{}", ans);
}
