use dbent::prelude::*;

#[derive(Entity)]
struct Test {
    data: String,
    id: Key<Int>,
}

fn main() {}
