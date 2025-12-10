use std::{
    collections::{HashMap, HashSet},
    mem::swap,
};

use advent_of_code::ws;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{u16, u64},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::delimited,
    IResult, Parser,
};
use num_rational::Rational64;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
advent_of_code::solution!(10);

#[derive(Debug)]
struct Setup {
    button_state: Vec<bool>,
    buttons: Vec<Vec<u64>>,
    joltages: Vec<u16>,
}

fn parse_setups(input: &str) -> IResult<&'_ str, Vec<Setup>> {
    separated_list1(
        tag("\n"),
        map(
            (
                delimited(
                    tag("["),
                    many1(map(alt((tag("."), tag("#"))), |c| c == "#")),
                    tag("]"),
                ),
                many1(ws(delimited(
                    tag("("),
                    separated_list1(tag(","), u64),
                    tag(")"),
                ))),
                delimited(tag("{"), separated_list1(tag(","), u16), tag("}")),
            ),
            |(button_state, buttons, joltages)| Setup {
                button_state,
                buttons,
                joltages,
            },
        ),
    )
    .parse(input)
}

#[derive(Debug, Clone)]
struct Sum(Rational64, HashMap<u8, Rational64>);

impl Sum {
    fn reduce(&mut self, system: &SoE) {
        for (v, v_eq) in &system.solved_vars {
            self.reduce_one(*v, v_eq);
        }
    }

    fn reduce_one(&mut self, v: u8, v_eq: &Sum) {
        if let Some(coef) = self.1.remove(&v) {
            self.add_terms(v_eq, coef);
        }
    }

    fn add_terms(&mut self, new_terms: &Sum, coef: Rational64) {
        self.0 += new_terms.0 * coef;

        new_terms.1.iter().for_each(|(v, v_coef)| {
            self.1
                .entry(*v)
                .and_modify(|old_coef| *old_coef += *v_coef * coef)
                .or_insert(*v_coef * coef);
        });

        self.1.retain(|_, v| *v != Rational64::ZERO);
    }

    fn solve_for_one(&self, active: &HashSet<u8>) -> Option<(u8, Sum)> {
        let mut result = self.clone();
        let v = active
            .iter()
            .copied()
            .find(|i| self.1.contains_key(i))
            .or_else(|| self.1.keys().next().copied())?;

        let v_coef = result.1.remove(&v).unwrap() * Rational64::new(-1, 1);

        result.0 /= v_coef;
        result.1.values_mut().for_each(|v| *v /= v_coef);

        Some((v, result))
    }

    fn eval(&self, clicks: &[u16]) -> Option<u16> {
        let res = self.0
            + self.1.iter().fold(Rational64::ZERO, |acc, (v, v_coef)| {
                acc + Rational64::new(clicks[*v as usize] as i64, 1) * v_coef
            });

        if res >= Rational64::ZERO && *res.denom() == 1 {
            Some(*res.numer() as u16)
        } else {
            None
        }
    }
}

/// Very simple linear system of equations
#[derive(Debug, Default)]
struct SoE {
    active_vars: HashSet<u8>,
    solved_vars: HashMap<u8, Sum>,

    // Sums of terms that all equal zero
    equations: Vec<Sum>,
}

impl SoE {
    fn add_equation(&mut self, mut eq: Sum) {
        eq.reduce(self);

        let Some((v, v_eq)) = eq.solve_for_one(&self.active_vars) else {
            return;
        };

        if self.active_vars.remove(&v) {
            self.solved_vars
                .values_mut()
                .for_each(|exp| exp.reduce_one(v, &v_eq));
        }

        self.active_vars.extend(v_eq.1.keys().copied());
        self.solved_vars.insert(v, v_eq);
        self.equations.push(eq);
    }

    fn eval(&self, clicks: &mut [u16]) -> Option<usize> {
        for (v, eq) in &self.solved_vars {
            clicks[*v as usize] = eq.eval(clicks)?;
        }

        Some(clicks.iter().map(|c| *c as usize).sum())
    }
}

impl Setup {
    fn target_bits(&self) -> u64 {
        self.button_state
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(i, b)| b.then_some(i))
            .fold(0, |acc, i| acc + (1 << i))
    }

    fn button_bits(&self) -> Vec<u64> {
        self.buttons
            .iter()
            .map(|button| {
                button.iter().copied().fold(0, |acc, i| acc + (1 << i))
            })
            .collect()
    }

    fn solve_part_1(&self) -> usize {
        let mut combos = vec![(0, 0)];
        let mut next_combos = vec![];
        let buttons = self.button_bits();
        let target = self.target_bits();

        // Iterate through all subsets by size. No button can be pressed more
        // than once because xor is commutative and its own inverse (Pressing a
        // button twice is the same as not pushing it)
        for result in 1..=self.buttons.len() {
            for (i, button) in buttons.iter().copied().enumerate() {
                let button_index_mask = 1 << i;
                for (combo, set) in combos.iter().copied() {
                    let new_set = set | button_index_mask;
                    if new_set == set {
                        continue;
                    }

                    let new_combo = combo ^ button;

                    if new_combo == target {
                        return result;
                    }

                    next_combos.push((new_combo, new_set));
                }
            }

            swap(&mut combos, &mut next_combos);
        }

        unreachable!()
    }

    fn as_equations(&self) -> Vec<Sum> {
        let button_bits = self.button_bits();
        self.joltages
            .iter()
            .copied()
            .enumerate()
            .map(|(i, j)| {
                let reg_mask = 1 << i;
                Sum(
                    -Rational64::new(j as i64, 1),
                    button_bits
                        .iter()
                        .copied()
                        .enumerate()
                        .filter_map(|(i, b)| {
                            (b & reg_mask > 0)
                                .then_some((i as u8, Rational64::ONE))
                        })
                        .collect(),
                )
            })
            .collect()
    }

    fn max_single_clicks(&self) -> usize {
        self.joltages
            .iter()
            .map(|j| *j as usize)
            .max()
            .unwrap_or_default()
    }

    fn solve_part_2(self) -> usize {
        let mut soe = SoE::default();
        self.as_equations()
            .into_iter()
            .for_each(|e| soe.add_equation(e));

        // working space for evaluation
        let mut clicks = vec![0; self.buttons.len()];

        // No active variables means this is fully defined, so we can evaluate
        // directly with no searching
        if soe.active_vars.is_empty() {
            return soe.eval(&mut clicks).unwrap();
        }

        // the maximum times we will click a single button is the maximum
        // joltage
        let max_clicks = self.max_single_clicks() as u64;

        let independent_vars = soe.active_vars.iter().copied().collect_vec();

        // There shouldn't be many independent variables left, so brute force
        // all possible combinations to find the answer
        (0..(max_clicks.pow(soe.active_vars.len() as u32)))
            .filter_map(|mut i| {
                clicks.iter_mut().for_each(|c| *c = 0);
                for v in &independent_vars {
                    clicks[*v as usize] = (i % max_clicks) as u16;
                    i /= max_clicks;
                }

                soe.eval(&mut clicks)
            })
            .min()
            .unwrap()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let setups = parse_setups(input).unwrap().1;

    Some(setups.into_iter().map(|s| s.solve_part_1()).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let setups = parse_setups(input).unwrap().1;

    Some(setups.into_par_iter().map(|s| s.solve_part_2()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
