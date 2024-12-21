use crate::common::matrix::Matrix;

#[derive(Clone)]
pub struct PheromoneNetwork {
    tau_min: f64,
    tau_max: f64,
    pub pheromone_matrix: Matrix<f64>,
}

impl PheromoneNetwork {
    pub fn new(dimension: (usize, usize)) -> Self {
        Self {
            tau_min: Default::default(),
            tau_max: Default::default(),
            pheromone_matrix: Matrix::new(dimension, Default::default()),
        }
    }

    pub fn set_tau_max(&mut self, rho: f64, cost: f64) {
        self.tau_max = 1.0 / (cost * (1.0 - rho));
    }

    pub fn set_tau_min(
        &mut self,
        p_best: f64,
        dimension: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.tau_max == Default::default() {
            return Err("Set tau max before setting tau min".into());
        }
        let n = dimension as f64;
        let avg = n / 2.0;
        let p = f64::powf(p_best, 1.0 / n);
        self.tau_min = f64::min(self.tau_max, self.tau_max * (1.0 - p) / ((avg - 1.0) * p));
        Ok(())
    }

    pub fn evaporate_from_all(&mut self, rho: f64) {
        for pheromone_trail in self.pheromone_matrix.data_mut().iter_mut() {
            *pheromone_trail = f64::max(self.tau_min, *pheromone_trail * rho);
        }
    }

    pub fn deposit(&mut self, from: usize, to: usize, heuristic_cost: f64, symmetric: bool) {
        let edge = &mut self.pheromone_matrix[(from, to)];
        *edge = f64::min(self.tau_max, *edge + heuristic_cost);
        if symmetric {
            self.pheromone_matrix[(to, from)] = *edge;
        }
    }
}
