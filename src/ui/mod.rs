use std::{collections::VecDeque, fs, iter::FromIterator, path::Path, process, time::Instant};

use tui::widgets::ListState;

use crate::techniques::Technique;

pub mod simulation;
pub mod simulation_target;


pub struct StatefulList<T> {
    pub state: ListState,
    pub items: VecDeque<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: VecDeque::new(),
        }
    }

    pub fn with_items(items: VecDeque<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn add_item(&mut self, item: T) {
        self.items.push_front(item);
        while self.items.len() > 100 {
            self.items.pop_back();
        }
    }
}

#[derive(Clone)]
pub struct Simulation {
    pub id: u32,
    pub technique: Technique,
    pub file: String,
    pub activity: String,
    pub start: Instant,
    pub y: i32,
    pub mined: u32,
    pub exposed: u32,
    pub lava: u32,
    pub ores: u32,
}

impl Simulation {
    fn new(id: u32, technique: Technique, file: String, start: Instant, y: i32) -> Simulation {
        return Simulation {
            id,
            technique,
            file,
            activity: String::from("Initializing Simulation"),
            start,
            y,
            mined: 0,
            exposed: 0,
            lava: 0,
            ores: 0,
        };
    }
}

pub enum Simulations {
    Single(Technique, String, i32),
    Range(Technique, String, i32, i32),
    Techniques(Vec<Technique>, i32, i32, u32),
    TechniqueParameters(Vec<Technique>, i32, i32, u32),
    Chunks(i32, i32, u32),
}

// Create mining_data, regions, if they are not already present. Fetch ValidBlocks.txt if it is not present.
fn verify_directory_structure() -> bool {
    let mut regions = true;
    // These two paths should be changed to create the dir and handle the error rather than 
    if !Path::new("mining_data/").exists() {
        fs::create_dir("mining_data/").unwrap();
    }

    if !Path::new("chunk_data/").exists() {
        fs::create_dir("chunk_data/").unwrap();
    }

    if !Path::new("regions/").exists() {
        fs::create_dir("regions/").unwrap();
        regions = false;
    } else if Path::new("regions/").read_dir().unwrap().next().is_none() {
        regions = false;
    }

    if !Path::new("ValidBlocks.txt").exists() {
        process::Command::new("curl").args(&["https://raw.githubusercontent.com/nuhtan/minecraft_analysis/master/ValidBlocks.txt", "-o", "ValidBlocks.txt"]).spawn().unwrap();
    }

    return regions;
}

pub enum SimType {
    Single,
    Range,
    Techniques,
    TechniquesParameters,
    Chunk,
    Quit,
}

impl SimType {
    fn iterable() -> Vec<String> {
        return vec![
            "Single",
            "Range",
            "Techniques",
            "Parameters",
            "Chunk",
            "Quit",
        ]
        .iter()
        .map(|f| f.to_string())
        .collect();
    }
}

#[derive(Clone, Copy)]
pub enum UIRenderState {
    DirectoryStructure,
    SimulationType,
    TechniqueSelect,
    TechniquesSelect,
    ThreadCount,
    YLevel,
    YRange,
    RegionSelect,
    Simulate,
    Quit,
    Error,
}

pub struct UIState {
    sim_type: StatefulList<String>,
    technique: StatefulList<String>,
    techniques: Vec<usize>,
    no_yes: (StatefulList<String>, UIRenderState),
    y_level: String,
    min: String,
    max: String,
    second_range: bool,
    error: (String, UIRenderState),
    files: StatefulList<String>,
    threads: String,
    techniques_current: usize,
}

impl UIState {
    fn new() -> UIState {
        let mut file_names = Vec::new();
        for file in fs::read_dir("regions").unwrap() {
            let file = file.unwrap();
            let f_name = file.file_name();
            let name = f_name.to_str().unwrap().to_string();
            let c_name = name.clone();
            file_names.push(c_name);
        }
        return UIState {
            sim_type: StatefulList::with_items(VecDeque::from_iter(SimType::iterable())),
            technique: StatefulList::with_items(VecDeque::from_iter(Technique::iterable())),
            techniques: Vec::new(),
            no_yes: (
                StatefulList::with_items(VecDeque::from_iter(
                    ["no".to_string(), "yes".to_string()]
                        .iter()
                        .map(|f| f.clone())
                        .collect::<Vec<String>>(),
                )),
                UIRenderState::SimulationType,
            ),
            y_level: String::new(),
            min: String::new(),
            max: String::new(),
            second_range: false,
            error: (String::new(), UIRenderState::SimulationType),
            files: StatefulList::with_items(VecDeque::from_iter(
                file_names
                    .iter()
                    .map(|f| f.as_str())
                    .filter(|f| f.contains(".mca")) // Only include region files
                    .map(|f| f.to_string()), // change str to String
            )),
            threads: String::new(),
            techniques_current: 0,
        };
    }
}