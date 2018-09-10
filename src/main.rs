fn main() {
    let name = String::from("Ilya");

    println!("Charachter at 4 position {:?}",
        match name.chars().nth(4) {
            Some(c) => c.to_string(),
            None => "No character found".to_string(),
        }
    );

    let value = 2;
    match value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => (),
    };
}
