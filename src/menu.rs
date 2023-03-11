use std::io;
pub enum MenuOptions {
    ADD,
    SEARCH,
    LOAD,
    SAVE,
    QUIT,
}

pub fn menu() -> MenuOptions {
    let stdin = io::stdin();

    //loop until valid choice
    let choice: u8 = loop {
        // clear screen

        print!("{}[2J", 27 as char);

        println!(" 1. Add a person");
        println!(" 2. Search for birthdays");
        println!(" 3. Load");
        println!(" 4. Save");
        println!(" 5. Quit");

        println!("Enter your choice: ");

        let mut input = String::new();

        match stdin.read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                continue;
            }
        };

        match input.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 5 {
                    continue;
                }
                break num;
            }
            Err(_) => {
                println!("Please enter a valid option");
                continue;
            }
        };
    };

    match choice {
        1 => MenuOptions::ADD,
        2 => MenuOptions::SEARCH,
        3 => MenuOptions::LOAD,
        4 => MenuOptions::SAVE,
        5 => MenuOptions::QUIT,
        _ => todo!(),
    }
}

//write me a test
#[cfg(test)]
mod tests {
    use std::io::Stdin;

    use super::*;

    #[test]
    fn test_menu() {
        {}
    }
}
