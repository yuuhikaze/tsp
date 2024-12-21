mod ant;
mod pheromone;

use ant::Ant;
use pheromone::PheromoneNetwork;

use crate::{
    Position, Statistics, TspSolver,
    common::{
        matrix::Matrix,
        probability::{get_random_f64, get_random_usize},
    },
    tsplib::TspLibInstance,
};

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

    pub fn colony_size(&self) -> usize {
        self.colony_size
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
            (instance.dimension(), parameters.nn_bounded_length),
            Default::default(),
        );
        Self {
            instance,
            parameters,
            nn,
        }
    }

    /// initializes nearest neighbor martix (per node)
    pub fn initialize_nn_matrix(&mut self) {
        for node in 0..self.instance.dimension() {
            let mut nn_row: Vec<usize> = (0..self.instance.dimension()).collect();
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
            while added_nn < self.nn.cols() {
                if nn_row[i] != node {
                    self.nn[(node, added_nn)] = nn_row[i];
                    added_nn += 1;
                }
                i += 1;
            }
        }
    }

    // creates solution based on the shortest distance (nearest neighbors)
    fn create_heuristic_solution(&self, position: Position) -> Ant {
        let mut ant = Ant::default();
        ant.initialize(self.instance.dimension());
        let mut current_node = match position {
            Position::Origin => 0,
            Position::Random => todo!(),
        };
        ant.visit(current_node);
        for _ in 1..self.instance.dimension() {
            let mut next_node = current_node;
            // go to unvisited nearest neighbor
            for &node in self.nn.row(current_node) {
                if !ant.is_visited(node) {
                    next_node = node;
                    break;
                }
            }
            // all nearest neighbors are visited
            // visit next node based on minimum distance
            if next_node == current_node {
                let mut min_distance = usize::MAX;
                for node in 0..self.instance.dimension() {
                    if !ant.is_visited(node) {
                        let distance = self.nn[(current_node, node)];
                        if distance < min_distance {
                            min_distance = distance;
                            next_node = node;
                        }
                    }
                }
            }
            assert!(next_node != current_node);
            ant.visit(next_node);
            current_node = next_node;
        }
        ant
    }

    fn move_ant(
        &self,
        ant: &mut Ant,
        heuristic_information: &Matrix<f64>,
        pheromone_nw: &PheromoneNetwork,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let current_node = ant.visited()[ant.visited().len() - 1];
        let mut nn_unvisited_row = vec![Default::default(); self.parameters.nn_bounded_length];
        let mut added_unvisited_nn = 0;
        for &node in self.nn.row(current_node) {
            if !ant.is_visited(node) {
                nn_unvisited_row[added_unvisited_nn] = node;
                added_unvisited_nn += 1;
            }
        }
        let mut chosen_node = current_node;
        if added_unvisited_nn > 0 {
            let mut normalization_factor =
                vec![Default::default(); self.parameters.nn_bounded_length];
            let mut total = 0.0;
            for i in 0..added_unvisited_nn {
                let node = nn_unvisited_row[i];
                let product = pheromone_nw.pheromone_matrix()?[(current_node, node)]
                    * heuristic_information[(current_node, node)];
                total += product;
                normalization_factor[i] = total;
            }
            chosen_node = nn_unvisited_row[added_unvisited_nn - 1];
            let r = get_random_f64(0.0, 1.0) * total;
            for i in 0..added_unvisited_nn {
                if r < normalization_factor[i] {
                    chosen_node = nn_unvisited_row[i];
                    break;
                }
            }
        } else {
            let mut max_product = 0.0;
            for node in 0..self.instance.dimension() {
                if !ant.is_visited(node) {
                    let product = pheromone_nw.pheromone_matrix()?[(current_node, node)]
                        * heuristic_information[(current_node, node)];
                    if product > max_product {
                        max_product = product;
                        chosen_node = node;
                    }
                }
            }
        }
        assert!(chosen_node != current_node);
        ant.visit(chosen_node);
        Ok(())
    }
}

impl TspSolver for MMAS {
    type Solution = Ant;
    type MetaInf = PheromoneNetwork;
    fn solve<F>(&self, break_condition: F) -> Result<Self::Solution, Box<dyn std::error::Error>>
    where
        F: Fn(&Statistics<Self::MetaInf>) -> bool,
    {
        let heuristic_solution = self.create_heuristic_solution(Position::Origin);
        let heuristic_cost = self
            .instance
            .calculate_route_length(heuristic_solution.visited());
        let mut pheromone_nw = PheromoneNetwork::default();
        let tau_max = pheromone::compute_tau_max(self.parameters.rho, heuristic_cost);
        let tau_min =
            pheromone::compute_tau_min(tau_max, self.parameters.p_best, self.instance.dimension());
        pheromone_nw.update_pheromone_bounds(tau_max, tau_min);
        pheromone_nw
            .initialize_pheromone_matrix((self.instance.dimension(), self.instance.dimension()))?;
        let mut heuristic_information =
            Matrix::with_capacity((self.instance.dimension(), self.instance.dimension()));
        for &distance in self.instance.distance_matrix.data() {
            heuristic_information
                .data_mut()
                .push(1.0 / f64::powf(distance, self.parameters.beta));
        }
        let mut ants: Vec<Ant> = vec![Ant::default(); self.parameters.colony_size()];
        let mut global_best = Ant::default();
        let mut iteration = 0;
        let mut statistics = Statistics::default();
        while !break_condition(&statistics) {
            for ant in ants.iter_mut() {
                ant.initialize(self.instance.dimension());
                let start_node = get_random_usize(0, self.instance.dimension() - 1);
                ant.visit(start_node);
                for _ in 1..self.instance.dimension() {
                    self.move_ant(ant, &heuristic_information, &pheromone_nw)?;
                }
                ant.cost = self.instance.calculate_route_length(ant.visited());
                statistics.cost = Some(ant.cost);
            }
            let mut iteration_best = &ants[0];
            let mut new_global_best_found = false;
            for ant in ants.iter() {
                if ant.cost < global_best.cost {
                    global_best = ant.clone();
                    new_global_best_found = true;
                    println!(
                        "Best solution found [{}] at iteration: {}",
                        global_best.cost, iteration
                    );
                }
                if ant.cost < iteration_best.cost {
                    iteration_best = ant;
                }
            }
            if new_global_best_found {
                let tau_max = pheromone::compute_tau_max(self.parameters.rho, global_best.cost);
                let tau_min = pheromone::compute_tau_min(
                    tau_max,
                    self.parameters.p_best,
                    self.instance.dimension(),
                );
                pheromone_nw.update_pheromone_bounds(tau_max, tau_min);
            }
            pheromone_nw.evaporate_from_all(self.parameters.rho)?;
            let heuristic_cost = 1.0 / iteration_best.cost;
            let mut previous_node = iteration_best.visited()[iteration_best.visited().len() - 1];
            for &node in iteration_best.visited() {
                pheromone_nw.deposit(
                    previous_node,
                    node,
                    heuristic_cost,
                    self.instance.symmetric(),
                )?;
                previous_node = node;
            }
            // statistics.meta_inf = ...
            statistics.iteration = Some(iteration);
            iteration += 1;
        }
        Ok(global_best)
    }
}
