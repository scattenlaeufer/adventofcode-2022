use day02::Round;

fn main() {
    let line_vec = utils::read_file_to_line_vec("src/input".into());
    let mut total_score = 0;
    for line in line_vec {
        total_score += Round::parse_from_line(&line).score();
    }
    println!("Total score is: {}", total_score);
}
