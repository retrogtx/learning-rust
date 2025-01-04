use std::fs::read_to_string;
fn main() {
    let _file_path = read_from_file(String::from("file.txt"));
}

fn read_from_file(file_path: String) -> Result<String, String> {
    let text = read_to_string(file_path);
    match text{
        Ok(data) => Ok(data),
        Err(e) => Err(e.to_string())
    }
}