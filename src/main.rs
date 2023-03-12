//allow unused code
#![allow(dead_code)]

mod menu;
mod validator;
use menu::{menu, MenuOptions};

mod birthdays;

fn main() {
    loop {
        match menu() {
            MenuOptions::ADD => todo!(),
            MenuOptions::SEARCH => todo!(),
            MenuOptions::LOAD => todo!(),
            MenuOptions::SAVE => todo!(),
            MenuOptions::QUIT => todo!(),
        };
    }
}
