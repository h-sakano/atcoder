use proconio::{fastout, input};

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut parent = vec![];
        for x in 0..n {
            parent.push(x);
        }
        UnionFind {
            parent: parent,
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.parent[x] = self.find(self.parent[x]);
            self.parent[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
        } else {
            self.parent[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        friends: [(usize, usize); m],
    }

    let mut uf = UnionFind::new(n);
    for (a, b) in friends {
        let a = a - 1;
        let b = b - 1;
        uf.unite(a, b);
    }

    let mut groups = vec![0u64; n];
    for x in 0..n {
        let p = uf.find(x);
        groups[p] += 1;
    }

    let ans = groups.iter().max().copied().unwrap();
    println!("{}", ans)
}
