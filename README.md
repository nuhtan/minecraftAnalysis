# Minecraft Analysis for 1.18 Experimental Snapshot

## Contents Explanation:
- [mining_data](mining_data/) Contains csv files with the results from the mining simulations.
- [regions](regions/) Contains the region files that data is collected from.
- [results](results/) Contains graphs and other metrics for analysis.
- [src](src/) Contains the rust code that is the center of this project.
    - [main.rs](src/main.rs) Contains the code running simulations in parallel, code to determine which simulation to run, and the functionality to record the data into csv files.
    - [mining.rs](src/mining.rs) Contains many helper functions to enable more complicated mining strategies to easily be created.
    - [techniques.rs](src/techniques.rs) Contains the techniques that can be simulated.
- [results.py](results.py) Contains the code that generates graphs using data from [mining_data](mining_data/).
- [ValidBlocks.txt](ValidBlocks.txt) Contains the blocks that the simulations check for along with the output parameters.

## How to install and run:
```
git clone https://github.com/nuhtan/minecraftAnalysis.git
```
```
cargo build --release
```
```
./target/release/mc_analysis
```

## What still needs to be done:
- Documentation for rust files and results.py.
- Get Nick's R files and include them.
- [results.py](results.py) should be broken into functions for easier reading and reusability.
- Results should also generate stacked bar graphs to compare the different mining strategies directly.
- Create an explanation and comparison for results.
- There should be comparisons within the mining techniques to see what parameters lead to the most efficient mining strategy.
- Test cases and benchmarks.
- Chunk Simulations for general ore distributions.
- Internal documentation for more complex sections of the code.
- UI needs to handle the alternative simulations, chunk and parameter testing.
- Github pages to host conclusions.
- Upload this and mvp_anvil to crates.io.
- Exit during simulation.
- Less destructive exits, files should be saved after each y level not only once the entire range has been simulated.
- Predict finish time for simulations?
- Change UI color choices.