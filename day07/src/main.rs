use day07::Directory;

fn main() {
    let input = utils::read_file_to_line_vec("src/input".into());
    let max_size = 100000;
    println!(
        "The sum of the sizes of all directories with a size below {} is {}",
        max_size,
        Directory::init(input).get_dir_size_below(max_size)
    );
}
