use dbent::prelude::*;

#[derive(Label)]
enum Test {
    Id(Key<Int>),
    #[label] Data(String),
}

fn main() {}
