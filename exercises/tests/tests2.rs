// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

use std::fmt::Display;

#[derive(Debug)]
struct Equalizer {
    id: String,
}

impl Equalizer {
    fn new(id: String) -> Self {
        Equalizer { id }
    }
}

impl Display for Equalizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}", self.id)
    }
}

impl PartialEq for Equalizer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        let equalizer_one = Equalizer::new("same".to_string());
        let equalizer_two = Equalizer::new("same".to_string());

        print!("\nLeft({}), Right({})\n\n", equalizer_one, equalizer_two);

        assert_eq!(equalizer_one, equalizer_two)
    }
}
