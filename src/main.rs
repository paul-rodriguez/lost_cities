#![allow(non_snake_case)]

#[macro_use]
extern crate maplit;

use std::vec;
mod error;
mod model;
mod player;

use model::{Board, Card, Side};

fn main() {
    println!("Hello, world!");

    let _s = model::Card::set();
    let _a: vec::Vec<i32> = _s
        .into_iter()
        .map(|card| {
            println!("{}", card);
            1
        })
        .collect();
    let mut b = Board::new().scoreCard(Side::Up, Card::fromId(7)).unwrap();
    println!("{}", b);
    b = b.scoreCard(Side::Up, Card::fromId(2)).unwrap();
    println!("{}", b);
}
