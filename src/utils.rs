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

pub fn get_intances_dir() -> PathBuf {
    INSTANCES_DIR.clone()
}

// Creates data dir if it does not exist
pub fn create_data_dir() {
    fs::create_dir_all(&*INSTANCES_DIR).unwrap();
}

// Returns an iterator over the data directory
pub fn read_data_dir() -> ReadDir {
    fs::read_dir(&*INSTANCES_DIR).unwrap()
}
