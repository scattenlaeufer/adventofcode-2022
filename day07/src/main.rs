use day07::{Directory, MIN_FREE_SPACE};

fn main() {
    let input = utils::read_file_to_line_vec("src/input".into());
    let max_size = 100000;
    let dir = Directory::init(input);
    println!(
        "The sum of the sizes of all directories with a size below {} is {}",
        max_size,
        dir.get_dir_size_below(max_size)
    );
    println!(
        "The size of the smallest directory to be deleted to {} free is {}",
        MIN_FREE_SPACE,
        dir.get_smallest_dir_size_to_delete(MIN_FREE_SPACE)
    )
}
