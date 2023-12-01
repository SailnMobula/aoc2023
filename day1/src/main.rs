use std::env::args;
use std::fs::read_to_string;

fn main() {
    let file_path = args().nth(1).expect("No file path provided");
    let puzzle_input = get_puzzle_input(&file_path).expect("Could not read file");

    println!("Result A: {}", complete_a(&puzzle_input));
    println!("Result B: {}", complete_b(&puzzle_input));
}

fn complete_a(puzzle_input: &[String]) -> u32 {
    let first_and_last_vec = find_first_and_last(puzzle_input, |s| {
        digits_to_u32s(s).iter().map(|(_, digit)| *digit).collect()
    });
    sum_it(first_and_last_vec)
}

fn complete_b(puzzle_input: &[String]) -> u32 {
    let first_and_last_vec = find_first_and_last(puzzle_input, |s| digits_and_words_to_u32(s));
    sum_it(first_and_last_vec)
}

fn sum_it(input: Vec<(Option<u32>, Option<u32>)>) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(_, c)| {
            format!("{}{}", c.0.unwrap_or(0), c.1.unwrap_or(0))
                .trim()
                .parse::<u32>()
                .unwrap_or(0)
        })
        .sum()
}

fn find_first_and_last<F>(puzzle_input: &[String], extractor: F) -> Vec<(Option<u32>, Option<u32>)>
where
    F: Fn(&String) -> Vec<u32>,
{
    puzzle_input
        .iter()
        .map(|s| {
            let extracted_digits = extractor(s);
            (
                extracted_digits.first().cloned(),
                extracted_digits.last().cloned(),
            )
        })
        .collect::<Vec<(_, _)>>()
}

fn digits_and_words_to_u32(s: &str) -> Vec<u32> {
    let mut words_digits = words_to_u32s(s);
    let mut char_digits = digits_to_u32s(s);
    words_digits.append(&mut char_digits);
    words_digits.sort_by(|a, b| a.0.cmp(&b.0));
    words_digits.iter().map(|(_, digit)| *digit).collect()
}

fn words_to_u32s(s: &str) -> Vec<(u32, u32)> {
    let number_words = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut numbers = number_words
        .iter()
        .enumerate()
        .flat_map(|(digit, word)| {
            s.match_indices(word)
                .map(|(i, _)| (i as u32, digit as u32))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    numbers.sort_by(|a, b| a.0.cmp(&b.0));
    numbers
}

fn digits_to_u32s(s: &str) -> Vec<(u32, u32)> {
    s.char_indices()
        .map(|(i, c)| (i, c.to_digit(10)))
        .filter(|(_, c)| c.is_some())
        .map(|(i, c)| (i as u32, c.unwrap()))
        .collect()
}

fn get_puzzle_input(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file_contents = read_to_string(file_path)?;
    let puzzle_input: Vec<String> = file_contents.lines().map(|s| s.to_string()).collect();
    Ok(puzzle_input)
}
