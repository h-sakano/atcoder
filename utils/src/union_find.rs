pub struct UnionFind {
    pub parent: Vec<usize>,
    pub rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let mut parent = vec![];
        for x in 0..n {
            parent.push(x);
        }
        UnionFind {
            parent: parent,
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.parent[x] = self.find(self.parent[x]);
            self.parent[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
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
