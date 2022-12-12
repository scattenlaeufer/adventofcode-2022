use day05::Plan;

fn main() {
    let input = utils::read_file_to_line_vec("src/input".into());
    println!("The top crates are {:?}", Plan::from_vec(input).run());
}
