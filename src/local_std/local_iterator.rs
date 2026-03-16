pub fn iterator_module_main() -> std::io::Result<()> {
    let numbers = vec![1, 3, 4, 5, 7, 8];
    let mut iter_numbers = numbers.iter().take(3);
    loop {
        let current_number = iter_numbers.next();
        match current_number {
            Some(number) => {
                println!("{number}");
            }
            None => break,
        };
    }

    println!("------");

    let mut counter: Counter = Default::default();

    loop {
        match counter.next() {
            Some(number) => {
                println!("{number}");
            }
            None => break,
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Counter {
    count: usize,
    max_count: usize,
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            count: 0,
            max_count: 10,
        }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.max_count {
            None
        } else {
            self.count += 1;
            Some(self.count)
        }
    }
}
