use std::io;
#[derive(PartialEq, Debug)]
pub enum MenuOptions {
    ADD,
    SEARCH,
    LOAD,
    SAVE,
    QUIT,
}

fn parse_menu_input(input: &str) -> Result<MenuOptions, &'static str> {
    match input.trim().parse() {
        Ok(num) => match num {
            1 => Ok(MenuOptions::ADD),
            2 => Ok(MenuOptions::SEARCH),
            3 => Ok(MenuOptions::LOAD),
            4 => Ok(MenuOptions::SAVE),
            5 => Ok(MenuOptions::QUIT),
            _ => Err("Invalid option, please enter a number between 1 and 5."),
        },
        Err(_) => Err("Invalid input, please enter a valid number"),
    }
}

pub fn menu() -> MenuOptions {
    let stdin = io::stdin();

    //loop until valid choice
    loop {
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
            Err(_) => continue,
        };

        match parse_menu_input(&input) {
            Ok(menu_option) => return menu_option,
            Err(err_msg) => {
                println!("{}", err_msg);
                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_menu_option() {
        assert_eq!(parse_menu_input("1\n").unwrap(), MenuOptions::ADD);
        assert_eq!(parse_menu_input("2\n").unwrap(), MenuOptions::SEARCH);
        assert!(parse_menu_input("7\n").is_err());
        assert_eq!(parse_menu_input("4\n").unwrap(), MenuOptions::SAVE);
        assert_eq!(parse_menu_input("5\n").unwrap(), MenuOptions::QUIT);
        assert!(parse_menu_input("\n").is_err());
    }
}
