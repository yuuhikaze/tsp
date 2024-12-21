use crate::common::matrix::Matrix;

#[derive(Default)]
pub struct PheromoneNetwork {
    tau_min: f64,
    tau_max: f64,
    pheromone_matrix: Option<Matrix<f64>>,
}

impl PheromoneNetwork {
    pub fn new(tau_min: f64, tau_max: f64, pheromone_matrix: Option<Matrix<f64>>) -> Self {
        Self {
            tau_min,
            tau_max,
            pheromone_matrix,
        }
    }

    pub fn update_pheromone_bounds(&mut self, tau_max: f64, tau_min: f64) {
        self.tau_max = tau_max;
        self.tau_min = tau_min;
    }

    pub fn initialize_pheromone_matrix(
        &mut self,
        dimension: (usize, usize),
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.tau_max == 0.0 {
            return Err("Update pheromone bounds first".into());
        } else if dimension.0 != dimension.1 {
            return Err("Dimension must be square".into());
        }
        self.pheromone_matrix = Some(Matrix::new(dimension, self.tau_max));
        Ok(())
    }

    pub fn evaporate_from_all(&mut self, rho: f64) -> Result<(), Box<dyn std::error::Error>> {
        for pheromone_trail in self
            .pheromone_matrix
            .as_mut()
            .ok_or("Pheromone matrix has not been initialized")?
            .data_mut()
            .iter_mut()
        {
            *pheromone_trail = f64::max(self.tau_min, *pheromone_trail * rho);
        }
        Ok(())
    }

    pub fn deposit(
        &mut self,
        from: usize,
        to: usize,
        heuristic_cost: f64,
        symmetric: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let edge = &mut self
            .pheromone_matrix
            .as_mut()
            .ok_or("Pheromone matrix has not been initialized")?[(from, to)];
        *edge = f64::min(self.tau_max, *edge + heuristic_cost);
        if symmetric {
            self.pheromone_matrix
                .as_mut()
                .ok_or("Pheromone matrix has not been initialized")?[(to, from)] = *edge;
        }
        Ok(())
    }

    pub fn pheromone_matrix(&self) -> Result<&Matrix<f64>, Box<dyn std::error::Error>> {
        let mat_ref = self
            .pheromone_matrix
            .as_ref()
            .ok_or("Pheromone matrix has not been initialized")?;
        Ok(mat_ref)
    }
}

pub fn compute_tau_max(rho: f64, cost: f64) -> f64 {
    1.0 / (cost * (1.0 - rho))
}

pub fn compute_tau_min(tau_max: f64, p_best: f64, nodes: usize) -> f64 {
    let n = nodes as f64;
    let avg = n / 2.0;
    let p = f64::powf(p_best, 1.0 / n);
    f64::min(tau_max, tau_max * (1.0 - p) / ((avg - 1.0) * p))
}
