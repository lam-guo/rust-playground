fn main() {
    let position = 1;
    let json = format!(r#"{{"position":{}}}"#, position);
    println!("{}", json);
}
