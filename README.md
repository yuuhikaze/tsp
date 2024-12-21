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

### More info

-   Read more about this project here: <https://yuuhikaze.github.io/static/dmath.html>

### References

-   Stützle, T., & Dorigo, M. (1999). ACO algorithms for the traveling salesman problem. Evolutionary Algorithms in Engineering and Computer Science, 4, 163–183. <http://staff.washington.edu/paymana/swarm/stutzle99-eaecs.pdf>
-   Stützle, T., & Hoos, H. H. (2000). MAX-MIN Ant System. Future Generation Computer Systems, 16(8), 889–914. <https://doi.org/10.1016/S0167-739X(00)00043-1>
-   RSkinderowicz. (2024). RSkinderowicz/MAX-MIN-Ant-System [C++]. <https://github.com/RSkinderowicz/MAX-MIN-Ant-System> (Original work published 2018)
