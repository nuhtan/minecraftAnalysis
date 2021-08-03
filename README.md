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