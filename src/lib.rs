pub mod common;
pub mod mmas;
pub mod tsplib;

pub trait TspSolver {
    fn solve(&self);
}
