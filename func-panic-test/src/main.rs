pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_safe_divide() -> Result<(), String> {
        assert!(safe_divide(10, 0).is_none());
        Ok(())
    }
}