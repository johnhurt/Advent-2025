use advent_of_code::Grid;
use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid<u8> = Grid::parse_lines(input);

    Some(
        grid.data
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, c)| *c == b'@')
            .filter(|(i, _)| {
                grid.neighbors_with_diagonals(*i)
                    .filter(|(_, i)| grid.data[*i] == b'@')
                    .count()
                    < 4
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid: Grid<u8> = Grid::parse_lines(input);
    let mut last_updated = vec![0; grid.data.len()];

    let mut total = 0;

    let mut to_search = grid
        .data
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(i, c)| (c == b'@').then_some(i))
        .collect_vec();

    let mut updated = Vec::with_capacity(to_search.len());
    let mut generation = 1;

    loop {
        updated.extend(to_search.drain(..).filter(|i| {
            grid.neighbors_with_diagonals(*i)
                .filter(|(_, i)| grid.data[*i] == b'@')
                .count()
                < 4
        }));

        if updated.is_empty() {
            break;
        }

        updated.iter().copied().for_each(|i| grid.data[i] = b'.');

        total += updated.len();

        to_search.extend(
            updated
                .drain(..)
                .flat_map(|i| {
                    grid.neighbors_with_diagonals(i).filter_map(|(_, j)| {
                        (grid.data[j] == b'@').then_some(j)
                    })
                })
                .filter(|i| {
                    // This is just a faster way to do `.unique()`
                    if last_updated[*i] == generation {
                        false
                    } else {
                        last_updated[*i] = generation;
                        true
                    }
                }),
        );

        generation += 1;
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
