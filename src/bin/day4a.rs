use advent::{day4::*, read_stdin};

fn main() {
    let mut collection = Vec::<u32>::new();

    for line in read_stdin() {
        let game = parse_game(line.expect("should be a line"));
        let win_count = game.winning.intersection(&game.actual).count() as u32;
        let points = if win_count > 0 {
            2_u32.pow(win_count - 1)
        } else {
            0_u32
        };

        collection.push(points);
    }

    let result = &collection.iter().sum::<u32>();

    println!("{result:?}");
}
