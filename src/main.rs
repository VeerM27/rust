use std::fs;

enum MyResult<A, B> {
    Ok(A),
    Err(B),
}

fn main() {
    let res = fs::read_to_string("example.txt");

    match res {
        Ok(content) => {
            println!("File content: {}", content);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn read_from_file(file_content: String) -> MyResult<String, String> {
    let res = fs::read_to_string("example.txt");

    if let Ok(content) = res {
        return MyResult::Ok(content);
    } else {
        return MyResult::Err("Failed to read file".to_string());
    }
}