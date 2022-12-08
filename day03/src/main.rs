use day03::Rucksack;

fn main() {
    let content = utils::read_file_to_line_vec("src/input".into());
    println!(
        "The sum of all priorities is {}",
        content
            .iter()
            .map(|c| { Rucksack::from_content(c).get_priority() })
            .sum::<u32>()
    );
}
