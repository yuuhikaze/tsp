pub mod matrix {
    use std::ops::{Index, IndexMut};

    pub struct Matrix<T> {
        data: Vec<T>,
        cols: usize,
    }

    impl<T: Clone> Matrix<T> {
        pub fn new(dimension: (usize, usize), default_value: T) -> Self {
            Matrix {
                data: vec![default_value; dimension.0 * dimension.1],
                cols: dimension.1,
            }
        }

        pub fn data(&self) -> &Vec<T> {
            &self.data
        }

        pub fn data_mut(&mut self) -> &mut Vec<T> {
            &mut self.data
        }

        pub fn resize(&mut self, dimension: (usize, usize), default_value: T) {
            self.data.resize(dimension.0 * dimension.1, default_value);
            self.cols = dimension.1;
        }
    }

    impl<T> Default for Matrix<T> {
        fn default() -> Self {
            Self {
                data: Default::default(),
                cols: Default::default(),
            }
        }
    }

    impl<T> Index<(usize, usize)> for Matrix<T> {
        type Output = T;
        fn index(&self, index: (usize, usize)) -> &Self::Output {
            let (row, col) = index;
            &self.data[row * self.cols + col]
        }
    }

    impl<T> IndexMut<(usize, usize)> for Matrix<T> {
        fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
            let (row, col) = index;
            &mut self.data[row * self.cols + col]
        }
    }
}

pub mod storage {
    use std::{
        fs::{self, ReadDir},
        path::PathBuf,
        sync::LazyLock,
    };

    use directories::ProjectDirs;

    static INSTANCES_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        ProjectDirs::from("com", "yuuhikaze", "tsp")
            .expect("Unable to determine project directories")
            .data_dir()
            .join("instances")
    });

    /// Returns instances directory
    pub fn get_intances_dir() -> PathBuf {
        INSTANCES_DIR.clone()
    }

    /// Creates data dir if it does not exist
    pub fn create_data_dir() {
        fs::create_dir_all(&*INSTANCES_DIR).unwrap();
    }

    /// Returns an iterator over the data directory
    pub fn read_data_dir() -> ReadDir {
        fs::read_dir(&*INSTANCES_DIR).unwrap()
    }
}
