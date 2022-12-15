use day08::Map;

fn main() {
    let input_vec = utils::read_file_to_line_vec("src/input".into());
    let map = Map::from_input_vec(input_vec);
    println!(
        "The number of visible trees is {}",
        map.get_number_of_visible_trees()
    );
}
