# TSP Visualizer

Visualize TSP/ATSP exact, metaheuristic solutions

### Features

-   Current supported algorithms
    -   MMAS (MAX-MIN Ant System)
    -   Brute force [TODO]
    -   Physarum polycephalum [TODO]
-   Load TSP/ATSP TSPLIB instances (`EUC_2D`, `EXPLICIT`)

### Usage

1.  Place some [instances](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95) under `$XDG_DATA_HOME/tsp/instances`
1.  Execute the program: `cargo r --release`
1.  Enjoy!

### Requirements

-   Up to date Rust nightly compiler
