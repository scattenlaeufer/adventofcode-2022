use day02::Round;

fn main() {
    println!("Score: {}", Round::parse_from_line("A Y").score());
    println!("Score: {}", Round::parse_from_line("B X").score());
    println!("Score: {}", Round::parse_from_line("C Z").score());
}
