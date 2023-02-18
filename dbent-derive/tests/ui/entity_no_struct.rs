use dbent::prelude::*;

#[derive(Entity)]
enum Test {
    Id(Key<Int>),
    Data(String),
}

fn main() {}
