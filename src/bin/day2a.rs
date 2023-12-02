use advent::{day2::*, read_stdin};

fn main() {
    let maxes = parse_colors("12 red cubes, 13 green cubes, and 14 blue cubes".to_string());

    let mut collection = Vec::<u32>::new();

    for line in read_stdin() {
        let game = parse_game(line.expect("should be a line"));

        if game.highs.red <= maxes.red
            && game.highs.green <= maxes.green
            && game.highs.blue <= maxes.blue
        {
            collection.push(game.id);
        }
    }

    let result = &collection.iter().sum::<u32>();

    println!("{result:?}");
}
