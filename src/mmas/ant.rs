#[derive(Clone)]
pub struct Ant {
    visited: Vec<usize>,
    is_visited: Vec<bool>,
    pub cost: f64,
}

impl Ant {
    pub fn new(visited: Vec<usize>, is_visited: Vec<bool>, cost: f64) -> Self {
        Self {
            visited,
            is_visited,
            cost,
        }
    }

    pub fn visited(&self) -> &Vec<usize> {
        &self.visited
    }

    pub fn initialize(&mut self, dimension: usize) {
        self.visited.clear();
        self.visited.reserve(dimension);
        self.is_visited.clear();
        self.is_visited.reserve(dimension);
    }

    pub fn visit(&mut self, node: usize) {
        assert!(!self.is_visited[node]);
        self.visited.push(node);
        self.is_visited[node] = true;
    }

    pub fn is_visited(&self, node: usize) -> bool {
        self.is_visited[node]
    }

    pub fn all_visited(&self) -> bool {
        !self.is_visited.contains(&false)
    }
}

impl Default for Ant {
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), f64::MAX)
    }
}
