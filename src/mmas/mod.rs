use crate::{common::matrix::Matrix, tsplib::TspLibInstance};

pub struct MMASParameters {
    /// pheromone trail evaporation
    rho: f64,
    /// number of ants that will explore the search space
    colony_size: usize,
    /// pheromone trail exploitation parameter
    alpha: f64,
    /// heuristic information exploitation parameter
    beta: f64,
    /// probability of moving to the best solution found so far
    p_best: f64,
    // candidate list size
    nn_bounded_length: usize,
}

impl MMASParameters {
    pub fn new(
        rho: f64,
        colony_size: usize,
        alpha: f64,
        beta: f64,
        p_best: f64,
        nn_bounded_length: usize,
    ) -> Self {
        Self {
            rho,
            colony_size,
            alpha,
            beta,
            p_best,
            nn_bounded_length,
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct MMAS {
    /// TSPLIB instance
    instance: TspLibInstance,
    /// MMAS parameters
    parameters: MMASParameters,
    /// nearest neighbor matrix
    nn: Matrix<usize>,
}

impl MMAS {
    pub fn new(instance: TspLibInstance, parameters: MMASParameters) -> Self {
        let nn = Matrix::new(
            (instance.dimension, parameters.nn_bounded_length),
            usize::default(),
        );
        Self {
            instance,
            parameters,
            nn,
        }
    }

    /// initializes nearest neighbor martix (per node)
    pub fn initialize_nn_matrix(&mut self) {
        for node in 0..self.instance.dimension {
            let mut nn_row: Vec<usize> = (0..self.instance.dimension).collect();
            nn_row.sort_by(|&a, &b| {
                self.instance.distance_matrix[(node, a)]
                    .partial_cmp(&self.instance.distance_matrix[(node, b)])
                    .unwrap()
            });
            assert!(
                self.instance.distance_matrix[(node, nn_row[0])]
                    <= self.instance.distance_matrix[(node, nn_row[1])]
            );
            let mut added_nn = 0;
            let mut i = 0;
            while added_nn < self.parameters.nn_bounded_length {
                if nn_row[i] != node {
                    self.nn[(node, added_nn)] = nn_row[i];
                    added_nn += 1;
                }
                i += 1;
            }
        }
    }
}
