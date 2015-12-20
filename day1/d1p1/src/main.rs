use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(error) => { panic!(error) }
        _ => {}
    };

    let mut position = 0;
    for c in input.chars() {
        match c {
            '(' => { position += 1; }
            ')' => { position -= 1; }
            _ => {}
        }
    }

    println!("Final position is {}.", position);

    position = 0;
    for (i,c) in input.chars().enumerate() {
        match c {
            '(' => { position += 1; }
            ')' => { position -= 1; }
            _ => {}
        }
        if position == -1 {
            println!("Entered basement at {}.", i + 1);
            break;
        }
    }
}
