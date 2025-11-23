pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_greet() {
        let greeting = greet("Alice");
        assert!(greeting.contains("Alice"));
    }
}