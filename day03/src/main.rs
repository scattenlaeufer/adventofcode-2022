use day03::{Group, Rucksack};

fn main() {
    let content = utils::read_file_to_line_vec("src/input".into());
    let rucksack_vec: Vec<Rucksack> = content.iter().map(|c| Rucksack::from_content(c)).collect();
    println!(
        "The sum of all rucksack priorities is {}",
        rucksack_vec
            .iter()
            .map(|r| { r.get_priority() })
            .sum::<u32>()
    );

    let mut group_vec: Vec<Group> = Vec::new();
    for i in 0..rucksack_vec.len() / 3 {
        let index = i * 3;
        group_vec.push(Group::new([
            &rucksack_vec[index],
            &rucksack_vec[index + 1],
            &rucksack_vec[index + 2],
        ]));
    }
    println!(
        "The sum of all group priorities is {}",
        group_vec.iter().map(|g| { g.get_priority() }).sum::<u32>()
    );
}
