fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn main() {
    let my_string = String::from("raman");
    let res = find_first_a(my_string);
    
    match res {
        Some(index) => println!("The first occurrence of 'a' is at index: {}", index),
        None => println!("The character 'a' was not found in the string."),
    }
}