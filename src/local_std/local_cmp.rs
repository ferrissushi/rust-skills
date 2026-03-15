use std::cmp::{self, Ordering};

pub fn launch_cmp_module_test() -> std::io::Result<()> {
    let a = cmp::max(12, 23);
    println!("{a}");

    let firstname = "Nicky".to_string();
    let lastname = "Hariniaina".to_string();

    let b = cmp::max_by(&firstname, &lastname, |firstname, lastname| {
        firstname.len().cmp(&lastname.len())
    });

    println!("{b}");

    let c = cmp::max_by_key(&firstname, &lastname, |name| name.len());

    println!("{c}");

    // ---------------------------

    Ok(())
}

enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

struct Book {
    isbn: i32,
    format: BookFormat,
}

impl Eq for Book {}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }

    fn ne(&self, other: &Self) -> bool {
        self.isbn != other.isbn
    }
}

impl PartialOrd for Book {
    fn lt(&self, other: &Self) -> bool {
        self.isbn < other.isbn
    }

    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.isbn > other.isbn {
            Some(Ordering::Greater)
        } else if self.isbn < other.isbn {
            Some(Ordering::Less)
        } else if self.isbn == other.isbn {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}
