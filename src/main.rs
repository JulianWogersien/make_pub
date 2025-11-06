use make_pub::make_pub;

// Testable via `cargo expand --bin make_pub`
#[make_pub]
#[derive(Debug)]
struct Conf {
    field1: i32,
    field2: f32,
}

fn main() {
    let test = Conf{field1: 42, field2: 0.1};

    println!("Conf: {:?}", test);
}