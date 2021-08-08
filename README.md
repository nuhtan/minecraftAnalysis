# Minecraft Analysis for 1.18 Experimental Snapshot

## Contents Explanation:
- [mining_data](mining_data/) Contains csv files with the results from the mining simulations.
- [python](python/) Contains the now depreciated code that was the basis for the rust implementation. There are errors and inconsistencies that can be found in this code.
    - The inner contents of this directory reflect the same structure and thought process as [src](src/).
- [regions](regions/) Contains the region files that data is collected from.
- [results](results/) Contains graphs and other metrics for analysis.
- [src](src/) Contains the rust code that is the center of this project.
    - [main.rs](src/main.rs) Contains the code running simulations in parallel, code to determine which simulation to run, and the functionality to record the data into csv files.
    - [mining.rs](src/mining.rs) Contains many helper functions to enable more complicated mining strategies to easily be created.
    - [techniques.rs](src/techniques.rs) Contains the techniques that can be simulated.
- [results.py](results.py) Contains the code that generates graphs using data from [mining_data](mining_data/).
- [ValidBlocks.txt](ValidBlocks.txt) Contains the blocks that the simulations check for along with the output parameters.

## How to install (Not Accurate):
```
git clone https://github.com/nuhtan/minecraftAnalysis.git
```
```
cargo build --release
```
```
./target/release/minecraft_analysis
```

## What still needs to be done:
- Documentation still needs to be finished for rust files.
- Get Nick's R files and include them.
- [results.py](results.py) should be broken into functions for eaier reading and reusability.
- Results should also generate stacked bar graphs to compare the different mining strategies directly.
- Results should be changed to be a ratio of the block in question compared to the number of blocks mined/exposed.
- Create an explanation and comparison for results.
- There should be comparisons within the mining techniques to see what parameters lead to the most efficient mining strategy.
- Test cases and benchmarks.
- CLI walk through as an alternative to inputting cli arguments.
- Project directories should be created if they don't already exist.
- Python files should be moved to a separate repository as they no longer serve as a reference due to bugs and inaccuracies.