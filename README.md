# Minecraft Analysis for 1.18 Experimental Snapshot

## Contents Explanation:
- There are two directories that need some context.
    - [regions](regions/)
    - [results](results/)
- Regions contains the regions files that were used for data collection. There are 26 files that have had all of the 32x32 chunks generated.
- Results contains the results of running [this](src/main.rs) rust file with the default parameters that are currently in [program.py](program.py).
- [program.py](program.py) is a python file that opens a minecraft region file and simulates a specified mining strategy.
- [main.rs](src/main.rs) is a rust file to run multiple iterations of [program.py](program.py) in parallel.
- [results.py](results.py) reads the results from [program.py](program.py) and generates graphs of the distributions of ores and lava found depending on y level.

## How to install:
```
git clone https://github.com/nuhtan/minecraftAnalysis.git
```
```
pip install git+https://github.com/nuhtan/anvil-parser.git
```
```
cargo build
```

### Run either of the .py files as such:
```
python program.py r.x.z.mca strategy
```
```
python results.py
```
where r.x.z.mca is one of the region files and strategy is either basic or poke.


### For the rust file just run
```
cargo run n
```
where n is the number of threads you would like to allocate.

## What still needs to be done:
- The individual mining functions should return the nunmber of blocks mined rather than having the number hardcoded or semi hard coded in.
- Mining functions and strategies should be verified for correctness.
- Ore expansion should increase the blocks mined rather than it being a constant.
- Documentation still needs to be finished for both python and rust files.
- Get Nick's R files and include them.
- [results.py](results.py) should be broken into functions for eaier reading and reusability.
- Results should also generate stacked bar graphs to compare the different mining strategies directly.
- Results should be changed to be a ratio of the block in question compared to the number of blocks mined/exposed.
- This file needs to be updated to explain the contents of the repo better.
- Create an explanation and comparison for results.
- There should be comparisons within the mining techniques to see what parameters lead to the most efficient mining strategy.