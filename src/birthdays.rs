use std::{default::Default, io::Write};

const MAX_PERSONS: usize = 10;

/**
 * Entry struct to keep each friend name and birthday data
 *
 */
#[derive(Clone)]
pub struct Entry {
    name: String,
    birth_day: u32,
    birth_month: u32,
}

impl Default for Entry {
    fn default() -> Self {
        Entry {
            name: String::new(),
            birth_day: 0,
            birth_month: 0,
        }
    }
}

/**
 * A store to keep the birthdays
 */
pub struct Birthdays {
    pub friends: Vec<Entry>,
}

impl Birthdays {
    pub fn new() -> Self {
        let friends: Vec<Entry> = Vec::with_capacity(MAX_PERSONS as usize);
        Birthdays { friends }
    }

    /**
     * Add a new entry to the store
     */
    pub fn add(&mut self, entry: Entry) -> Result<(), String> {
        if self.friends.len() >= MAX_PERSONS {
            return Err("There is no enough space to save the entry".to_owned());
        } else {
            self.friends.push(entry);
            return Ok(());
        };
    }

    fn size(self) -> usize {
        self.friends.len()
    }

    /**
     * Search and prints the birthdays of a specific day and month to the writer
     */
    fn print_birthdays_to_writer(
        &self,
        birth_day: u32,
        birth_month: u32,
        mut writer: impl std::io::Write,
    ) {
        for friend in &self.friends {
            if friend.birth_day == birth_day && friend.birth_month == birth_month {
                writeln!(
                    writer,
                    "{}: {}/{}",
                    friend.name, friend.birth_day, friend.birth_month,
                )
                .unwrap();
            }
        }
    }

    /**
     * Search and prints the birthdays of a specific day and month to the stadard output
     */
    pub fn print_birthdays(&self, birth_day: u32, birth_month: u32) {
        self.print_birthdays_to_writer(birth_day, birth_month, std::io::stdout());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};

    #[test]
    fn test_new() {
        let birthdays = Birthdays::new();
        assert_eq!(birthdays.friends.capacity(), MAX_PERSONS);
    }

    #[test]
    fn test_add_birthday() {
        let mut store = Birthdays::new();

        let b = Entry {
            name: String::from("John"),
            birth_day: 20,
            birth_month: 2,
        };

        store.add(b).unwrap();

        assert_eq!(store.size(), 1);
    }

    #[test]
    fn test_cannot_add_more_than_defined_size() {
        let mut store = Birthdays::new();

        for _i in 0..10 {
            store.add(Entry::default()).unwrap();
        }

        let res = store.add(Entry::default());

        assert!(res.is_err());
    }

    #[test]
    fn test_size() {
        let mut store = Birthdays::new();

        for _i in 0..10 {
            store.add(Entry::default()).unwrap();
        }

        assert_eq!(store.size(), 10);
    }

    #[test]
    fn test_print_birthdays() {
        let mut store = Birthdays::new();

        let entry = Entry {
            name: "john".to_owned(),
            birth_day: 01,
            birth_month: 02,
        };

        let entry2 = Entry {
            name: "Bill".to_owned(),
            birth_day: 3,
            birth_month: 10,
        };

        store.add(entry.clone()).unwrap();
        store.add(entry2.clone()).unwrap();

        let mut buffer = Vec::new();

        store.print_birthdays_to_writer(entry.birth_day, entry.birth_month, &mut buffer);

        let required_output = format!(
            "{}: {}/{}\n",
            entry.name, entry.birth_day, entry.birth_month
        );
        assert_eq!(buffer, required_output.as_bytes());
    }
}
