use day04::Group;

fn main() {
    let input_vec = utils::read_file_to_line_vec("src/input".into());
    println!(
        "Nuber of fully overlapping groups: {:?}",
        input_vec
            .iter()
            .map(|i| Group::from_str(&i).one_contains_the_other() as u32)
            .sum::<u32>()
    );
}
