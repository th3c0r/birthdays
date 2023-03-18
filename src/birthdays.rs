use crate::validator::validate_date_for_birthday;
use std::default::Default;

const MAX_PERSONS: usize = 10;

/**
 * Entry struct to keep each friend name and birthday data
 *
 */
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Entry<'a> {
    name: &'a str,
    birth_day: u8,
    birth_month: u8,
}

#[derive(Clone, Debug)]
pub struct Birthdays<'a> {
    pub friends: Vec<Entry<'a>>,
}

impl<'a> Birthdays<'a> {
    pub fn new() -> Self {
        let friends: Vec<Entry> = Vec::with_capacity(MAX_PERSONS as usize);
        Birthdays { friends }
    }

    /**
     * Add a new entry to the store
     */
    pub fn add(&mut self, entry: Entry<'a>) -> Result<(), String> {
        if self.friends.len() >= MAX_PERSONS {
            return Err("There is no enough space to save the entry".to_owned());
        }

        validate_date_for_birthday(entry.birth_day, entry.birth_month)?;

        self.friends.push(entry);

        Ok(())
    }

    fn size(self) -> usize {
        self.friends.len()
    }

    /**
     * Search and prints the birthdays of a specific day and month to the writer
     */
    fn print_birthdays_to_writer(
        &self,
        birth_day: u8,
        birth_month: u8,
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
    pub fn print_birthdays(&self, birth_day: u8, birth_month: u8) {
        self.print_birthdays_to_writer(birth_day, birth_month, std::io::stdout());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let birthdays = Birthdays::new();
        assert_eq!(birthdays.friends.capacity(), MAX_PERSONS);
    }

    #[test]
    fn test_add_birthday() {
        let mut store = Birthdays::new();

        let b = Entry {
            name: "John",
            birth_day: 20,
            birth_month: 2,
        };

        store.add(b).unwrap();

        assert_eq!(store.size(), 1);
    }

    #[test]
    fn test_cannot_add_more_than_defined_size() {
        let mut store = Birthdays::new();

        let entry = Entry {
            name: "demo",
            birth_day: 1,
            birth_month: 1,
        };

        for _i in 0..10 {
            store.add(entry.clone()).unwrap();
        }

        let res = store.add(Entry::default());

        assert!(res.is_err());
    }

    #[test]
    fn test_size() {
        let mut store = Birthdays::new();
        let entry = Entry {
            name: "demo",
            birth_day: 1,
            birth_month: 1,
        };

        for _i in 0..10 {
            store.add(entry.clone()).unwrap();
        }

        assert_eq!(store.size(), 10);
    }

    #[test]
    fn test_print_birthdays() {
        let mut store = Birthdays::new();

        let entry = Entry {
            name: "john",
            birth_day: 01,
            birth_month: 02,
        };

        let entry2 = Entry {
            name: "Bill",
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

    #[test]
    fn test_that_cannot_create_birthday_with_invalid_month() {
        struct BirthdayTest<'a> {
            entry: Entry<'a>,
            status: bool,
        }

        let mut tests = Vec::<BirthdayTest>::new();

        tests.push(BirthdayTest {
            entry: Entry {
                name: "Test 01",
                birth_day: 14,
                birth_month: 9,
            },
            status: true,
        });

        tests.push(BirthdayTest {
            entry: Entry {
                name: "Test 02",
                birth_day: 34,
                birth_month: 3,
            },
            status: false,
        });

        tests.push(BirthdayTest {
            entry: Entry {
                name: "Test 03",
                birth_month: 3,
                birth_day: 29,
            },
            status: true,
        });

        tests.push(BirthdayTest {
            entry: Entry {
                name: "Test 04",
                birth_day: 30,
                birth_month: 02,
            },
            status: false,
        });

        let mut store = Birthdays::new();

        for test in tests {
            let res = match store.add(test.entry.clone()) {
                Ok(_) => true,
                Err(_) => false,
            };

            assert_eq!(
                res, test.status,
                "We have proglem to the {}/{}",
                test.entry.birth_day, test.entry.birth_month
            );
        }
    }
}
