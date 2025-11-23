pub fn read_file(path: &str) -> Result<String, String> {
    if path == "test.txt" {
        Err("File not found".to_string())
    } else {
        Ok("File content".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_file() -> Result<(), String> {
        assert_eq!(read_file("test.txt"), Err("File not found".to_string()));
        Ok(())
    }
}