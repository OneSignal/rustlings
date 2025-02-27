// options1.rs
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a hint.

// you can modify anything EXCEPT for this function's signature
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
// TODO: Return an Option!
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22
    if time_of_day < 22 {
        Some(5)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), None);
        assert_eq!(maybe_icecream(22), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the Option?
        let icecreams = maybe_icecream(12);
        match icecreams {
            Some(5) => {}
            None => panic!("There is no icecream!"),
            Some(icecreams) => panic!("Here are the icecreams {}", icecreams),
        }
        // assert_eq!(icecreams, 5);
    }
}
