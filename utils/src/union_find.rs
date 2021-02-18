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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(1), 1);
        assert_eq!(uf.find(2), 2);
        assert_eq!(uf.find(3), 3);
        assert_eq!(uf.find(4), 4);
    }

    #[test]
    fn test_unite() {
        let mut uf = UnionFind::new(5);
        uf.unite(0, 1);
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(1), 0);
        assert_eq!(uf.find(2), 2);
        uf.unite(1, 2);
        assert_eq!(uf.find(2), 0);
        assert_eq!(uf.same(0, 2), true);
    }
}
