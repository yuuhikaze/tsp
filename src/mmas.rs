pub struct MMASParameters {
    /// pheromone trail evaporation
    pub rho: f64,
    /// number of ants that will explore the search space
    pub colony_size: usize,
    /// pheromone trail exploitation parameter
    pub alpha: f64,
    /// heuristic information exploitation parameter
    pub beta: f64,
    /// probability of moving to the best solution found so far
    pub p_best: f64,
    /// candidate list size
    pub nn_list_size: usize,
}
