pub fn is_even(n: u32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_even() {
        assert!(is_even(4), "4 should be even");
    }
}