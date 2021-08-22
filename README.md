# Minecraft Analysis for 1.18 Experimental Snapshots

## Project Explanation:
TODO

## Project Layout Description:
- [chunk_data/](chunk_data/): Contains simulated data from the current [region files](regions/).
- [mining_data/](mining_data/): Contains simulated data for each of the [current techniques] for each of the [region files](regions/).
- [regions/](regions/): Contains fully generated region files from Minecraft 1.18 Experimental Snapshot 4. These files were generated using the [world-pregen](https://github.com/GoldenDelicios/world-pregen) datapack.
- [results/](results/): Contains csv files that have been processed and are ready for display.
- [graphical_results/](graphical_results/): Contains graphs from different simulations.
- [src/](src/): Contains the rust source code that runs and manages the simulations.
    - [bin/](src/bin/): Contains files related to the executable of the project.
        - [mc_analysis.rs](): Contains the code to launch the simulations.
        - [ui.rs](): Contains the code to draw a ui for selecting the simulation type and parameters along with a ui for monitoring simulations.
    - [lib.rs](): Contains a small amount of code relating to updating simulation status.
    - [mining.rs](): Contains functions that make up the steps to simulate mining such as mining a 2x1 or 1x1 hole in a horizontal direction.
    - [simulations.rs](): Contains functions that handle the larger task of simulating a mining technique. These functions go through the process of using techniques to get blocks, trimming possible duplicate blocks (this shouldn't happen?), categorizing blocks, and recording data.
    - [techniques.rs](): Contains the various techniques that are simulated.
- [static/](static/): Contains the files for the website that hosts the analysis.

## Libraries used in the creation of this project:
- [anvil-parser](https://github.com/matcool/anvil-parser): This is the original Python library that was used to get block data from Minecraft worlds.
- [anvil-parser (fork)](https://github.com/nuhtan/anvil-parser): This is my fork of anvil-parser that has changes to allow access to the new block height ranges.
- [depreciated_minecraft_analysis](https://github.com/nuhtan/depreciated_minecraft_analysis): This was the original code for this project before performance concerns and scope creep became concerns.
- [hematite-nbt](https://github.com/PistonDevelopers/hematite_nbt): The Rust library to read the nbt file structure that Minecraft region files use.
- [mvp_anvil](https://github.com/nuhtan/mvp_anvil): A custom Rust library that provides support for getting regions, chunks, and blocks out of the [Blobs](https://docs.rs/hematite-nbt/0.5.2/nbt/struct.Blob.html) that [hematite-nbt](https://github.com/PistonDevelopers/hematite_nbt) returns for region files.
- [csv](https://github.com/BurntSushi/rust-csv): A Rust library for interacting with .csv files.
- [threadpool](https://github.com/rust-threadpool/rust-threadpool): A library that provides a Struct to manage launch multithreaded workloads on a fixed number of worker threads.
- [tui](https://github.com/fdehau/tui-rs): A terminal ui library, the [crossterm](https://github.com/crossterm-rs/crossterm) library is being used as a backend for better support on more OS's.

## How to install and run:
### Using a prebuilt executable:
1. Navigate to the actions tab of this repo.
2. Click the latest workflow run that has a passing build (green checkmark).
3. Under 'Artifacts' download *mcAnalysis* for Linux or *mcAnalysis.exe* for Windows. I do not currently have access to a Mac computer to test the build process with.
4. Place the file in a directory and launch it. On Linux you may need to give the file execution permissions with 'chmod +x mcAnalysis'.
5. The ui should launch in a terminal and some directories should be created.
6. Follow the instructions in the ui and place region files in the regions/ directory.
7. Press enter to continue and select whatever simulation you would like to run.
### Building yourself:
```
git clone https://github.com/nuhtan/minecraft_analysis.git
```
```
cd minecraft_analysis
```
```
cargo build --release
```
- The executable will be located in 'target/release/'.
- You can now run './target/release/mc_analysis' or /target/release/mc_analysis.exe'.
- Continue from step 5 of the prebuilt executable instructions.

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
- Update the directories that are being created if they do not already exist.