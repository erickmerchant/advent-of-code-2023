use advent::{day2::*, read_stdin};

fn main() {
    let mut collection = Vec::<u32>::new();

    for line in read_stdin() {
        let game = parse_game(line.expect("should be a line"));

        collection.push(game.highs.red * game.highs.green * game.highs.blue);
    }

    let result = &collection.iter().sum::<u32>();

    println!("{result:?}");
}
