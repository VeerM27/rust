fn main() {
let mut _a: i8 = 5;
let mut _b: i8 = 10;
print!("{} + {} = {}", _a, _b, do_sum(_a, _b));
_a = _a + do_sum(_a, _b);
print!("\nNew a = {}", _a);
}

fn do_sum(a: i8, b: i8) -> i8 {
    a + b
}