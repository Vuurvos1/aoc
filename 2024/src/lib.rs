use std::fmt::Display;

pub mod d01;
pub mod d23;
pub mod d25;

pub trait Solution {
    type Part1: Display;
    type Part2: Display;

    fn day(&self) -> u32;

    fn solve_p1(&self, input: &str) -> Self::Part1;
    fn solve_p2(&self, input: &str) -> Self::Part2;
}
