use std::fmt::Display;

pub mod utils;

pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;
pub mod d06;
pub mod d07;
pub mod d08;
pub mod d09;
pub mod d10;
pub mod d11;
pub mod d12;
pub mod d13;
pub mod d14;
pub mod d15;
pub mod d16;
pub mod d17;
pub mod d18;
pub mod d19;
pub mod d20;
pub mod d21;
pub mod d22;
pub mod d23;
pub mod d24;
pub mod d25;

pub trait Solution {
    type Part1: Display;
    type Part2: Display;

    fn solve_p1(&self, input: &str) -> Self::Part1;
    fn solve_p2(&self, input: &str) -> Self::Part2;
}
