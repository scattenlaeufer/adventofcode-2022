use day06::get_start_index;

fn main() {
    let input_vec = utils::read_file_to_line_vec("src/input".into());
    println!(
        "The index of the package start is {}",
        get_start_index(&input_vec[0], 4)
    );
    println!(
        "The index of the message start is {}",
        get_start_index(&input_vec[0], 14)
    );
}
