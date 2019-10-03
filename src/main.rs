pub mod ty;
pub mod id;
pub mod syntax_impl;
pub mod syntax_creator_impl;

#[macro_use]
extern crate nom;
#[macro_use]
mod util;
pub mod token_parser;
mod parser;

fn main() {
    println!("Hello, world!");
}
