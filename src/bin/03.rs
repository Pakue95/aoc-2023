advent_of_code::solution!(3);
extern crate timeit;
use array2d::Array2D;
use itertools::Itertools;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct PartNum {
    number: u32,
    digits_pos: Vec<(usize, usize)>,
}

fn get_adjacent_pos(pos: &(usize, usize), radius: usize) -> Vec<(usize, usize)> {
    (pos.0.saturating_sub(radius)..=pos.0.saturating_add(radius))
        .cartesian_product(pos.1.saturating_sub(radius)..=pos.1.saturating_add(radius))
        .filter(|x| *x != *pos)
        .collect()
}

fn get_adjacent(arr: &Array2D<char>, pos: &(usize, usize)) -> Vec<char> {
    let options = get_adjacent_pos(pos, 1);

    options
        .iter()
        .filter_map(|x| arr.get(x.0, x.1))
        .cloned()
        .collect()
}

fn get_adjacent_partnum(
    arr: &Array2D<Option<PartNum>>,
    pos: &(usize, usize),
) -> Vec<Option<PartNum>> {
    let options = get_adjacent_pos(pos, 1);

    options
        .iter()
        .filter_map(|x| arr.get(x.0, x.1))
        .map(|x| (*x).clone())
        .collect()
}

fn index_part_nums(arr: &Array2D<char>) -> Vec<PartNum> {
    let mut result = Vec::new();
    for (row_idx, row_iter) in arr.rows_iter().enumerate() {
        let items: Vec<(usize, &char)> = row_iter.enumerate().collect();
        for (is_digit, characters_group) in &items.iter().group_by(|(_idx, c)| c.is_ascii_digit()) {
            if !is_digit {
                continue;
            }

            let characters: Vec<&(usize, &char)> = characters_group.collect();

            let number: u32 = String::from_iter(characters.iter().map(|x| x.1))
                .parse()
                .expect("Can't create digit string!");

            let indices: Vec<(usize, usize)> = characters.iter().map(|x| (x.0, row_idx)).collect();

            result.push(PartNum {
                number,
                digits_pos: indices,
            });
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let n_rows: usize = input.lines().count();
    let n_col: usize = input.lines().next().unwrap().len();
    let char_it = input.chars().filter(|x| *x != '\n');
    let grid = Array2D::from_iter_row_major(char_it, n_rows, n_col).expect("INPUT");

    // Search all part numbers with their positions in the input
    let all_part_num = index_part_nums(&grid);

    // Go through all part numbers and check whether they are next to a symbol
    let result: u32 = all_part_num
        .iter()
        .filter_map(|part| {
            for (x, y) in &part.digits_pos {
                for c in get_adjacent(&grid, &(*y, *x)) {
                    if !c.is_ascii_digit() && c != '.' {
                        return Some(part.number);
                    }
                }
            }
            None
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let n_rows: usize = input.lines().count();
    let n_col: usize = input.lines().next().unwrap().len();
    let char_it = input.chars().filter(|x| *x != '\n');
    let grid = Array2D::from_iter_row_major(char_it, n_rows, n_col).expect("INPUT");

    // Search all part numbers with their positions in the input
    let all_part_num = index_part_nums(&grid);

    let mut part_num_grid: Array2D<Option<PartNum>> =
        Array2D::filled_with(None, grid.num_rows(), grid.num_columns());
    for num in &all_part_num {
        for pos in &num.digits_pos {
            part_num_grid.set(pos.1, pos.0, Some(num.clone())).ok();
        }
    }

    let result: u32 = grid
        .indices_column_major()
        .filter(|(row, column)| *grid.get(*row, *column).expect("Empty field!") == '*')
        .filter_map(|(gear_x, gear_y)| {
            let mut adj_nums: Vec<PartNum> = Vec::new();
            for part in get_adjacent_partnum(&part_num_grid, &(gear_x, gear_y)) {
                if part.is_none() {
                    continue;
                }
                adj_nums.push(part.unwrap());
            }
            let adj_nums: Vec<PartNum> = adj_nums.into_iter().unique().collect();
            if adj_nums.len() == 2 {
                return Some(adj_nums[0].number * adj_nums[1].number);
            }
            None
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use timeit::timeit;
    use timeit::timeit_loops;
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }

    #[test]
    fn test_speed() {

        timeit!({
            let input = &advent_of_code::template::read_file("inputs", DAY);
            let char_vec: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
            let _grid = Array2D::from_rows(&char_vec).expect("Array2D from char vector had an error!");
            // println!("{_grid:?}");
        });

        timeit!({
            let input = &advent_of_code::template::read_file("inputs", DAY);
            let n_rows: usize = input.lines().count();
            let n_col: usize = input.lines().next().unwrap().len();
            let char_it = input.lines().map(|x| x.chars()).flatten();
            // println!("rows: {n_rows} cols: {n_col}, {char_it}");
            let _grid = Array2D::from_iter_row_major(char_it, n_rows, n_col);
            // println!("{_grid:?}");
        });
    }
}
