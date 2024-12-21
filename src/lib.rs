pub mod common;
pub mod mmas;
pub mod tsplib;

pub struct Statistics<S> {
    /// Current iteration
    iteration: Option<usize>,
    /// Solution quality indicator
    cost: Option<f64>,
    /// Domain-specific construct
    meta_inf: Option<S>,
}

impl<S> Statistics<S> {
    pub fn new(iteration: Option<usize>, cost: Option<f64>, meta_inf: Option<S>) -> Self {
        Self {
            iteration,
            cost,
            meta_inf,
        }
    }

    pub fn iteration(&self) -> Option<usize> {
        self.iteration
    }

    pub fn cost(&self) -> Option<f64> {
        self.cost
    }

    pub fn mime(&self) -> Option<&S> {
        self.meta_inf.as_ref()
    }
}

impl<S> Default for Statistics<S> {
    fn default() -> Self {
        Self {
            iteration: Default::default(),
            cost: Default::default(),
            meta_inf: Default::default(),
        }
    }
}

pub trait TspSolver {
    type Solution;
    type MetaInf;
    fn solve<F>(&self, break_condition: F) -> Result<Self::Solution, Box<dyn std::error::Error>>
    where
        F: Fn(&Statistics<Self::MetaInf>) -> bool;
}

pub enum Position {
    Origin,
    Random,
}
