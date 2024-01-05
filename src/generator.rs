use bevy::prelude::*;
use sudoku::board::Sudoku;

pub struct GeneratePlugin;

impl Plugin for GeneratePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SudokuGenerator>()
            .add_event::<NewPuzzle>()
            .add_systems(Startup, first_puzzle)
            .add_systems(Update, new_puzzle);
    }
}

#[derive(Event)]
struct NewPuzzle;

fn first_puzzle(mut new_ev: ResMut<Events<NewPuzzle>>) {
    new_ev.send(NewPuzzle);
}

fn new_puzzle(
    mut new_puzzle: EventReader<NewPuzzle>,
    mut sudoku_generator: ResMut<SudokuGenerator>,
) {
    if new_puzzle.read().next().is_some() {
        *sudoku_generator = SudokuGenerator::new();
    }
}

#[derive(Resource)]
pub struct SudokuGenerator {
    complete: Sudoku,
    init: Sudoku,
}

impl Default for SudokuGenerator {
    fn default() -> Self {
        let complete = Sudoku::generate_solved();
        let init = Sudoku::generate_from(complete);
        Self { complete, init }
    }
}

impl SudokuGenerator {
    pub fn new() -> Self {
        Self::default()
    }
}
