use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
    iter::Peekable,
    path::PathBuf,
};

#[derive(Debug, PartialEq)]
enum EdgeWeightType {
    Euc2d,
    Explicit,
}

#[derive(Debug)]
pub struct TspLibInstance {
    path: PathBuf,
    dimension: usize,
    symmetric: bool,
    edge_weight_type: EdgeWeightType,
    distance_matrix: Vec<f64>,
    nn_matrix: Vec<Vec<usize>>,
}

impl TspLibInstance {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            dimension: 0,
            symmetric: true,
            edge_weight_type: EdgeWeightType::Euc2d,
            distance_matrix: Default::default(),
            nn_matrix: Default::default(),
        }
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn load_data_from_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines().peekable();
        while let Some(line_result) = lines.next() {
            let line = line_result?;
            if line.starts_with("TYPE") {
                self.symmetric = if line.contains("TSP") {
                    true
                } else if line.contains("ATSP") {
                    false
                } else {
                    return Err("Problem is different from the supported kinds: TSP, ATSP".into());
                };
            } else if line.starts_with("DIMENSION") {
                self.dimension = line
                    .split(':')
                    .nth(1)
                    .ok_or("Instance dimension was not found")?
                    .trim()
                    .parse()?;
            } else if line.starts_with("EDGE_WEIGHT_TYPE") {
                self.edge_weight_type = if line.contains("EUC_2D") {
                    EdgeWeightType::Euc2d
                } else if line.contains("EXPLICIT") {
                    EdgeWeightType::Explicit
                } else {
                    return Err(
                        "Edge weight type is different from the supported kinds: EUC_2D, EXPLICIT"
                            .into(),
                    );
                };
            } else if line.starts_with("NODE_COORD_SECTION") {
                self.read_node_coordinates_section(&mut lines)?;
            } else if line.starts_with("EDGE_WEIGHT_SECTION") {
                self.read_edge_weight_section(&mut lines)?;
            }
        }
        Ok(())
    }

    fn read_node_coordinates_section(
        &mut self,
        lines: &mut Peekable<std::io::Lines<BufReader<File>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut coords: Vec<(f64, f64)> = Vec::with_capacity(self.dimension);
        for line_result in lines.by_ref() {
            let line = line_result?;
            if line.contains("EOF") {
                break;
            }
            let parts: Vec<f64> = line
                .split_whitespace()
                .skip(1) // Skip node index
                .take(2) // Take `x1` and `x2`
                .map(|s| s.parse().unwrap())
                .collect();
            coords.push((parts[0], parts[1]));
        }
        self.distance_matrix
            .resize(self.dimension * self.dimension, Default::default());
        (0..self.dimension).for_each(|i| {
            let (from_x, from_y) = coords[i];
            (0..self.dimension).for_each(|j| {
                if i != j {
                    let (to_x, to_y) = coords[j];
                    let dx = to_x - from_x;
                    let dy = to_y - from_y;
                    let distance = (dx.powi(2) + dy.powi(2)).sqrt().round(); // !!!
                    self.distance_matrix[i * self.dimension + j] = distance;
                }
            });
        });
        Ok(())
    }

    fn read_edge_weight_section(
        &mut self,
        lines: &mut Peekable<std::io::Lines<BufReader<File>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.edge_weight_type != EdgeWeightType::Explicit {
            return Err("Expected EXPLICIT edge weight type".into());
        }
        for line_result in lines.by_ref() {
            let line = line_result?;
            if line.contains("EOF") {
                break;
            }
            let parts = line.split_whitespace();
            for dist_str in parts {
                let distance: f64 = dist_str.parse().unwrap();
                self.distance_matrix.push(distance);
            }
        }
        assert!(self.distance_matrix.len() == self.dimension * self.dimension);
        Ok(())
    }

    /// initializes nearest neighbor martix (per node)
    pub fn initialize_nn_matrix(&mut self, mut nn_list_size: usize) {
        self.nn_matrix.resize(self.dimension, Default::default());
        nn_list_size = min(self.dimension - 1, nn_list_size);
        for node in 0..self.dimension {
            let mut nn_list: Vec<usize> = (0..self.dimension).collect();
            nn_list.sort_by(|&a, &b| {
                self.get_distance(node, a)
                    .partial_cmp(&self.get_distance(node, b))
                    .unwrap()
            });
            assert!(self.get_distance(node, nn_list[0]) <= self.get_distance(node, nn_list[1]));
            let nn_bounded_list = &mut self.nn_matrix[node];
            nn_bounded_list.clear();
            nn_bounded_list.reserve(nn_list_size);
            nn_bounded_list.extend(
                nn_list
                    .iter()
                    .filter(|&&x| x != node)
                    .take(nn_list_size)
                    .cloned(),
            );
        }
    }

    fn get_distance(&self, from: usize, to: usize) -> f64 {
        self.distance_matrix[from * self.dimension + to]
    }
}
