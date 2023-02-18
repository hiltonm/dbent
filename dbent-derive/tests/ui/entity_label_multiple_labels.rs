use dbent::prelude::*;

#[derive(Label)]
struct Test {
    id: Key<Int>,
    #[label] data1: String,
    #[label] data2: String,
}

fn main() {}
