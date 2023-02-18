
use dbent::prelude::*;

#[derive(Label)]
struct Test(Key<Int>, #[label] String);

fn main() {}
