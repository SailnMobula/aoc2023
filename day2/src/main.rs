use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

fn main() {
    let file_path = args().nth(1).expect("No file path provided");
    let puzzle_input = get_puzzle_input(&file_path).expect("Could not read file");

    let games = parse_games(puzzle_input);
    let test_reveal_set = RevealSet::from_string("12 red, 13 green, 14 blue");

    let result_a = games
        .iter()
        .filter(|g| g.possible_with_reveal_sets(&test_reveal_set))
        .map(|g| g.id)
        .sum::<u32>();

    println!("Result a: {}", result_a);

    let result_b = games.iter().map(|g| g.calc_pow_of_set()).sum::<u32>();
    println!("Result b: {}", result_b);
}

fn parse_games(puzzle_input: Vec<String>) -> Vec<Game> {
    puzzle_input
        .iter()
        .map(|s| Game::from(s))
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct Game {
    id: u32,
    reveal_sets: Vec<RevealSet>,
}

#[derive(Debug)]
struct RevealSet {
    cubes: HashMap<String, u32>,
}

impl Game {
    fn new(id: u32, reveal_sets: Vec<RevealSet>) -> Game {
        Game { id, reveal_sets }
    }
    fn from(s: &str) -> Self {
        let mut parts = s.split(": ");
        let id = parts
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let reveal_sets = parts
            .next()
            .unwrap()
            .split("; ")
            .map(RevealSet::from_string)
            .collect();
        Game::new(id, reveal_sets)
    }
    fn possible_with_reveal_sets(&self, other: &RevealSet) -> bool {
        self.reveal_sets.iter().all(|rs| rs.fits_into(other))
    }

    fn calc_pow_of_set(&self) -> u32 {
        self.calc_max_reveal_set().iter().product()
    }

    fn calc_max_reveal_set(&self) -> Vec<u32> {
        vec!["red", "green", "blue"]
            .iter()
            .filter_map(|color| self.get_max_set_from(color))
            .collect()
    }

    fn get_max_set_from(&self, color: &str) -> Option<u32> {
        self.reveal_sets
            .iter()
            .map(|rs| rs.cubes.clone())
            .filter_map(|c| c.get(color).cloned())
            .max()
    }
}

impl RevealSet {
    fn new(cubes: HashMap<String, u32>) -> Self {
        RevealSet { cubes }
    }

    fn from_string(s: &str) -> Self {
        let cubes = s.split(", ").map(RevealSet::cube_from_string).collect();
        RevealSet::new(cubes)
    }

    fn cube_from_string(s: &str) -> (String, u32) {
        let mut parts = s.trim().split(' ');
        let count = parts.next().unwrap().parse::<u32>().unwrap();
        let color = parts.next().unwrap();
        match color {
            "blue" | "green" | "red" => (color.to_string(), count),
            _ => panic!("Invalid color"),
        }
    }

    fn fits_into(&self, other: &RevealSet) -> bool {
        self.cubes.iter().all(|(color, count)| {
            other
                .cubes
                .get(color)
                .map_or(false, |other_count| count <= other_count)
        })
    }
}
fn get_puzzle_input(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file_contents = read_to_string(file_path)?;
    let puzzle_input: Vec<String> = file_contents.lines().map(|s| s.to_string()).collect();
    Ok(puzzle_input)
}
