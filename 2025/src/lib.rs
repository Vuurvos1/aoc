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
// pub mod d12;

pub trait Solution {
    type Part1: Display;
    type Part2: Display;

    fn solve_p1(&self, input: &str) -> Self::Part1;
    fn solve_p2(&self, input: &str) -> Self::Part2;
}
