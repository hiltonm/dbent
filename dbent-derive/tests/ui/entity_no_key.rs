use dbent_derive::Entity;

#[derive(Entity)]
struct Test {
    id: Option<usize>,
    data: String,
}

fn main() {}
