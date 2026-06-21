fn main() {
    stack_fn();
    heap_fn();
    update_string();
}

fn stack_fn() {
    let a = 4;
    let b = 4;
    let c = a + b;
    print!("{} + {} = {}\n", a, b, c);
}

fn heap_fn() {
    let a = String::from("Hello");
    let b = String::from("World");
    let c = format!("{} {}\n", a, b);
    print!("{}", c);
}

fn update_string() {
    let mut a = String::from("Hello");
    print!("{}\n", a);
    a.push_str(", World!");
    print!("{}", a);
}