use std::cell::RefCell;

use advent_of_code::Grid;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<char> = Grid::parse_lines(input);

    Some(
        grid.data
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, c)| *c == '@')
            .filter(|(i, _)| {
                grid.neighbors_with_diagonals(*i)
                    .filter(|(_, i)| grid.data[*i] == '@')
                    .count()
                    < 4
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: RefCell<Grid<char>> = RefCell::new(Grid::parse_lines(input));

    let mut total = 0;
    let len = grid.borrow().data.len();

    loop {
        let removed = (0..len)
            .map(|i| (i, grid.borrow().data[i]))
            .filter(|(_, c)| *c == '@')
            .filter(|(i, _)| {
                if grid
                    .borrow()
                    .neighbors_with_diagonals(*i)
                    .filter(|(_, i)| grid.borrow().data[*i] == '@')
                    .count()
                    < 4
                {
                    grid.borrow_mut().data[*i] = '.';
                    true
                } else {
                    false
                }
            })
            .count();

        if removed == 0 {
            break;
        }

        total += removed;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
