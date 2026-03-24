// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert() {
        assert!(is_even(2), "2 should be even");
        assert!(is_even(0), "0 should be even");
    }
}
