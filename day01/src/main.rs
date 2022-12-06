use std::path::PathBuf;
use utils;

fn read_file_to_vec(file_name: PathBuf) -> Vec<Vec<i32>> {
    let mut current_elve = Vec::new();
    let mut result = Vec::new();
    let lines_vec = utils::read_file_to_line_vec(file_name);
    for line in lines_vec {
        if line.is_empty() {
            result.push(current_elve);
            current_elve = Vec::new();
        } else {
            current_elve.push(line.parse().unwrap())
        }
    }

    result
}

fn main() {
    let input_data = read_file_to_vec("src/input".into());
    let max_calories = input_data
        .iter()
        .map(|elve| elve.iter().sum::<i32>())
        .reduce(i32::max);
    println!("max calories for one elve: {:?}", max_calories);
    let mut calories_by_elve = input_data
        .iter()
        .map(|elve| elve.iter().sum::<i32>())
        .collect::<Vec<i32>>();
    calories_by_elve.sort();
    calories_by_elve.reverse();
    let top_three_calories = &calories_by_elve[0..3];
    println!(
        "calories of the top three elves: {:?}",
        top_three_calories.iter().sum::<i32>()
    )
}
