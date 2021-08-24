use std::{collections::VecDeque, io, sync::{Arc, Mutex, mpsc::Receiver}, time::{Duration, Instant}};

use crossterm::{event::{self, Event, KeyCode, poll}, terminal::{self, disable_raw_mode}};
use tui::{Terminal, backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Style}, text::Spans, widgets::{Block, Borders, List, ListItem, Paragraph}};

use crate::ProgramStatus;

use super::{Simulation, StatefulList};


pub fn simulation_ui(
    receiver: Receiver<ProgramStatus>,
    ender: Arc<Mutex<bool>>,
    title: String,
    files: u32,
    threads: u32,
    techniques: u32,
    y_range: (i32, i32),
    start: Instant,
) {
    let backend = CrosstermBackend::new(io::stdout());
    terminal::enable_raw_mode().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    let mut state: StatefulList<Simulation> = StatefulList::with_items(VecDeque::new());
    state.state.select(Some(0));
    let mut completed = 0;
    loop {
        for _ in 0..25 {
            match receiver.recv_timeout(Duration::from_millis(1)) {
                Ok(status) => match status {
                    ProgramStatus::StartingSim(id, technique, file, start, y) => {
                        state.add_item(Simulation::new(id, technique, file, start, y));
                    }
                    ProgramStatus::UpdateSim(id, activity, mined, exposed, lava, ores) => {
                        let mut loc = 0;
                        for i in 0..state.items.len() {
                            if state.items[i].id == id {
                                loc = i;
                            }
                        }
                        state.items[loc].activity = activity;
                        state.items[loc].mined = mined;
                        state.items[loc].exposed = exposed;
                        state.items[loc].lava = lava;
                        state.items[loc].ores = ores;
                        terminal.clear().unwrap();
                    }
                    ProgramStatus::FinishSim(id) => {
                        println!("Finish message");
                        let mut loc = 0;
                        for i in 0..state.items.len() {
                            if state.items[i].id == id {
                                loc = i;
                            }
                        }
                        state.items.remove(loc).unwrap();
                        state.state.select(Some(0));
                        completed += 1
                    }
                },
                Err(_) => {}
            }
        }

        terminal
            .draw(|f| {
                let sections = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                    .split(f.size());
                let left_sections = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                    .split(sections[0]);
                let right_sections = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                    .split(sections[1]);
                let secs = start.elapsed().as_secs() % 60;
                let mins = (start.elapsed().as_secs() / 60) % 60;
                let hours = (start.elapsed().as_secs() / 3600) % 24;
                let days = start.elapsed().as_secs() / (3600 * 24);
                let top_right = Paragraph::new(vec![
                    Spans::from(title.clone()),
                    Spans::from(format!(
                        "Duration: {:02}:{:02}:{:02}:{:02}",
                        days, hours, mins, secs
                    )),
                ])
                .block(Block::default().borders(Borders::ALL));
                let bot_right = Paragraph::new(vec![
                    Spans::from(format!("{} Region Files", files)),
                    Spans::from(format!("{} Threads Allocated", threads)),
                    Spans::from(format!("{} Simulations Completed", completed)),
                    Spans::from(format!("{} Techniques", techniques)),
                    Spans::from(format!("Y: [{}, {}]", y_range.0, y_range.1)),
                ])
                .block(Block::default().borders(Borders::ALL));
                let items: Vec<ListItem> = state
                    .items
                    .iter()
                    .map(|i| {
                        ListItem::new(format!(
                            "({}) {} - {:02}:{:02}",
                            i.id,
                            i.activity,
                            i.start.elapsed().as_secs() / 60,
                            i.start.elapsed().as_secs() % 60
                        ))
                    })
                    .collect();
                let top_left = List::new(items)
                    .highlight_style(Style::default().fg(Color::Cyan))
                    .block(Block::default().borders(Borders::ALL));
                let bot_left_sections = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                    .split(left_sections[1]);
                let bot_left_left;
                let bot_left_right;
                if state.items.len() < 1 {
                    bot_left_left =
                        Paragraph::new("").block(Block::default().borders(Borders::ALL));
                    bot_left_right =
                        Paragraph::new("").block(Block::default().borders(Borders::ALL));
                } else {
                    let target = state.items[state.state.selected().unwrap()].clone();
                    bot_left_left = Paragraph::new(vec![
                        Spans::from(target.file),
                        Spans::from(target.activity),
                        Spans::from(format!(
                            "{:02}:{:02}",
                            target.start.elapsed().as_secs() / 60,
                            target.start.elapsed().as_secs() % 60
                        )),
                        Spans::from(format!("Y: {}", target.y)),
                    ])
                    .block(Block::default().borders(Borders::ALL));
                    bot_left_right = Paragraph::new(vec![
                        Spans::from(format!("Blocks Mined: {}", target.mined)),
                        Spans::from(format!("Blocks Exposed: {}", target.exposed)),
                        Spans::from(format!("Lava: {}", target.lava)),
                        Spans::from(format!("Ores: {}", target.ores)),
                    ])
                    .block(Block::default().borders(Borders::ALL));
                }

                f.render_widget(top_right, right_sections[0]);
                f.render_widget(bot_right, right_sections[1]);
                f.render_stateful_widget(top_left, left_sections[0], &mut state.state);
                f.render_widget(bot_left_left, bot_left_sections[0]);
                f.render_widget(bot_left_right, bot_left_sections[1]);
            })
            .unwrap();

        if poll(Duration::from_millis(200)).unwrap() {
            let event = event::read().unwrap();
            match event {
                Event::Key(key) => match key.code {
                    KeyCode::Up => state.previous(),
                    KeyCode::Down => state.next(),
                    _ => {}
                },
                _ => {}
            }
        }

        let end = ender.lock().unwrap();
        if *end {
            break;
        }
    }
    terminal.clear().unwrap();
    terminal.set_cursor(0, 0).unwrap();
    disable_raw_mode().unwrap();
}