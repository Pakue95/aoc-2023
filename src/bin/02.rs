advent_of_code::solution!(2);

#[derive(Debug, Copy, Clone)]
struct OneDraw {
    green: u32,
    blue: u32,
    red: u32,
}

#[derive(Debug)]
struct GameOfDice {
    draws: Vec<OneDraw>,
    to_check: OneDraw,
    id: u32,
}

fn parse_line(to_check: OneDraw, line: String) -> GameOfDice {
    let (game, draws) = line.split_once(": ").unwrap();
    let id: u32 = game.split_once(' ').unwrap().1.parse::<u32>().unwrap();

    let mut return_value: GameOfDice = GameOfDice {
        draws: Vec::new(),
        to_check,
        id,
    };

    for draw in draws.split("; ") {
        let mut this_draw: OneDraw = OneDraw {
            green: 0,
            blue: 0,
            red: 0,
        };

        for ball in draw.split(", ") {
            let (count, color) = ball.split_once(' ').unwrap();
            if color == "green" {
                this_draw.green = count.parse::<u32>().unwrap()
            }
            if color == "red" {
                this_draw.red = count.parse::<u32>().unwrap()
            }
            if color == "blue" {
                this_draw.blue = count.parse::<u32>().unwrap()
            }
        }

        return_value.draws.push(this_draw)
    }

    return_value
}

fn valid_game(game: &GameOfDice) -> bool {
    return game.draws.iter().all(|x| {
        x.green <= game.to_check.green && x.blue <= game.to_check.blue && x.red <= game.to_check.red
    });
}

fn min_draw_in_game(game: &GameOfDice) -> OneDraw {
    OneDraw {
        green: game.draws.iter().map(|x| x.green).max().unwrap(),
        blue: game.draws.iter().map(|x| x.blue).max().unwrap(),
        red: game.draws.iter().map(|x| x.red).max().unwrap(),
    }
}

fn pow_of_draw(draw: &OneDraw) -> u32 {
    draw.blue * draw.green * draw.red
}

pub fn part_one(input: &str) -> Option<u32> {
    let to_check = OneDraw {
        green: 13,
        blue: 14,
        red: 12,
    };

    let result: u32 = input
        .lines()
        .map(|x| parse_line(to_check, x.to_string()))
        .filter(valid_game)
        .map(|x| x.id)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let to_check = OneDraw {
        green: 13,
        blue: 14,
        red: 12,
    };

    let result: u32 = input
        .lines()
        .map(|x| {
            let game = parse_line(to_check, x.to_string());
            let min_draw = min_draw_in_game(&game);
            pow_of_draw(&min_draw)
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
